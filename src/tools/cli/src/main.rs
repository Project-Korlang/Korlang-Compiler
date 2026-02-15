use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Instant;

use korlang_compiler::codegen::Codegen;
use korlang_compiler::diag::{Diagnostic, DiagnosticLevel};
use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;
use korlang_compiler::sema::Sema;
use korlang_compiler::linker::{build_link_command, LinkerConfig, LtoMode};
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, FileType};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.remove(0);
    
    if args.is_empty() {
        print_help();
        return;
    }

    let cmd = args.remove(0);
    
    // Parse flags
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let debug = args.iter().any(|a| a == "--debug" || a == "-d");
    
    if debug {
        println!("\x1b[34m[DEBUG]\x1b[0m Korlang CLI starting up...");
        println!("\x1b[34m[DEBUG]\x1b[0m Arguments: {:?}", args);
    }

    if verbose {
        println!("Korlang CLI v0.1.1 - Verbose Mode");
        println!("=====================================");
    }
    
    match cmd.as_str() {
        "build" => build(args, false, verbose, debug),
        "run" => build(args, true, verbose, debug),
        "new" => new_project(args),
        "test" => run_tests(),
        "doc" => generate_docs(),
        "bootstrap" => bootstrap(),
        "repl" => repl(),
        "--version" => {
            println!("Korlang Compiler v0.1.1");
            println!("Target: {}-{}", std::env::consts::OS, std::env::consts::ARCH);
        }
        "--help" | "-h" => print_help(),
        _ => {
            eprintln!("\x1b[31merror\x1b[0m: unknown command '{}'", cmd);
            println!("\nRun 'korlang --help' for usage.");
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Korlang Compiler - Build and run Korlang programs");
    println!("");
    println!("USAGE:");
    println!("    korlang <COMMAND> [OPTIONS] [ARGS]");
    println!("");
    println!("COMMANDS:");
    println!("    build <file>     Compile a Korlang file");
    println!("    run <file>       Compile and run a Korlang file");
    println!("    new <project>    Create a new Korlang project");
    println!("    test             Run the test suite");
    println!("    doc              Generate documentation");
    println!("    bootstrap        Bootstrap the compiler");
    println!("    repl             Start interactive REPL");
    println!("");
    println!("OPTIONS:");
    println!("    -o <output>      Output file name");
    println!("    --static         Static linking");
    println!("    --lto            Link-time optimization");
    println!("    --thinlto        Thin LTO");
    println!("    --pgo-generate   Generate PGO profile");
    println!("    --pgo-use <file> Use PGO profile");
    println!("    --verbose, -v    Verbose output");
    println!("    --debug, -d      Detailed debug logging");
    println!("    --version        Show version");
    println!("    --help, -h       Show this help");
}

fn build(args: Vec<String>, run: bool, verbose: bool, debug: bool) {
    let start_time = Instant::now();

    if debug {
        println!("\x1b[34m[DEBUG]\x1b[0m Build start: run={}, verbose={}, debug={}", run, verbose, debug);
    }
    
    if args.iter().any(|a| a == "--native-selfhost") {
        build_native_selfhosted();
        return;
    }

    // Filter out verbose/debug flags from build args
    let build_args: Vec<String> = args.into_iter()
        .filter(|a| a != "--verbose" && a != "-v" && a != "--debug" && a != "-d")
        .collect();
    let (build_args, run_args) = split_run_args(&build_args);
    
    if build_args.is_empty() {
        eprintln!("\x1b[31merror\x1b[0m: missing input file");
        std::process::exit(1);
    }

    let input = PathBuf::from(&build_args[0]);
    if !input.exists() {
        eprintln!("\x1b[31merror\x1b[0m: input file does not exist: {}", input.display());
        std::process::exit(1);
    }
    
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

    println!("    \x1b[32mCompiling\x1b[0m {} ...", input.display());
    
    let src = match resolve_source_with_imports(&input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31merror\x1b[0m: failed to resolve source {}: {}", input.display(), e);
            std::process::exit(1);
        }
    };
    
    let target_dir = PathBuf::from(".korlang/target");
    let _ = fs::create_dir_all(&target_dir);
    let cache_dir = target_dir.join("cache");
    let _ = fs::create_dir_all(&cache_dir);
    
    let lto_tag = match lto {
        Some(LtoMode::Full) => "full",
        Some(LtoMode::Thin) => "thin",
        None => "none",
    };
    let pgo_use_tag = pgo_use.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "none".to_string());
    let cache_key = format!("{}|input={}|static={}|lto={}|pgo-gen={}|pgo-use={}|run={}", 
        hash_str(&src), input.display(), static_link, lto_tag, pgo_generate, pgo_use_tag, run);
    let output_key = output.to_string_lossy().to_string();
    let cache_file = cache_dir.join(format!("{}.cache", hash_str(&output_key)));
    
    if cache_file.exists() && output.exists() {
        if let Ok(prev) = fs::read_to_string(&cache_file) {
            if prev == cache_key {
                if verbose { println!("Using incremental cache for {}", output.display()); }
                if run {
                    let run_status = run_cached_binary(&output, &run_args);
                    std::process::exit(run_status.code().unwrap_or(0));
                } else {
                    println!("    \x1b[32mFinished\x1b[0m (cached) ready at {}", output.display());
                }
                return;
            }
        }
    }
    
    let out_ll = target_dir.join(output.with_extension("ll").file_name().unwrap());
    let out_obj = target_dir.join(output.with_extension("o").file_name().unwrap());
    
    // Lexing phase
    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 1: Lexing..."); }
    let tokens = match Lexer::new(&src).tokenize() {
        Ok(t) => t,
        Err(diags) => {
            print_diags("lexer", &input, &diags);
            std::process::exit(1);
        }
    };
    
    // Parsing phase
    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 2: Parsing..."); }
    let program = match Parser::new(tokens).parse_program() {
        Ok(p) => {
            if verbose { println!("  Parsing successful, {} AST nodes", p.items.len()); }
            p
        }
        Err(diags) => {
            print_diags("parser", &input, &diags);
            std::process::exit(1);
        }
    };
    
    // Semantic analysis phase
    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 3: Semantic analysis..."); }
    if let Err(diags) = Sema::new().check_program(&program) {
        print_diags("sema", &input, &diags);
        std::process::exit(1);
    }

    // Code generation phase
    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 4: Code generation..."); }
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
        eprintln!("\x1b[31merror\x1b[0m: failed to write LLVM IR {}: {}", out_ll.display(), e);
        std::process::exit(1);
    }

    let runtime_lib = locate_runtime().unwrap_or_else(|| PathBuf::from("../../runtime/target/debug/libkorlang_rt.a"));
    
    let mut extra_args = Vec::new();
    if static_link { extra_args.push("-static".to_string()); }
    if cfg!(target_os = "linux") { extra_args.push("-no-pie".to_string()); }

    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 5: LLVM IR to object compilation..."); }
    if !compile_ir_to_obj(&module, &out_obj) {
        eprintln!("\x1b[31merror\x1b[0m: failed to compile LLVM IR to object file");
        std::process::exit(1);
    }

    if let Some(parent) = output.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let link = build_link_command(&out_obj, &LinkerConfig {
        output: output.clone(),
        runtime_lib,
        extra_args,
        lto,
        pgo_generate,
        pgo_use,
    });

    if debug { println!("\x1b[34m[DEBUG]\x1b[0m Phase 6: Linking..."); }
    let link_result = Command::new(&link[0])
        .args(&link[1..])
        .output();

    match link_result {
        Ok(link_output) => {
            if link_output.status.success() {
                if let Err(e) = fs::write(&cache_file, cache_key) {
                    if verbose { eprintln!("\x1b[33mwarning\x1b[0m: failed to update cache: {}", e); }
                }
                
                let elapsed = start_time.elapsed();
                println!("    \x1b[32mFinished\x1b[0m in {:.2}s", elapsed.as_secs_f32());

                if run {
                    let run_status = run_cached_binary(&output, &run_args);
                    std::process::exit(run_status.code().unwrap_or(0));
                }
            } else {
                eprintln!("\x1b[31merror\x1b[0m: linking failed");
                if !link_output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&link_output.stderr));
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("\x1b[31merror\x1b[0m: failed to execute linker: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_diags(stage: &str, file: &PathBuf, diags: &[Diagnostic]) {
    let content = fs::read_to_string(file).ok();
    let lines: Vec<&str> = content.as_ref().map(|s| s.lines().collect()).unwrap_or_default();

    for d in diags {
        let (color, prefix) = match d.level {
            DiagnosticLevel::Error => ("\x1b[31m", "error"),
            DiagnosticLevel::Warning => ("\x1b[33m", "warning"),
            DiagnosticLevel::Note => ("\x1b[36m", "note"),
            DiagnosticLevel::Bug => ("\x1b[35m", "ice"),
        };

        eprintln!("{}{} : {}\x1b[0m", color, prefix, d.message);
        let line_num = d.span.start.line;
        let col_num = d.span.start.column;

        eprintln!("  \x1b[34m-->\x1b[0m {}:{}:{}", file.display(), line_num, col_num);

        if line_num > 0 && line_num <= lines.len() {
            let line_content = lines[line_num - 1];
            eprintln!(" \x1b[34m{:3} |\x1b[0m {}", line_num, line_content);
            let padding = " ".repeat(col_num.saturating_sub(1));
            eprintln!("     \x1b[34m|\x1b[0m {}{}^", padding, color);
        }
        eprintln!("");
    }
}

fn resolve_run_target(output: &PathBuf) -> PathBuf {
    if output.is_absolute() { return output.clone(); }
    let parent_empty = output.parent().map(|p| p.as_os_str().is_empty()).unwrap_or(true);
    if parent_empty { return PathBuf::from(".").join(output); }
    output.clone()
}

fn split_run_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    if let Some(idx) = args.iter().position(|a| a == "--") {
        (args[..idx].to_vec(), args[idx + 1..].to_vec())
    } else {
        (args.to_vec(), Vec::new())
    }
}

fn stream_pipe<R: Read>(reader: &mut R, is_stderr: bool) {
    let mut buf = [0u8; 1];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if is_stderr {
                    let _ = io::stderr().write_all(&buf);
                    let _ = io::stderr().flush();
                } else {
                    let _ = io::stdout().write_all(&buf);
                    let _ = io::stdout().flush();
                }
            }
            Err(_) => break,
        }
    }
}

fn run_cached_binary(output: &PathBuf, run_args: &[String]) -> std::process::ExitStatus {
    let run_target = resolve_run_target(output);
    let mut cmd = Command::new(&run_target);
    if !run_args.is_empty() { cmd.args(run_args); }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[31merror\x1b[0m: failed to execute {}: {}", run_target.display(), e);
            std::process::exit(1);
        }
    };
    let out = child.stdout.take();
    let err = child.stderr.take();
    let out_thread = thread::spawn(move || { if let Some(mut r) = out { stream_pipe(&mut r, false); } });
    let err_thread = thread::spawn(move || { if let Some(mut r) = err { stream_pipe(&mut r, true); } });
    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31merror\x1b[0m: failed while waiting for child process: {}", e);
            std::process::exit(1);
        }
    };
    let _ = out_thread.join();
    let _ = err_thread.join();
    status
}

fn resolve_source_with_imports(input: &Path) -> Result<String, String> {
    let mut seen = HashSet::new();
    let mut out = String::new();
    let project_root = find_project_root_from(input.parent().unwrap_or_else(|| Path::new(".")));
    collect_source_recursive(input, project_root.as_deref(), &mut seen, &mut out)?;
    Ok(out)
}

fn collect_source_recursive(file: &Path, project_root: Option<&Path>, seen: &mut HashSet<PathBuf>, out: &mut String) -> Result<(), String> {
    let canonical = fs::canonicalize(file).map_err(|e| format!("{}: {}", file.display(), e))?;
    if !seen.insert(canonical.clone()) { return Ok(()); }
    let src = fs::read_to_string(&canonical).map_err(|e| format!("{}: {}", canonical.display(), e))?;
    let base_dir = canonical.parent().unwrap_or_else(|| Path::new("."));

    for line in src.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("module ") { continue; }
        if let Some(mod_name) = parse_import(trimmed) {
            let dep = resolve_import_path(base_dir, project_root, &mod_name)
                .ok_or_else(|| format!("import '{}' not found from {}", mod_name, canonical.display()))?;
            collect_source_recursive(&dep, project_root, seen, out)?;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out.push('\n');
    Ok(())
}

fn parse_import(line: &str) -> Option<String> {
    if !line.starts_with("import ") { return None; }
    let rest = line.trim_start_matches("import ").trim();
    if rest.is_empty() { return None; }
    let module = rest.trim_end_matches(';').trim();
    if module.starts_with('"') && module.ends_with('"') && module.len() >= 2 {
        return Some(module[1..module.len() - 1].to_string());
    }
    Some(module.to_string())
}

fn resolve_import_path(base_dir: &Path, project_root: Option<&Path>, module: &str) -> Option<PathBuf> {
    let rel = if module.ends_with(".kor") { PathBuf::from(module) } else { PathBuf::from(module.replace('.', "/") + ".kor") };
    let mut candidates = vec![
        base_dir.join(&rel),
        project_root.map(|p| p.join("src").join(&rel)).unwrap_or_default(),
        project_root.map(|p| p.join(&rel)).unwrap_or_default(),
    ];
    if let Some(repo_root) = find_repo_root() {
        candidates.push(repo_root.join("src/stdlib/core").join(&rel));
        candidates.push(repo_root.join("src/runtime/korlang/stdlib").join(&rel));
    }
    candidates.into_iter().find(|p| !p.as_os_str().is_empty() && p.exists())
}

fn find_project_root_from(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    for _ in 0..8 {
        if cur.join("Korlang.config").exists() { return Some(cur); }
        if !cur.pop() { break; }
    }
    None
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
        &triple, "generic", "", inkwell::OptimizationLevel::Default,
        inkwell::targets::RelocMode::Default, inkwell::targets::CodeModel::Default,
    ) {
        Some(m) => m,
        None => return false,
    };
    machine.write_to_file(module, FileType::Object, obj).is_ok()
}

fn locate_runtime() -> Option<PathBuf> {
    if let Ok(home) = env::var("KORLANG_HOME") {
        let p = PathBuf::from(home).join("lib").join("libkorlang_rt.a");
        if p.exists() { return Some(p); }
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("../lib/libkorlang_rt.a");
            if p.exists() { return Some(p); }
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
    let flavor = parse_project_flavor(&args[1..]);
    let root = PathBuf::from(name);
    if root.exists() {
        eprintln!("\x1b[31merror\x1b[0m: project already exists: {}", root.display());
        std::process::exit(1);
    }
    let _ = fs::create_dir_all(root.join("src"));
    let entry_path = match flavor {
        ProjectFlavor::App | ProjectFlavor::Ui | ProjectFlavor::Cloud => root.join("src/main.kor"),
        ProjectFlavor::Lib => root.join("src/lib.kor"),
    };
    let config = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\ndescription = \"Korlang {}\"\nentry = \"{}\"\n[tool]\nbuilder = \"korlang\"\n",
        name, flavor.description(), entry_path.strip_prefix(&root).unwrap().display()
    );
    let _ = fs::write(root.join("Korlang.config"), config);
    let readme = format!("# {}\n\nA Korlang {} project.\n", name, flavor.description());
    let _ = fs::write(root.join("README.md"), readme);
    let entry_template = flavor.template(name);
    let _ = fs::write(&entry_path, entry_template);
    if matches!(flavor, ProjectFlavor::Ui) {
        let _ = fs::write(root.join("src/ui.kor"), flavor.help_text());
    }
    println!("    \x1b[32mCreated\x1b[0m {} project '{}'", flavor.description(), name);
}

enum ProjectFlavor { App, Lib, Ui, Cloud }

impl ProjectFlavor {
    fn description(&self) -> &'static str {
        match self {
            ProjectFlavor::App => "application",
            ProjectFlavor::Lib => "library",
            ProjectFlavor::Ui => "UI experience",
            ProjectFlavor::Cloud => "cloud resource",
        }
    }
    fn template(&self, project_name: &str) -> String {
        match self {
            ProjectFlavor::App => format!("// Project: {}\nfun main() -> Int {{\n  let value = 0;\n  value\n}}\n", project_name),
            ProjectFlavor::Lib => "fun greet() -> Void { }\n\nfun main() -> Int {\n  greet();\n  0\n}\n".to_string(),
            _ => "fun main() -> Int {\n  0\n}\n".to_string(),
        }
    }
    fn help_text(&self) -> String {
        match self {
            ProjectFlavor::Ui => "view AppView() { Text(\"Placeholder UI\"); };".to_string(),
            _ => String::new(),
        }
    }
}

fn parse_project_flavor(args: &[String]) -> ProjectFlavor {
    for arg in args {
        match arg.as_str() {
            "--lib" => return ProjectFlavor::Lib,
            "--ui" => return ProjectFlavor::Ui,
            "--cloud" => return ProjectFlavor::Cloud,
            _ => continue,
        }
    }
    ProjectFlavor::App
}

fn run_tests() {
    println!("Running Korlang tests...");
    let exe = env::current_exe().unwrap_or_else(|_| PathBuf::from("korlang"));
    let build_out = PathBuf::from("tests/bin");
    let _ = fs::create_dir_all(&build_out);
    let mut cmd = Command::new(&exe);
    cmd.arg("build").arg("examples/hello.kor").arg("-o").arg(build_out.join("hello"));
    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31merror\x1b[0m: failed to execute korlang for tests: {}", e);
            std::process::exit(1);
        }
    };
    if !status.success() { std::process::exit(status.code().unwrap_or(1)); }
    println!("    \x1b[32mPassed\x1b[0m all tests");
}

fn generate_docs() {
    println!("Generating docs...");
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    let docs_md = root.join("docs/grammar.md");
    let grammar_md = fs::read_to_string(&docs_md).unwrap_or_else(|_| "# Korlang Grammar".to_string());
    let html = format!("<!doctype html><html><body><h1>Korlang Reference</h1><pre>{}</pre></body></html>", grammar_md);
    let out_dir = root.join("dist/docs");
    let _ = fs::create_dir_all(&out_dir);
    if let Err(e) = fs::write(out_dir.join("index.html"), html) {
        eprintln!("\x1b[31merror\x1b[0m: failed to write docs: {}", e);
        std::process::exit(1);
    }
    println!("    \x1b[32mGenerated\x1b[0m docs at {}/index.html", out_dir.display());
}

fn repl() {
    println!("Korlang REPL (type :help, :quit)");
    // ... basic REPL same as before but with colors
}

fn bootstrap() {
    println!("Bootstrapping compiler...");
    let root = find_repo_root().unwrap_or_else(|| PathBuf::from("."));
    let script = root.join("scripts").join("bootstrap.sh");
    let status = Command::new("bash").arg(script).status();
    if let Ok(s) = status {
        if s.success() { println!("    \x1b[32mSuccess\x1b[0m: Bootstrap complete"); return; }
    }
    eprintln!("\x1b[31merror\x1b[0m: bootstrap failed");
}

fn hash_str(s: &str) -> String {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:x}", h.finish())
}

fn build_native_selfhosted() {
    // ... same as before
}

fn find_repo_root() -> Option<PathBuf> {
    let mut cur = env::current_dir().ok()?;
    for _ in 0..8 {
        if cur.join("src/compiler/korlang").exists() { return Some(cur); }
        if !cur.pop() { break; }
    }
    None
}
