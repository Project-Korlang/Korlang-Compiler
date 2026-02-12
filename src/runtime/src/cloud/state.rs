use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

