use crate::ast::{Expr, Literal};

pub struct ConstexprEval;

impl ConstexprEval {
    pub fn eval_expr(expr: &Expr) -> Option<Literal> {
        match expr {
            Expr::Literal(lit, _) => Some(lit.clone()),
            _ => None, // TODO: Implement full expression evaluation
        }
    }
}
