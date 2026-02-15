use crate::sema::{Type, Sema};
use crate::diag::{Diagnostic, Span};

pub fn check_nullability(sema: &mut Sema, expected: &Type, actual: &Type, span: Span) {
    if let Type::Optional(_) = actual {
        if !matches!(expected, Type::Optional(_) | Type::Any | Type::Unknown) {
            sema.report_error(format!("cannot use nullable value of type {:?} where {:?} is expected", actual, expected), span);
        }
    }
}

pub fn check_extension_member(sema: &mut Sema, receiver_ty: &Type, name: &str, span: Span) -> Type {
    crate::extension::check_extension_dispatch(sema, receiver_ty, name, span)
}

impl Sema {
    pub fn report_error(&mut self, msg: impl Into<String>, span: Span) {
        self.diags.push(Diagnostic::error(msg, span));
    }
}
