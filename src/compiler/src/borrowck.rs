use crate::ast::*;
use crate::sema::{Type, Sema};
use crate::diag::{Diagnostic, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    Owned,
    Borrowed,
    MutBorrowed,
    Moved,
}

pub struct BorrowChecker<'a> {
    sema: &'a mut Sema,
    locals: HashMap<String, OwnershipState>,
}

impl<'a> BorrowChecker<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self {
            sema,
            locals: HashMap::new(),
        }
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var(v) => {
                self.check_expr(&v.value);
                self.locals.insert(v.name.clone(), OwnershipState::Owned);
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

    pub fn check_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.check_expr(tail);
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name, span) => {
                if let Some(state) = self.locals.get(name) {
                    if *state == OwnershipState::Moved {
                        self.sema.diags.push(Diagnostic::error(
                            format!("use of moved value: '{}'", name),
                            *span,
                        ));
                    }
                }
            }
            Expr::Assign { left, right, .. } => {
                self.check_expr(right);
                if let Expr::Ident(name, _) = &**left {
                    // Re-owning a variable
                    self.locals.insert(name.clone(), OwnershipState::Owned);
                }
            }
            Expr::Call { callee, args, .. } => {
                self.check_expr(callee);
                for arg in args {
                    self.check_expr(arg);
                    // Simple move semantics: passing an ident to a function moves it
                    // unless we implement borrowing syntax later
                    if let Expr::Ident(name, _) = arg {
                        self.locals.insert(name.clone(), OwnershipState::Moved);
                    }
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::Unary { expr, .. } => {
                self.check_expr(expr);
            }
            Expr::If { cond, then_block, else_block, .. } => {
                self.check_expr(cond);
                self.check_block(then_block);
                self.check_block(else_block);
            }
            _ => {}
        }
    }
}
