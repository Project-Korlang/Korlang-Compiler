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
        }
    }
}
