use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

mod resolver;
mod build_hooks;
mod registry;
mod github_registry;

#[derive(Debug, Deserialize)]
struct Config {
    package: Option<Package>,
    dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    source: Option<String>,
    version: Option<String>,
    path: Option<String>,
    repo: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: kpm <Korlang.config>");
        std::process::exit(1);
    }
    let contents = fs::read_to_string(&args[1]).expect("read config");
    let cfg: Config = toml::from_str(&contents).expect("parse config");

    if let Some(pkg) = cfg.package {
        println!("package: {} {}", pkg.name.unwrap_or_default(), pkg.version.unwrap_or_default());
    }
    if let Some(deps) = cfg.dependencies {
        for d in deps {
            println!("dep: {} {:?} {:?}", d.name, d.source, d.version.or(d.path).or(d.repo));
        }
    }

    let state_path = Path::new("KPM.lock");
    if !state_path.exists() {
        let lock = "[package]\nname = \"\"\nversion = \"\"\n";
        let _ = fs::write(state_path, lock);
    }
}
