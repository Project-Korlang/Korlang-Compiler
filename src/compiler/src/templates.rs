use crate::ast::*;
use crate::sema::Type;
use crate::symbols::Symbol;
use std::collections::HashMap;

pub struct TemplateSystem {
    pub instantiations: HashMap<Symbol, Type>,
}

impl TemplateSystem {
    pub fn new() -> Self {
        Self {
            instantiations: HashMap::new(),
        }
    }

    pub fn instantiate_type(&mut self, base_ty: &Type, args: &[Type], params: &[GenericParam]) -> Type {
        // Simple substitution for now
        let mut subst: HashMap<Symbol, Type> = HashMap::new();
        for (p, a) in params.iter().zip(args.iter()) {
            subst.insert(p.name, a.clone());
        }
        self.apply_subst(base_ty, &subst)
    }

    fn apply_subst(&self, ty: &Type, subst: &HashMap<Symbol, Type>) -> Type {
        match ty {
            Type::Named(name) | Type::Parameter(name) => {
                if let Some(t) = subst.get(name) {
                    t.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Array(inner) => Type::Array(Box::new(self.apply_subst(inner, subst))),
            Type::Tuple(elems) => Type::Tuple(elems.iter().map(|e| self.apply_subst(e, subst)).collect()),
            Type::Func(params, ret) => Type::Func(
                params.iter().map(|p| self.apply_subst(p, subst)).collect(),
                Box::new(self.apply_subst(ret, subst)),
            ),
            Type::Optional(inner) => Type::Optional(Box::new(self.apply_subst(inner, subst))),
            _ => ty.clone(),
        }
    }
}
