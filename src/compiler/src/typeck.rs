use crate::sema::{Type, Sema};
use crate::diag::{Diagnostic, Span};

pub fn check_nullability(sema: &mut Sema, expected: &Type, actual: &Type, span: Span) {
    if let Type::Optional(_) = actual {
        if !matches!(expected, Type::Optional(_) | Type::Any | Type::Unknown) {
            sema.report_error(format!("cannot use nullable value of type {:?} where {:?} is expected", actual, expected), span);
        }
    }
}

impl Sema {
    pub fn report_error(&mut self, msg: impl Into<String>, span: Span) {
        self.diags.push(Diagnostic::error(msg, span));
    }
}
