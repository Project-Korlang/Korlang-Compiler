use crate::ast::*;
use crate::sema::{Sema, Type};
use crate::diag::{Diagnostic, Span};

pub struct AsyncCompiler<'a> {
    sema: &'a mut Sema,
}

impl<'a> AsyncCompiler<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self { sema }
    }

    pub fn check_async_fun(&mut self, f: &FunDecl) {
        if f.is_async {
            // Transform the function into a state machine
            // For now, we just validate that it returns a Result or some awaitable type
            if let Some(ret) = &f.ret {
                let ret_ty = self.sema.type_from_ref(ret);
                // In Korlang, async functions return a Future<T> implicitly
                // The sema should wrap the return type in Future
            }
        }
    }

    pub fn check_await_expr(&mut self, expr: &Expr, span: Span) -> Type {
        // Await can only be used inside async functions
        // The expression being awaited must be a Future/Coroutine
        let ty = self.sema.check_expr_with(expr, false);
        // Simplified: check if ty is generic type 'Future' or 'Result'
        match ty {
            Type::Generic(name, args) if name == "Future" || name == "Result" => {
                args.get(0).cloned().unwrap_or(Type::Unit)
            }
            _ => {
                self.sema.report_error("can only await expressions of type Future<T> or Result<T, E>", span);
                Type::Unknown
            }
        }
    }
}
