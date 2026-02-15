use crate::sema::{Sema, Type};
use crate::diag::Span;

pub fn resolve_extension_method(sema: &Sema, receiver_ty: &Type, method_name: &str) -> Option<Type> {
    if let Some(methods) = sema.extensions.get(method_name) {
        for (recv, sig) in methods {
            if recv == receiver_ty {
                return Some(sig.clone());
            }
        }
    }
    None
}

pub fn check_extension_dispatch(sema: &mut Sema, receiver_ty: &Type, method_name: &str, span: Span) -> Type {
    if let Some(sig) = resolve_extension_method(sema, receiver_ty, method_name) {
        sig
    } else {
        sema.report_error(format!("no extension method '{}' found for type {:?}", method_name, receiver_ty), span);
        Type::Unknown
    }
}
