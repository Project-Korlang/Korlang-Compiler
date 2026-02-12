use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub r#type: String,
    pub props: serde_json::Value,
}

pub trait ResourceRuntime {
    fn plan(&self, resources: &[Resource]) -> Vec<String>;
    fn apply(&self, resources: &[Resource]) -> Result<(), String>;
    fn destroy(&self, resources: &[Resource]) -> Result<(), String>;
}

