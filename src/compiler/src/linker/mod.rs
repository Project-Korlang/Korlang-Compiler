use std::path::{Path, PathBuf};

pub mod lto;

#[derive(Debug, Clone)]
pub struct LinkerConfig {
    pub output: PathBuf,
    pub runtime_lib: PathBuf,
    pub extra_args: Vec<String>,
    pub lto: Option<LtoMode>,
    pub pgo_generate: bool,
    pub pgo_use: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum LtoMode {
    Full,
    Thin,
}

pub fn build_link_command(object_file: &Path, config: &LinkerConfig) -> Vec<String> {
    let mut cmd = vec![
        "cc".to_string(),
        object_file.display().to_string(),
        config.runtime_lib.display().to_string(),
        "-o".to_string(),
        config.output.display().to_string(),
    ];
    if let Some(mode) = config.lto {
        match mode {
            LtoMode::Full => cmd.push("-flto".to_string()),
            LtoMode::Thin => cmd.push("-flto=thin".to_string()),
        }
    }
    if config.pgo_generate {
        cmd.push("-fprofile-generate".to_string());
    }
    if let Some(profile) = &config.pgo_use {
        cmd.push(format!("-fprofile-use={}", profile.display()));
    }
    cmd.extend(config.extra_args.iter().cloned());
    cmd
}
