use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

use korlang_compiler::codegen::Codegen;
use korlang_compiler::diag::Diagnostic;
use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;
use korlang_compiler::sema::Sema;
use korlang_compiler::linker::{build_link_command, LinkerConfig, LtoMode};
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, FileType};

fn main() {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    match cmd.as_str() {
        "build" => build(args.collect(), false),
        "run" => build(args.collect(), true),
        "new" => new_project(args.collect()),
        "test" => run_tests(),
        "doc" => generate_docs(),
        "bootstrap" => bootstrap(),
        "repl" => repl(),
        _ => {
            eprintln!("Usage: korlang <build|run|new|test|doc|bootstrap|repl> <file.kor> [-o out] [--static] [--lto|--thinlto] [--pgo-generate] [--pgo-use file]");
            eprintln!("       korlang build --native-selfhost");
            eprintln!("       korlang run file.kor -- arg1 arg2");
            std::process::exit(1);
        }
    }
}

fn build(args: Vec<String>, run: bool) {
    if args.iter().any(|a| a == "--native-selfhost") {
        build_native_selfhosted();
        return;
    }

    if args.is_empty() {
        eprintln!("missing input file");
        std::process::exit(1);
    }
    let (build_args, run_args) = split_run_args(&args);
    let input = PathBuf::from(&build_args[0]);
    let mut output = PathBuf::from("a.out");
    let mut static_link = false;
    let mut lto = None;
    let mut pgo_generate = false;
    let mut pgo_use: Option<PathBuf> = None;

    let mut i = 1;
    while i < build_args.len() {
        if build_args[i] == "-o" && i + 1 < build_args.len() {
            output = PathBuf::from(&build_args[i + 1]);
            i += 2;
        } else if build_args[i] == "--static" {
            static_link = true;
            i += 1;
        } else if build_args[i] == "--lto" {
            lto = Some(LtoMode::Full);
            i += 1;
        } else if build_args[i] == "--thinlto" {
            lto = Some(LtoMode::Thin);
            i += 1;
        } else if build_args[i] == "--pgo-generate" {
            pgo_generate = true;
            i += 1;
        } else if build_args[i] == "--pgo-use" && i + 1 < build_args.len() {
            pgo_use = Some(PathBuf::from(&build_args[i + 1]));
            i += 2;
        } else {
            i += 1;
        }
    }

    let src = match fs::read_to_string(&input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read source {}: {}", input.display(), e);
            std::process::exit(1);
        }
    };
    let target_dir = PathBuf::from(".korlang/target");
    let _ = fs::create_dir_all(&target_dir);
    let out_ll = target_dir.join(output.with_extension("ll").file_name().unwrap());
    let out_obj = target_dir.join(output.with_extension("o").file_name().unwrap());
    let tokens = match Lexer::new(&src).tokenize() {
        Ok(t) => t,
        Err(diags) => {
            print_diags("lex", &input, &diags);
            std::process::exit(1);
        }
    };
    let program = match Parser::new(tokens).parse_program() {
        Ok(p) => p,
        Err(diags) => {
            print_diags("parse", &input, &diags);
            std::process::exit(1);
        }
    };
    if let Err(diags) = Sema::new().check_program(&program) {
        print_diags("sema", &input, &diags);
        std::process::exit(1);
    }

    let context = Context::create();
    let codegen = Codegen::new(&context, "main");
    let module = match codegen.emit_program(&program) {
        Ok(m) => m,
        Err(diags) => {
            print_diags("codegen", &input, &diags);
            std::process::exit(1);
        }
    };
    if let Err(e) = module.print_to_file(&out_ll) {
        eprintln!("failed to write IR {}: {}", out_ll.display(), e);
        std::process::exit(1);
    }

    let runtime_lib = locate_runtime().unwrap_or_else(|| PathBuf::from("../../runtime/target/debug/libkorlang_rt.a"));
    let mut extra_args = Vec::new();
    if static_link {
        extra_args.push("-static".to_string());
    }
    if cfg!(target_os = "linux") {
        extra_args.push("-no-pie".to_string());
    }

    // Compile IR to object using LLVM target machine (no external clang).
    if !compile_ir_to_obj(&module, &out_obj) {
        eprintln!("Failed to compile LLVM IR to object.");
        return;
    }

    let link = build_link_command(&out_obj, &LinkerConfig {
        output: output.clone(),
        runtime_lib,
        extra_args,
        lto,
        pgo_generate,
        pgo_use,
    });

    if run {
        println!("Compiling: {}", input.display());
    } else {
        println!("LLVM IR written to: {}", out_ll.display());
        println!("Link with: {}", link.join(" "));
    }

    let status = Command::new(&link[0])
        .args(&link[1..])
        .status();

    match status {
        Ok(s) if s.success() => {
            if run {
                let run_target = resolve_run_target(&output);
                let mut cmd = Command::new(&run_target);
                if !run_args.is_empty() {
                    cmd.args(&run_args);
                }
                match cmd.status() {
                    Ok(run_status) if run_status.success() => {}
                    Ok(run_status) => {
                        let code = run_status.code().unwrap_or(1);
                        std::process::exit(code);
                    }
                    Err(e) => {
                        eprintln!("failed to execute {}: {}", run_target.display(), e);
                        std::process::exit(1);
                    }
                }
            }
        }
        _ => {
            eprintln!("link failed; binary not executed");
            std::process::exit(1);
        }
    }
}

fn resolve_run_target(output: &PathBuf) -> PathBuf {
    if output.is_absolute() {
        return output.clone();
    }
    // `a.out` must be executed as `./a.out` on Unix shells.
    let parent_empty = output
        .parent()
        .map(|p| p.as_os_str().is_empty())
        .unwrap_or(true);
    if parent_empty {
        return PathBuf::from(".").join(output);
    }
    output.clone()
}

fn split_run_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    if let Some(idx) = args.iter().position(|a| a == "--") {
        (args[..idx].to_vec(), args[idx + 1..].to_vec())
    } else {
        (args.to_vec(), Vec::new())
    }
}

fn print_diags(stage: &str, file: &PathBuf, diags: &[Diagnostic]) {
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";
    eprintln!("{red}error{reset}: {stage} failed for {}", file.display());
    for d in diags {
        eprintln!(
            "{cyan}--> {reset}{}:{}:{}",
            file.display(),
            d.span.start.line,
            d.span.start.column
        );
        eprintln!("  {yellow}|{reset} {}", d.message);
    }
}

fn compile_ir_to_obj(module: &inkwell::module::Module, obj: &PathBuf) -> bool {
    Target::initialize_all(&InitializationConfig::default());
    let triple = TargetMachine::get_default_triple();
    module.set_triple(&triple);
    let target = match Target::from_triple(&triple) {
        Ok(t) => t,
        Err(_) => return false,
    };
    let machine = match target.create_target_machine(
        &triple,
        "generic",
        "",
        inkwell::OptimizationLevel::Default,
        inkwell::targets::RelocMode::Default,
        inkwell::targets::CodeModel::Default,
    ) {
        Some(m) => m,
        None => return false,
    };
    machine.write_to_file(module, FileType::Object, obj).is_ok()
}

fn locate_runtime() -> Option<PathBuf> {
    if let Ok(home) = env::var("KORLANG_HOME") {
        let p = PathBuf::from(home).join("lib").join("libkorlang_rt.a");
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("../lib/libkorlang_rt.a");
            if p.exists() {
                return Some(p);
            }
        }
    }
    None
}

fn new_project(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("korlang new <name> [--lib|--ui|--cloud]");
        std::process::exit(1);
    }
    let name = &args[0];
    let root = PathBuf::from(name);
    let _ = fs::create_dir_all(root.join("src"));
    let config = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
        name
    );
    let _ = fs::write(root.join("Korlang.config"), config);
    let main = "fun main() -> Int {\n  0\n}\n";
    let _ = fs::write(root.join("src/main.kor"), main);
    println!("Created {}", root.display());
}

fn run_tests() {
    println!("Running Korlang tests (placeholder)...");
}

fn generate_docs() {
    println!("Generating docs (placeholder)...");
}

fn repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut history: Vec<String> = Vec::new();
    let mut state_lines: Vec<String> = Vec::new();
    let hist_path = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".korlang/repl_history");
    if let Ok(s) = fs::read_to_string(&hist_path) {
        history.extend(s.lines().map(|s| s.to_string()));
    }
    println!("Korlang REPL (type :help, :quit)");
    loop {
        let _ = write!(stdout, "korlang> ");
        let _ = stdout.flush();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).ok().unwrap_or(0) == 0 {
            break;
        }
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        history.push(line.clone());
        if line == ":quit" || line == ":q" {
            break;
        }
        if line == ":help" {
            println!(":quit             exit");
            println!(":history          show command history");
            println!(":complete <pref>  show keyword completions");
            println!("Enter `let` bindings or expressions.");
            continue;
        }
        if line == ":history" {
            for (i, h) in history.iter().enumerate() {
                println!("{:>4}: {}", i + 1, h);
            }
            continue;
        }
        if let Some(pref) = line.strip_prefix(":complete ") {
            for kw in repl_keywords().into_iter().filter(|k| k.starts_with(pref)) {
                println!("{kw}");
            }
            continue;
        }

        let is_binding = line.starts_with("let ");
        let source = if is_binding {
            let mut body = state_lines.join("\n");
            if !body.is_empty() {
                body.push('\n');
            }
            body.push_str(&line);
            body.push_str(";\n0");
            format!("fun main() -> Int {{\n{}\n}}\n", body)
        } else {
            let mut body = state_lines.join("\n");
            if !body.is_empty() {
                body.push('\n');
            }
            body.push_str(&format!("let __repl_value = {};\n0", line));
            format!("fun main() -> Int {{\n{}\n}}\n", body)
        };

        let tmp = PathBuf::from(".korlang/repl_main.kor");
        let _ = fs::create_dir_all(".korlang");
        if let Err(e) = fs::write(&tmp, source) {
            eprintln!("failed to write repl temp file: {e}");
            continue;
        }
        let status = Command::new(env::current_exe().unwrap_or_else(|_| PathBuf::from("korlang")))
            .arg("run")
            .arg(&tmp)
            .status();
        match status {
            Ok(s) if s.success() => {
                if is_binding {
                    state_lines.push(line);
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("repl execution failed: {e}"),
        }
    }
    let _ = fs::create_dir_all(hist_path.parent().unwrap_or_else(|| std::path::Path::new(".")));
    let mut out = String::new();
    for h in &history {
        out.push_str(h);
        out.push('\n');
    }
    let _ = fs::write(hist_path, out);
}

fn repl_keywords() -> Vec<&'static str> {
    vec![
        "fun", "let", "if", "else", "while", "for", "match", "return", "struct", "enum", "view",
        "resource", "print", "println", "readLine",
    ]
}

fn bootstrap() {
    let exe = env::current_exe().ok();
    let root = exe
        .as_ref()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let script = root.join("scripts").join("bootstrap.sh");
    let status = Command::new("bash").arg(script).status();
    match status {
        Ok(s) if s.success() => {}
        _ => eprintln!("bootstrap failed"),
    }
}

fn hash_str(s: &str) -> String {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:x}", h.finish())
}

fn build_native_selfhosted() {
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    let out_dir = root.join("build");
    let runtime_home = out_dir.join("runtime");
    let runtime_lib = runtime_home.join("lib").join("libkorlang_rt.a");
    let out_file = out_dir.join("selfhosted.kor");
    let out_bin = out_dir.join("korlang-selfhosted");

    let _ = fs::create_dir_all(out_dir.join("runtime/lib"));

    let mut files = Vec::new();
    collect_korlang_files_top_level(&root.join("src/compiler/korlang"), &mut files);
    files.sort();
    if files.is_empty() {
        eprintln!("no Korlang compiler sources found under src/compiler/korlang");
        std::process::exit(1);
    }

    let mut merged = String::new();
    for f in files {
        let src = match fs::read_to_string(&f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("failed to read {}: {}", f.display(), e);
                std::process::exit(1);
            }
        };
        for line in src.lines() {
            let s = line.trim();
            if s.starts_with("module ") || s.starts_with("import ") {
                continue;
            }
            if s.starts_with("fun ") && !s.contains('{') && !s.ends_with('{') {
                continue;
            }
            merged.push_str(line);
            merged.push('\n');
        }
        merged.push('\n');
    }
    merged = normalize_generics(merged);

    if let Err(e) = fs::write(&out_file, merged) {
        eprintln!("failed to write {}: {}", out_file.display(), e);
        std::process::exit(1);
    }

    if !runtime_lib.exists() {
        let mut copied = false;
        for cand in [
            root.join("dist/runtime/lib/libkorlang_rt.a"),
            root.join("src/runtime/lib/libkorlang_rt.a"),
            root.join("src/runtime/target/release/libkorlang_rt.a"),
            root.join("src/runtime/target/debug/libkorlang_rt.a"),
        ] {
            if cand.exists() {
                if let Err(e) = fs::copy(&cand, &runtime_lib) {
                    eprintln!("failed to copy runtime lib from {}: {}", cand.display(), e);
                    std::process::exit(1);
                }
                copied = true;
                break;
            }
        }
        if !copied {
            eprintln!("missing runtime lib: place libkorlang_rt.a at build/runtime/lib or dist/runtime/lib");
            std::process::exit(1);
        }
    }

    let stage1 = root.join("dist/bootstrap-stage1/bin/korlang");
    let korlang_bin = env::var("KORLANG_BIN").map(PathBuf::from).unwrap_or(stage1);
    if !korlang_bin.exists() {
        eprintln!("missing KORLANG_BIN: {}", korlang_bin.display());
        std::process::exit(1);
    }

    let status = Command::new(&korlang_bin)
        .env("KORLANG_HOME", &runtime_home)
        .env("KORLANG_SEMA_PERMISSIVE", "1")
        .arg("build")
        .arg(&out_file)
        .arg("-o")
        .arg(&out_bin)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("native selfhost build complete: {}", out_bin.display());
        }
        _ => {
            eprintln!("native selfhost build failed");
            std::process::exit(1);
        }
    }
}

fn find_repo_root() -> Option<PathBuf> {
    let mut cur = env::current_dir().ok()?;
    for _ in 0..8 {
        if cur.join("src/compiler/korlang").exists() && cur.join("scripts").exists() {
            return Some(cur);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

fn collect_korlang_files_top_level(dir: &PathBuf, out: &mut Vec<PathBuf>) {
    let rd = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(_) => return,
    };
    for ent in rd.flatten() {
        let p = ent.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("kor") {
            out.push(p);
        }
    }
}

fn normalize_generics(mut text: String) -> String {
    while text.contains("List<") {
        text = replace_generic(&text, "List", true);
    }
    while text.contains("Result<") {
        text = replace_generic(&text, "Result", false);
    }
    text
}

fn replace_generic(text: &str, name: &str, to_brackets: bool) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let pat = format!("{}<", name);
    let patb = pat.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        if i + patb.len() <= bytes.len() && &bytes[i..i + patb.len()] == patb {
            i += patb.len();
            let mut depth = 1i32;
            let start = i;
            while i < bytes.len() && depth > 0 {
                match bytes[i] as char {
                    '<' => depth += 1,
                    '>' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
            let inner_end = i.saturating_sub(1);
            let inner = text[start..inner_end].trim();
            if to_brackets {
                out.push('[');
                out.push_str(inner);
                out.push(']');
            } else {
                out.push_str("Any");
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}
