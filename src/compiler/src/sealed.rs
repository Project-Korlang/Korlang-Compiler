use crate::ast::{SealedDecl, Item};
use crate::sema::Sema;
use crate::symbols::Symbol;
use std::collections::HashSet;

pub struct SealedEnforcer<'a> {
    sema: &'a mut Sema,
}

impl<'a> SealedEnforcer<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self { sema }
    }

    pub fn validate_sealed_decl(&mut self, s: &SealedDecl) {
        let mut child_names = HashSet::new();
        for item in &s.items {
            match item {
                Item::Struct(st) => {
                    child_names.insert(st.name);
                }
                Item::Enum(e) => {
                    child_names.insert(e.name);
                }
                _ => {
                    self.sema.diags.push(crate::diag::Diagnostic::error("sealed classes can only contain structs or enums", s.span));
                }
            }
        }
    }

    pub fn get_exhaustiveness_info(&self, sealed_name: Symbol) -> Vec<Symbol> {
        if let Some(s) = self.sema.sealed_types.get(&sealed_name) {
            s.items.iter().filter_map(|item| {
                match item {
                    Item::Struct(st) => Some(st.name),
                    Item::Enum(e) => Some(e.name),
                    _ => None,
                }
            }).collect()
        } else {
            Vec::new()
        }
    }
}
