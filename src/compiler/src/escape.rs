use crate::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct EscapeResult {
    pub escapes: HashSet<String>,
}

pub fn analyze_escape(program: &Program) -> HashMap<String, EscapeResult> {
    let mut map = HashMap::new();
    for item in &program.items {
        if let Item::Fun(f) = item {
            map.insert(f.name.clone(), analyze_fun(f));
        }
    }
    map
}

fn analyze_fun(fun: &FunDecl) -> EscapeResult {
    let mut res = EscapeResult::default();
    let mut locals = HashSet::new();
    collect_locals(&fun.body, &mut locals);
    mark_escapes_in_block(&fun.body, &mut res, &locals);
    res
}

fn collect_locals(block: &Block, locals: &mut HashSet<String>) {
    for stmt in &block.stmts {
        if let Stmt::Var(v) = stmt {
            locals.insert(v.name.clone());
        }
        if let Stmt::Block(b) = stmt {
            collect_locals(b, locals);
        }
        if let Stmt::If(_, b, o, _) = stmt {
            collect_locals(b, locals);
            if let Some(s) = o {
                if let Stmt::Block(b) = &**s {
                    collect_locals(b, locals);
                }
            }
        }
        if let Stmt::While(_, b, _) = stmt {
            collect_locals(b, locals);
        }
        if let Stmt::For(_, _, b, _) = stmt {
            collect_locals(b, locals);
        }
        if let Stmt::Match(_, arms, _) = stmt {
            for arm in arms {
                if let Expr::Block(b) = &arm.body {
                    collect_locals(b, locals);
                }
            }
        }
    }
}

fn mark_escapes_in_block(block: &Block, res: &mut EscapeResult, locals: &HashSet<String>) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Return(Some(expr), _) => mark_escapes(expr, res, locals, true),
            Stmt::Var(v) => mark_escapes(&v.value, res, locals, false),
            Stmt::Expr(e, _) => mark_escapes(e, res, locals, false),
            Stmt::If(cond, b, o, _) => {
                mark_escapes(cond, res, locals, false);
                mark_escapes_in_block(b, res, locals);
                if let Some(s) = o {
                    if let Stmt::Block(b) = &**s {
                        mark_escapes_in_block(b, res, locals);
                    }
                }
            }
            Stmt::While(cond, b, _) => {
                mark_escapes(cond, res, locals, false);
                mark_escapes_in_block(b, res, locals);
            }
            Stmt::For(_, iter, b, _) => {
                mark_escapes(iter, res, locals, false);
                mark_escapes_in_block(b, res, locals);
            }
            Stmt::Match(e, arms, _) => {
                mark_escapes(e, res, locals, false);
                for arm in arms {
                    mark_escapes(&arm.body, res, locals, false);
                }
            }
            Stmt::Block(b) => mark_escapes_in_block(b, res, locals),
            _ => {}
        }
    }
    if let Some(tail) = &block.tail {
        mark_escapes(tail, res, locals, false);
    }
}

fn mark_escapes(expr: &Expr, res: &mut EscapeResult, locals: &HashSet<String>, should_mark: bool) {
    match expr {
        Expr::Ident(name, _) => {
            if should_mark && locals.contains(name) {
                res.escapes.insert(name.clone());
            }
        }
        Expr::Call { callee, args, .. } => {
            mark_escapes(callee, res, locals, false);
            for a in args {
                mark_escapes(a, res, locals, true);
            }
        }
        Expr::StructLit { fields, .. } => {
            for (_, value) in fields {
                mark_escapes(value, res, locals, should_mark);
            }
        }
        Expr::Member { target, .. } => mark_escapes(target, res, locals, should_mark),
        Expr::Index { target, index, .. } => {
            mark_escapes(target, res, locals, should_mark);
            mark_escapes(index, res, locals, should_mark);
        }
        Expr::Unary { expr, .. } => mark_escapes(expr, res, locals, should_mark),
        Expr::Binary { left, right, .. } => {
            mark_escapes(left, res, locals, should_mark);
            mark_escapes(right, res, locals, should_mark);
        }
        Expr::Assign { left, right, .. } => {
            mark_escapes(left, res, locals, should_mark);
            mark_escapes(right, res, locals, should_mark);
        }
        Expr::If { cond, then_block, else_block, .. } => {
            mark_escapes(cond, res, locals, should_mark);
            mark_escapes_in_block(then_block, res, locals);
            mark_escapes_in_block(else_block, res, locals);
        }
        Expr::Match { expr, arms, .. } => {
            mark_escapes(expr, res, locals, should_mark);
            for arm in arms {
                mark_escapes(&arm.body, res, locals, should_mark);
            }
        }
        Expr::Block(b) => mark_escapes_in_block(b, res, locals),
        Expr::Array(items, _) => {
            for it in items {
                mark_escapes(it, res, locals, true);
            }
        }
        Expr::Tensor(rows, _) => {
            for row in rows {
                for it in row {
                    mark_escapes(it, res, locals, true);
                }
            }
        }
        Expr::Interpolated { parts, .. } => {
            for p in parts {
                mark_escapes(p, res, locals, should_mark);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn analyze(src: &str) -> EscapeResult {
        let tokens = Lexer::new(src).tokenize().unwrap();
        let program = Parser::new(tokens).parse_program().unwrap();
        analyze_escape(&program).values().next().cloned().unwrap_or_default()
    }

    #[test]
    fn escape_return_marks_local() {
        let res = analyze("fun f() { let x = 1; return x; }");
        assert!(res.escapes.contains("x"));
    }

    #[test]
    fn escape_pass_to_call() {
        let res = analyze("fun f() { let x = 1; foo(x); }");
        assert!(res.escapes.contains("x"));
    }

    #[test]
    fn escape_in_array_literal() {
        let res = analyze("fun f() { let x = 1; let a = [x]; }");
        assert!(res.escapes.contains("x"));
    }

    #[test]
    fn no_escape_simple_use() {
        let res = analyze("fun f() { let x = 1; let y = x + 2; }");
        assert!(!res.escapes.contains("x"));
    }
}
