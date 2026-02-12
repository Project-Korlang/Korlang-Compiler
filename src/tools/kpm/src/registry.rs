use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEntry {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub packages: Vec<PackageEntry>,
}

pub fn load_index(path: &Path) -> Result<Index, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

pub fn save_index(path: &Path, index: &Index) -> Result<(), String> {
    let data = serde_json::to_string_pretty(index).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

pub fn download(_url: &str, _dest: &Path) -> Result<(), String> {
    // Placeholder: network fetch not wired yet.
    Err("download not implemented".into())
}

pub fn upload(_url: &str, _file: &Path) -> Result<(), String> {
    // Placeholder: network upload not wired yet.
    Err("upload not implemented".into())
}

