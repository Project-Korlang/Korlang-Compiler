use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub name: String,
    pub r#type: String,
    pub props: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub actions: Vec<String>,
}

pub trait Provider {
    fn plan(&self, resources: &[ResourceSpec]) -> Plan;
    fn apply(&self, resources: &[ResourceSpec]) -> Result<(), String>;
    fn destroy(&self, resources: &[ResourceSpec]) -> Result<(), String>;
}

