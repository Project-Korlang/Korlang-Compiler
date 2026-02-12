use std::process::Command;

#[derive(Debug, Clone)]
pub struct BuildHook {
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn run_hook(hook: &BuildHook) -> Result<(), String> {
    let status = Command::new(&hook.cmd)
        .args(&hook.args)
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("hook failed: {}", hook.cmd))
    }
}

