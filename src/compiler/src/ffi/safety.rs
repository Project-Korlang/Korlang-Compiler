use crate::diag::{Diagnostic, Span};
use crate::sema::Type;

pub struct FfiSafetyChecker;

impl FfiSafetyChecker {
    /// Validates if a type is safe to pass through an FFI boundary
    pub fn check_type_safety(ty: &Type, span: Span) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        match ty {
            Type::Named(n) if n == "String" => {
                diags.push(Diagnostic::warning(
                    "Passing Korlang String to FFI might require manual memory management on the other side.",
                    span
                ));
            }
            Type::Func(_, _) => {
                diags.push(Diagnostic::warning(
                    "Passing closures across FFI boundary requires C-compatible function pointers.",
                    span
                ));
            }
            _ => {}
        }
        diags
    }

    /// Injects safety checks for pointer arithmetic or out-of-bounds access if necessary
    pub fn verify_pointer_access(ptr_name: &str) -> String {
        format!("if ({} == nullptr) {{ throw std::runtime_error(\"FFI Safety Violation: Null pointer access\"); }}", ptr_name)
    }
}
