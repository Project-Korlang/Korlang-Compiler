use crate::registry_client::Index;
use std::path::Path;

pub fn init_github_registry(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path.join("packages")).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(path.join("authors")).map_err(|e| e.to_string())?;
    let index = Index { packages: Default::default() };
    crate::registry_client::save_index(&path.join("index.json"), &index)?;
    Ok(())
}

