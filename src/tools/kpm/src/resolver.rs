use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub versions: Vec<Version>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub req: VersionReq,
}

#[derive(Debug, Default)]
pub struct Resolution {
    pub selected: HashMap<String, Version>,
}

pub fn resolve(root: &[Dependency], universe: &HashMap<String, Package>) -> Result<Resolution, String> {
    let mut res = Resolution::default();
    let mut visiting = HashSet::new();
    for dep in root {
        resolve_dep(dep, universe, &mut res, &mut visiting)?;
    }
    Ok(res)
}

fn resolve_dep(
    dep: &Dependency,
    universe: &HashMap<String, Package>,
    res: &mut Resolution,
    visiting: &mut HashSet<String>,
) -> Result<(), String> {
    if let Some(v) = res.selected.get(&dep.name) {
        if dep.req.matches(v) {
            return Ok(());
        }
        return Err(format!("version conflict for {}", dep.name));
    }
    if !visiting.insert(dep.name.clone()) {
        return Err(format!("cycle detected at {}", dep.name));
    }

    let pkg = universe
        .get(&dep.name)
        .ok_or_else(|| format!("unknown package {}", dep.name))?;

    let mut candidates: Vec<_> = pkg
        .versions
        .iter()
        .filter(|v| dep.req.matches(v))
        .cloned()
        .collect();
    candidates.sort();
    let chosen = candidates.last().cloned().ok_or_else(|| format!("no version for {}", dep.name))?;
    res.selected.insert(dep.name.clone(), chosen);

    visiting.remove(&dep.name);
    Ok(())
}

