use crate::ast::Pattern;
use crate::sema::{Type, Sema};

pub struct PatternChecker<'a> {
    sema: &'a mut Sema,
}

impl<'a> PatternChecker<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self { sema }
    }

    pub fn check_pattern(&mut self, pat: &Pattern, expected_ty: &Type) {
        match pat {
            Pattern::Literal(lit, span) => {
                let lit_ty = self.sema.type_of_literal(lit);
                self.sema.unify(expected_ty, &lit_ty, *span);
            }
            Pattern::Ident(name, span) => {
                self.sema.define_var(name, expected_ty.clone(), *span);
            }
            Pattern::Wildcard(_) => {}
            Pattern::Tuple(parts, span) => {
                if let Type::Tuple(expected_parts) = expected_ty {
                    if parts.len() != expected_parts.len() {
                        self.sema.report_error(format!("tuple pattern length mismatch: expected {}, got {}", expected_parts.len(), parts.len()), *span);
                    } else {
                        for (p, et) in parts.iter().zip(expected_parts.iter()) {
                            self.check_pattern(p, et);
                        }
                    }
                } else {
                    self.sema.report_error(format!("expected tuple type for tuple pattern, got {:?}", expected_ty), *span);
                }
            }
            Pattern::Variant { name: _name, args, span: _span } => {
                // Simplified variant check: assume it belongs to an enum
                // In a real implementation, we would look up the enum definition
                for arg in args {
                    self.check_pattern(arg, &Type::Any);
                }
            }
            Pattern::Struct { name: _name, fields, span: _span } => {
                // Simplified struct check
                for (_, p) in fields {
                    self.check_pattern(p, &Type::Any);
                }
            }
            Pattern::Is(ty_ref, inner_pat, span) => {
                let target_ty = self.sema.type_from_ref(ty_ref);
                // check if expected_ty can be cast to target_ty
                // for simplicity, we assume success if they are named types or related
                self.check_pattern(inner_pat, &target_ty);
            }
        }
    }
    pub fn check_exhaustiveness(&mut self, matched_types: &[Type], sealed_name: &str, span: Span) {
        let enforcer = crate::sealed::SealedEnforcer::new(self.sema);
        let expected = enforcer.get_exhaustiveness_info(sealed_name);
        let mut matched_names = std::collections::HashSet::new();
        for ty in matched_types {
            if let Type::Named(name) = ty {
                matched_names.insert(name.clone());
            }
        }
        for name in expected {
            if !matched_names.contains(&name) {
                self.sema.report_error(format!("match is not exhaustive: missing variant '{}'", name), span);
            }
        }
    }
}
