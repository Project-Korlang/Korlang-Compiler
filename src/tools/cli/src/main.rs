use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

use korlang_compiler::codegen::Codegen;
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
        _ => {
            eprintln!("Usage: korlang <build|run|new|test|doc|bootstrap> <file.kor> [-o out] [--static] [--lto|--thinlto] [--pgo-generate] [--pgo-use file]");
            eprintln!("       korlang build --native-selfhost");
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
    let input = PathBuf::from(&args[0]);
    let mut output = PathBuf::from("a.out");
    let mut static_link = false;
    let mut lto = None;
    let mut pgo_generate = false;
    let mut pgo_use: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            output = PathBuf::from(&args[i + 1]);
            i += 2;
        } else if args[i] == "--static" {
            static_link = true;
            i += 1;
        } else if args[i] == "--lto" {
            lto = Some(LtoMode::Full);
            i += 1;
        } else if args[i] == "--thinlto" {
            lto = Some(LtoMode::Thin);
            i += 1;
        } else if args[i] == "--pgo-generate" {
            pgo_generate = true;
            i += 1;
        } else if args[i] == "--pgo-use" && i + 1 < args.len() {
            pgo_use = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else {
            i += 1;
        }
    }

    let src = fs::read_to_string(&input).expect("read source");
    let target_dir = PathBuf::from(".korlang/target");
    let _ = fs::create_dir_all(&target_dir);
    let out_ll = target_dir.join(output.with_extension("ll").file_name().unwrap());
    let out_obj = target_dir.join(output.with_extension("o").file_name().unwrap());
    let tokens = Lexer::new(&src).tokenize().expect("lex");
    let program = Parser::new(tokens).parse_program().expect("parse");
    Sema::new().check_program(&program).expect("sema");

    let context = Context::create();
    let codegen = Codegen::new(&context, "main");
    let module = codegen.emit_program(&program).expect("codegen");
    module.print_to_file(&out_ll).expect("write IR");

    let runtime_lib = locate_runtime().unwrap_or_else(|| PathBuf::from("../../runtime/target/debug/libkorlang_rt.a"));
    let mut extra_args = Vec::new();
    if static_link {
        extra_args.push("-static".to_string());
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

    println!("LLVM IR written to: {}", out_ll.display());
    println!("Link with: {}", link.join(" "));

    let status = Command::new(&link[0])
        .args(&link[1..])
        .status();

    match status {
        Ok(s) if s.success() => {
            if run {
                let _ = Command::new(&output).status();
            }
        }
        _ => eprintln!("link failed; binary not executed"),
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
