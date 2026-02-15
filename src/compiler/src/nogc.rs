use crate::ast::*;
use crate::sema::{Sema, Type};
use crate::diag::{Diagnostic, Span};

pub struct NoGcChecker<'a> {
    sema: &'a mut Sema,
}

impl<'a> NoGcChecker<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self { sema }
    }

    pub fn check_fun(&mut self, f: &FunDecl) {
        if f.nogc {
            self.check_block(&f.body);
        }
    }

    fn check_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.check_expr(tail);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var(v) => {
                self.check_expr(&v.value);
            }
            Stmt::Expr(e, _) => {
                self.check_expr(e);
            }
            Stmt::If(cond, then_block, else_stmt, _) => {
                self.check_expr(cond);
                self.check_block(then_block);
                if let Some(else_s) = else_stmt {
                    self.check_stmt(else_s);
                }
            }
            Stmt::While(cond, body, _) => {
                self.check_expr(cond);
                self.check_block(body);
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Array(_, span) => {
                self.sema.diags.push(Diagnostic::error("allocation not allowed in @nogc context", *span));
            }
            Expr::StructLit { span, .. } => {
                // Struct literals are okay if they are on stack, but currently, we treat them as managed
                // if they are not explicitly handled.
                // For now, let's flag all managed allocations.
                self.sema.diags.push(Diagnostic::error("managed allocation not allowed in @nogc context", *span));
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Ident(name, _) = &**callee {
                    if !self.sema.is_nogc_function(name) {
                        self.sema.diags.push(Diagnostic::error(format!("call to non-@nogc function '{}' from @nogc context", name), *span));
                    }
                }
                for arg in args {
                    self.check_expr(arg);
                }
            }
            _ => {
                // Recurse for nested expressions
            }
        }
    }
}
