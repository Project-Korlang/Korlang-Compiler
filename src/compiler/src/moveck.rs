use crate::ast::*;
use crate::sema::{Type, Sema};
use crate::diag::{Diagnostic, Span};
use std::collections::HashSet;

pub struct MoveChecker<'a> {
    sema: &'a mut Sema,
    moved_locals: HashSet<String>,
}

impl<'a> MoveChecker<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self {
            sema,
            moved_locals: HashSet::new(),
        }
    }

    pub fn check_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.check_expr(tail);
        }
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var(v) => {
                self.check_expr(&v.value);
            }
            Stmt::Expr(e, _) => {
                self.check_expr(e);
            }
            Stmt::Return(e, _) => {
                if let Some(expr) = e {
                    self.check_expr(expr);
                }
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
            Stmt::Block(b) => {
                self.check_block(b);
            }
            _ => {}
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name, span) => {
                if self.moved_locals.contains(name) {
                    self.sema.diags.push(Diagnostic::error(
                        format!("value '{}' used here after move", name),
                        *span,
                    ));
                }
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    // Check before marking as moved
                    self.check_expr(arg);
                    
                    if let Expr::Ident(name, _) = arg {
                        // Mark as moved AFTER checking it
                        self.moved_locals.insert(name.clone());
                    }
                }
            }
            Expr::Assign { left, right, .. } => {
                self.check_expr(right);
                if let Expr::Ident(name, _) = &**left {
                    // Re-assignment makes it available again
                    self.moved_locals.remove(name);
                }
            }
            _ => {}
        }
    }
}
