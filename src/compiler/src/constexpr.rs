use crate::ast::*;
use crate::sema::Type;

pub struct ConstexprEval;

impl ConstexprEval {
    pub fn eval_expr(expr: &Expr) -> Option<Literal> {
        match expr {
            Expr::Literal(lit, _) => Some(lit.clone()),
            _ => None, // TODO: Implement full expression evaluation
        }
    }
}
