use std::process::Command;

#[derive(Debug, Clone)]
pub struct BuildHook {
    pub name: String,
    pub cmd: Vec<String>,
}

pub fn run_hook(hook: &BuildHook) -> Result<(), String> {
    if hook.cmd.is_empty() {
        return Err("empty hook command".into());
    }
    let mut c = Command::new(&hook.cmd[0]);
    if hook.cmd.len() > 1 {
        c.args(&hook.cmd[1..]);
    }
    let status = c.status().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("hook failed: {}", hook.name))
    }
}

