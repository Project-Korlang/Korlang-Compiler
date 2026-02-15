use crate::ast::*;
use crate::sema::{Type, Sema};
use crate::diag::{Diagnostic, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lifetime {
    Static,
    Block(usize), // depth
    Param(String),
}

pub struct LifetimeChecker<'a> {
    sema: &'a mut Sema,
    current_depth: usize,
    bindings: HashMap<String, Lifetime>,
}

impl<'a> LifetimeChecker<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self {
            sema,
            current_depth: 0,
            bindings: HashMap::new(),
        }
    }

    pub fn enter_block(&mut self) {
        self.current_depth += 1;
    }

    pub fn exit_block(&mut self) {
        self.current_depth -= 1;
    }

    pub fn bind(&mut self, name: String, lifetime: Lifetime) {
        self.bindings.insert(name, lifetime);
    }

    pub fn check_block(&mut self, block: &Block) {
        self.enter_block();
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        if let Some(tail) = &block.tail {
            self.check_expr(tail);
        }
        self.exit_block();
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var(v) => {
                let life = self.check_expr(&v.value);
                self.bind(v.name.clone(), life);
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

    pub fn check_expr(&mut self, expr: &Expr) -> Lifetime {
        match expr {
            Expr::Literal(_, _) => Lifetime::Static,
            Expr::Ident(name, _) => {
                self.bindings.get(name).cloned().unwrap_or(Lifetime::Block(self.current_depth))
            }
            Expr::Call { args, .. } => {
                // Return minimum lifetime of arguments for now
                let min_life = Lifetime::Static;
                for arg in args {
                    let _life = self.check_expr(arg);
                    // logic to find shortest lifetime...
                }
                min_life
            }
            _ => Lifetime::Block(self.current_depth),
        }
    }

    pub fn validate_assignment(&mut self, target_life: &Lifetime, value_life: &Lifetime, span: Span) {
        // value must outlive target
        match (target_life, value_life) {
            (Lifetime::Static, Lifetime::Static) => {}
            (Lifetime::Static, _) => {
                self.sema.diags.push(Diagnostic::error("assigning short-lived value to static target", span));
            }
            (Lifetime::Block(td), Lifetime::Block(vd)) => {
                if vd > td {
                    self.sema.diags.push(Diagnostic::error("value does not live long enough", span));
                }
            }
            _ => {}
        }
    }
}
