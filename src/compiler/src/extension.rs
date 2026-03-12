use crate::sema::{Sema, Type};
use crate::diag::Span;

pub fn resolve_extension_method(sema: &Sema, receiver_ty: &Type, method_name: &crate::symbols::Symbol) -> Option<Type> {
    if let Some(methods) = sema.extensions.get(method_name) {
        for (recv, sig) in methods {
            if recv == receiver_ty {
                return Some(sig.clone());
            }
        }
    }
    None
}

pub fn check_extension_dispatch(sema: &mut Sema, receiver_ty: &Type, method_name: crate::symbols::Symbol, span: Span) -> Type {
    if let Some(sig) = resolve_extension_method(sema, receiver_ty, &method_name) {
        sig
    } else {
        let name_s = crate::symbols::lookup(method_name);
        sema.diags.push(crate::diag::Diagnostic::error(format!("no extension method '{}' found for type {:?}", name_s, receiver_ty), span));
        Type::Unknown
    }
}
