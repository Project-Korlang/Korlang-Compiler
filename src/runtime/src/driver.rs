use std::env;
use std::process::Command;

pub fn korlang_driver() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("korlang: no args (set KORLANG_BOOTSTRAP to bootstrap compiler)");
        return;
    }
    let bootstrap = match env::var("KORLANG_BOOTSTRAP") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("KORLANG_BOOTSTRAP not set; cannot delegate");
            return;
        }
    };
    // Replace argv0 with bootstrap compiler path.
    args[0] = bootstrap.clone();
    let status = Command::new(&bootstrap)
        .args(&args[1..])
        .status();
    if let Ok(s) = status {
        if !s.success() {
            eprintln!("bootstrap compiler failed");
        }
    } else {
        eprintln!("failed to launch bootstrap compiler");
    }
}
