use crate::types::Type;
use crate::ast::InterfaceDecl;
use std::collections::HashMap;

pub struct InterfaceSystem {
    pub interfaces: HashMap<String, InterfaceDecl>,
    pub implementations: HashMap<String, Vec<Type>>, // Interface Name -> Implementing Types
}

impl InterfaceSystem {
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            implementations: HashMap::new(),
        }
    }

    pub fn register_interface(&mut self, decl: InterfaceDecl) {
        self.interfaces.insert(decl.name.clone(), decl);
    }

    pub fn register_implementation(&mut self, interface_name: &str, ty: Type) {
        let entry = self.implementations.entry(interface_name.to_string()).or_insert_with(Vec::new);
        entry.push(ty);
    }
}
