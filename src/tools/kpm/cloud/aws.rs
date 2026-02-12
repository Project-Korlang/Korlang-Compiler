use super::{Plan, Provider, ResourceSpec};

pub struct AwsProvider;

impl Provider for AwsProvider {
    fn plan(&self, resources: &[ResourceSpec]) -> Plan {
        let actions = resources
            .iter()
            .map(|r| format!("create {} ({})", r.name, r.r#type))
            .collect();
        Plan { actions }
    }

    fn apply(&self, resources: &[ResourceSpec]) -> Result<(), String> {
        for r in resources {
            println!("[aws] apply {} ({})", r.name, r.r#type);
        }
        Ok(())
    }

    fn destroy(&self, resources: &[ResourceSpec]) -> Result<(), String> {
        for r in resources {
            println!("[aws] destroy {} ({})", r.name, r.r#type);
        }
        Ok(())
    }
}

