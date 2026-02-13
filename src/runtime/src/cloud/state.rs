use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub resources: HashMap<String, ResourceState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub r#type: String,
    pub props_hash: String,
    pub outputs: HashMap<String, String>,
}

impl State {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let data = fs::read(path).map_err(|e| e.to_string())?;
        serde_json::from_slice(&data).map_err(|e| e.to_string())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn detect_drift(&self, other: &State) -> Vec<String> {
        let mut drift = Vec::new();
        for (name, state) in &self.resources {
            match other.resources.get(name) {
                Some(remote) => {
                    if state.props_hash != remote.props_hash {
                        drift.push(format!("resource {} props_hash mismatch", name));
                    }
                    if state.r#type != remote.r#type {
                        drift.push(format!("resource {} type changed", name));
                    }
                }
                None => drift.push(format!("resource {} is missing remotely", name)),
            }
        }
        for name in other.resources.keys() {
            if !self.resources.contains_key(name) {
                drift.push(format!("resource {} exists remotely but not locally", name));
            }
        }
        drift
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn build_sample() -> State {
        let mut outputs = HashMap::new();
        outputs.insert("endpoint".into(), "https://example".into());
        let resource = ResourceState {
            r#type: "korlang.aws::Instance".into(),
            props_hash: "hash-123".into(),
            outputs,
        };
        let mut resources = HashMap::new();
        resources.insert("app".into(), resource);
        State { resources }
    }

    fn temp_state_path() -> (PathBuf, PathBuf) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let root = std::env::temp_dir().join(format!("korlang-cloud-state-{now}"));
        let _ = fs::create_dir_all(&root);
        let path = root.join("state.json");
        (root, path)
    }

    #[test]
    fn persist_and_load_state() {
        let (root, path) = temp_state_path();
        let sample = build_sample();
        sample.save(&path).expect("save state");
        let loaded = State::load(&path).expect("load state");
        assert_eq!(loaded.resources.len(), 1);
        assert!(loaded.resources.contains_key("app"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn detect_prop_drift() {
        let a = build_sample();
        let mut b = build_sample();
        if let Some(state) = b.resources.get_mut("app") {
            state.props_hash = "different".into();
        }
        let drift = a.detect_drift(&b);
        assert!(drift.iter().any(|m| m.contains("props_hash mismatch")));
    }

    #[test]
    fn detect_missing_resource() {
        let a = build_sample();
        let b = State::default();
        let drift = a.detect_drift(&b);
        assert!(drift.iter().any(|m| m.contains("missing remotely")));
    }
}
