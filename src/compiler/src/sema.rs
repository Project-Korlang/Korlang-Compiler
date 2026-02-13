use crate::ast::*;
use crate::diag::{Diagnostic, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    UInt,
    Float,
    Bool,
    Char,
    String,
    Unit,
    Any,
    Nothing,
    Tuple(Vec<Type>),
    Array(Box<Type>),
    Tensor(Box<Type>),
    Named(String),
    Func(Vec<Type>, Box<Type>),
    Unknown,
}

#[derive(Default)]
struct Scope {
    vars: HashMap<String, Type>,
}

pub struct Sema {
    scopes: Vec<Scope>,
    diags: Vec<Diagnostic>,
    functions: HashMap<String, Type>,
    nogc_functions: HashMap<String, bool>,
    permissive: bool,
}

impl Sema {
    pub fn new() -> Self {
        let mut s = Self {
            scopes: Vec::new(),
            diags: Vec::new(),
            functions: HashMap::new(),
            nogc_functions: HashMap::new(),
            permissive: std::env::var("KORLANG_SEMA_PERMISSIVE").ok().as_deref() == Some("1"),
        };
        s.push_scope();
        // Predeclare builtins used by the self-hosted compiler.
        s.define_builtin("null", Type::Any);
        s.define_builtin("List", Type::Named("List".to_string()));
        s.define_builtin("Result", Type::Named("Result".to_string()));
        s.define_builtin("@import", Type::Func(vec![Type::String], Box::new(Type::Any)));
        s.define_builtin("print", Type::Func(vec![Type::Any], Box::new(Type::Unit)));
        s.define_builtin("println", Type::Func(vec![Type::Any], Box::new(Type::Unit)));
        s.define_builtin("readLine", Type::Func(vec![], Box::new(Type::String)));
        s
    }

    pub fn check_program(mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        // Predeclare types (structs/enums/aliases) so references before definition work.
        for item in &program.items {
            match item {
                Item::Struct(s) => self.define_builtin(&s.name, Type::Named(s.name.clone())),
                Item::Enum(e) => self.define_builtin(&e.name, Type::Named(e.name.clone())),
                Item::TypeAlias(t) => self.define_builtin(&t.name, Type::Named(t.name.clone())),
                _ => {}
            }
        }
        for item in &program.items {
            if let Item::Fun(f) = item {
                let sig = self.fun_sig(f);
                self.functions.insert(f.name.clone(), sig);
                self.nogc_functions.insert(f.name.clone(), f.nogc);
            }
        }

        for item in &program.items {
            self.check_item(item);
        }

        if self.diags.is_empty() {
            Ok(())
        } else {
            Err(self.diags)
        }
    }

    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Fun(f) => self.check_fun(f),
            Item::Struct(_) => {}
            Item::Enum(_) => {}
            Item::TypeAlias(_) => {}
            Item::View(_) => {}
            Item::Resource(_) => {}
            Item::Const(v) => {
                let ty = self.check_expr(&v.value);
                if let Some(ann) = &v.ty {
                    let ann_ty = self.type_from_ref(ann);
                    self.unify(&ann_ty, &ty, v.span);
                }
                self.define_var(&v.name, ty, v.span);
            }
            Item::Stmt(s) => {
                self.check_stmt(s);
            }
        }
    }

    fn check_fun(&mut self, fun: &FunDecl) {
        self.push_scope();
        for p in &fun.params {
            let t = self.type_from_ref(&p.ty);
            self.define_var(&p.name, t, p.span);
        }
        let body_ty = self.check_block_with(&fun.body, fun.nogc);
        if let Some(ret) = &fun.ret {
            let ret_ty = self.type_from_ref(ret);
            if fun.name == "main" && ret_ty == Type::Int {
                // Allow main to return any type for now; codegen will default to 0.
            } else {
                self.unify(&ret_ty, &body_ty, fun.span);
            }
        }
        self.pop_scope();
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Type {
        self.check_stmt_with(stmt, false)
    }

    fn check_stmt_with(&mut self, stmt: &Stmt, nogc: bool) -> Type {
        match stmt {
            Stmt::Var(v) => {
                let ty = self.check_expr_with(&v.value, nogc);
                if let Some(ann) = &v.ty {
                    let ann_ty = self.type_from_ref(ann);
                    self.unify(&ann_ty, &ty, v.span);
                }
                self.define_var(&v.name, ty.clone(), v.span);
                Type::Unit
            }
            Stmt::Expr(e, _) => {
                self.check_expr_with(e, nogc);
                Type::Unit
            }
            Stmt::Return(expr, _) => {
                if let Some(e) = expr {
                    self.check_expr_with(e, nogc)
                } else {
                    Type::Unit
                }
            }
            Stmt::Break(_) | Stmt::Continue(_) => Type::Nothing,
            Stmt::If(cond, then_block, else_stmt, _) => {
                let cond_ty = self.check_expr_with(cond, nogc);
                self.unify(&Type::Bool, &cond_ty, self.span_of(cond));
                let then_ty = self.check_block_with(then_block, nogc);
                let else_ty = if let Some(stmt) = else_stmt {
                    match &**stmt {
                        Stmt::Block(b) => self.check_block_with(b, nogc),
                        Stmt::If(_, _, _, _) => self.check_stmt_with(stmt, nogc),
                        _ => Type::Unit,
                    }
                } else {
                    Type::Unit
                };
                self.join_types(then_ty, else_ty)
            }
            Stmt::While(cond, body, _) => {
                let cond_ty = self.check_expr_with(cond, nogc);
                self.unify(&Type::Bool, &cond_ty, self.span_of(cond));
                self.check_block_with(body, nogc);
                Type::Unit
            }
            Stmt::For(name, iter, body, span) => {
                let iter_ty = self.check_expr_with(iter, nogc);
                let elem = match iter_ty {
                    Type::Array(t) => *t,
                    _ => {
                        self.diags.push(Diagnostic::new("for-in expects array", *span));
                        Type::Unknown
                    }
                };
                self.push_scope();
                self.define_var(name, elem, *span);
                self.check_block_with(body, nogc);
                self.pop_scope();
                Type::Unit
            }
            Stmt::Match(expr, arms, _) => {
                let _ = self.check_expr_with(expr, nogc);
                let mut ty = Type::Unknown;
                for arm in arms {
                    self.push_scope();
                    self.bind_pattern(&arm.pat);
                    let arm_ty = self.check_expr_with(&arm.body, nogc);
                    self.pop_scope();
                    ty = if ty == Type::Unknown { arm_ty } else { self.join_types(ty, arm_ty) };
                }
                ty
            }
            Stmt::Block(b) => self.check_block_with(b, nogc),
        }
    }

    fn check_block(&mut self, block: &Block) -> Type {
        self.check_block_with(block, false)
    }

    fn check_block_with(&mut self, block: &Block, nogc: bool) -> Type {
        self.push_scope();
        for s in &block.stmts {
            self.check_stmt_with(s, nogc);
        }
        let ty = if let Some(tail) = &block.tail {
            self.check_expr_with(tail, nogc)
        } else {
            Type::Unit
        };
        self.pop_scope();
        ty
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        self.check_expr_with(expr, false)
    }

    fn check_expr_with(&mut self, expr: &Expr, nogc: bool) -> Type {
        match expr {
            Expr::Literal(l, span) => {
                if nogc {
                    if matches!(l, Literal::String(_)) {
                        self.diags.push(Diagnostic::new("allocation not allowed in @nogc", *span));
                    }
                }
                self.type_of_literal(l)
            }
            Expr::Ident(name, span) => self.lookup_var(name, *span),
            Expr::StructLit { name, fields, .. } => {
                for (_, value) in fields {
                    self.check_expr_with(value, nogc);
                }
                Type::Named(name.clone())
            }
            Expr::Unary { op, expr, span } => {
                let t = self.check_expr_with(expr, nogc);
                match op {
                    UnaryOp::Not => {
                        self.unify(&Type::Bool, &t, *span);
                        Type::Bool
                    }
                    UnaryOp::Neg | UnaryOp::Pos => self.expect_number(t, *span),
                    UnaryOp::BitNot => self.expect_int(t, *span),
                }
            }
            Expr::Binary { left, op, right, span } => {
                let lt = self.check_expr_with(left, nogc);
                let rt = self.check_expr_with(right, nogc);
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        self.expect_number(lt.clone(), *span);
                        self.expect_number(rt.clone(), *span);
                        match (lt, rt) {
                            (Type::Int, Type::Int) => Type::Int,
                            (Type::UInt, Type::UInt) => Type::UInt,
                            _ => Type::Float,
                        }
                    }
                    BinaryOp::DotAdd | BinaryOp::DotSub | BinaryOp::DotMul | BinaryOp::DotDiv | BinaryOp::MatMul => {
                        Type::Tensor(Box::new(Type::Float))
                    }
                    BinaryOp::Eq | BinaryOp::NotEq | BinaryOp::Lt | BinaryOp::LtEq | BinaryOp::Gt | BinaryOp::GtEq => {
                        Type::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        self.unify(&Type::Bool, &lt, *span);
                        self.unify(&Type::Bool, &rt, *span);
                        Type::Bool
                    }
                    BinaryOp::NullCoalesce => self.join_types(lt, rt),
                    BinaryOp::Pipe | BinaryOp::Arrow => rt,
                }
            }
            Expr::Assign { left, right, span, .. } => {
                let lt = self.check_expr_with(left, nogc);
                let rt = self.check_expr_with(right, nogc);
                self.unify(&lt, &rt, *span);
                lt
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Ident(name, _) = &**callee {
                    if name == "@import" || name == "@bridge" {
                        if args.is_empty() {
                            self.diags.push(Diagnostic::new("FFI call requires a string argument", *span));
                        } else if !matches!(args[0], Expr::Literal(Literal::String(_), _)) {
                            self.diags.push(Diagnostic::new("FFI call requires string literal", self.span_of(&args[0])));
                        }
                    }
                    if nogc && !self.is_nogc_function(name) {
                        self.diags.push(Diagnostic::new("call to non-@nogc function in @nogc", *span));
                    }
                }
                if matches!(**callee, Expr::Member { .. }) {
                    // Allow method-like calls in the self-hosted compiler without strict typing.
                    for arg in args {
                        let _ = self.check_expr_with(arg, nogc);
                    }
                    return Type::Unknown;
                }
                let ct = self.check_expr_with(callee, nogc);
                match ct {
                    Type::Func(params, ret) => {
                        if params.len() != args.len() {
                            self.diags.push(Diagnostic::new("argument count mismatch", *span));
                        }
                        for (arg, p) in args.iter().zip(params.iter()) {
                            let at = self.check_expr_with(arg, nogc);
                            self.unify(p, &at, self.span_of(arg));
                        }
                        *ret
                    }
                    Type::Unknown | Type::Any => Type::Unknown,
                    _ => {
                        self.diags.push(Diagnostic::new("call to non-function", *span));
                        Type::Unknown
                    }
                }
            }
            Expr::Member { target, .. } => {
                let _ = self.check_expr_with(target, nogc);
                Type::Unknown
            }
            Expr::Index { target, index, span } => {
                let t = self.check_expr_with(target, nogc);
                let _ = self.check_expr_with(index, nogc);
                match t {
                    Type::Array(inner) => *inner,
                    Type::Named(name) if name == "List" => Type::Unknown,
                    Type::Unknown | Type::Any => Type::Unknown,
                    _ => {
                        self.diags.push(Diagnostic::new("indexing non-array", *span));
                        Type::Unknown
                    }
                }
            }
            Expr::If { cond, then_block, else_block, span } => {
                let ct = self.check_expr_with(cond, nogc);
                self.unify(&Type::Bool, &ct, self.span_of(cond));
                let tt = self.check_block_with(then_block, nogc);
                let et = self.check_block_with(else_block, nogc);
                self.join_types(tt, et)
            }
            Expr::Match { expr, arms, span } => {
                let _ = self.check_expr_with(expr, nogc);
                let mut ty = Type::Unknown;
                for arm in arms {
                    self.push_scope();
                    self.bind_pattern(&arm.pat);
                    let at = self.check_expr_with(&arm.body, nogc);
                    self.pop_scope();
                    ty = if ty == Type::Unknown { at } else { self.join_types(ty, at) };
                }
                ty
            }
            Expr::Block(b) => self.check_block_with(b, nogc),
            Expr::Array(items, _) => {
                if nogc {
                    self.diags.push(Diagnostic::new("allocation not allowed in @nogc", self.span_of(expr)));
                }
                let mut ty = Type::Unknown;
                for it in items {
                    let t = self.check_expr_with(it, nogc);
                    ty = if ty == Type::Unknown { t } else { self.join_types(ty, t) };
                }
                Type::Array(Box::new(ty))
            }
            Expr::Tensor(_, _) => {
                if nogc {
                    self.diags.push(Diagnostic::new("allocation not allowed in @nogc", self.span_of(expr)));
                }
                Type::Tensor(Box::new(Type::Float))
            }
            Expr::Interpolated { parts, .. } => {
                if nogc {
                    self.diags.push(Diagnostic::new("allocation not allowed in @nogc", self.span_of(expr)));
                }
                for p in parts {
                    self.check_expr_with(p, nogc);
                }
                Type::String
            }
        }
    }

    fn define_builtin(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.vars.insert(name.to_string(), ty);
        }
    }

    fn bind_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Ident(name, span) => {
                self.define_var(name, Type::Unknown, *span);
            }
            Pattern::Tuple(parts, _) => {
                for p in parts {
                    self.bind_pattern(p);
                }
            }
            Pattern::Variant { args, .. } => {
                for p in args {
                    self.bind_pattern(p);
                }
            }
            Pattern::Struct { fields, .. } => {
                for f in fields {
                    self.bind_pattern(&f.1);
                }
            }
            Pattern::Literal(_, _) | Pattern::Wildcard(_) => {}
        }
    }

    fn type_of_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Char(_) => Type::Char,
            Literal::Bool(_) => Type::Bool,
        }
    }

    fn type_from_ref(&self, tr: &TypeRef) -> Type {
        match tr {
            TypeRef::Named(name, _) => match name.as_str() {
                "Int" => Type::Int,
                "UInt" => Type::UInt,
                "Float" => Type::Float,
                "Bool" => Type::Bool,
                "Char" => Type::Char,
                "String" => Type::String,
                "Any" => Type::Any,
                "Nothing" => Type::Nothing,
                _ => Type::Named(name.clone()),
            },
            TypeRef::Tuple(elems, _) => Type::Tuple(elems.iter().map(|e| self.type_from_ref(e)).collect()),
            TypeRef::Array(inner, _) => Type::Array(Box::new(self.type_from_ref(inner))),
            TypeRef::Tensor { elem, .. } => Type::Tensor(Box::new(self.type_from_ref(elem))),
            TypeRef::Optional(inner, _) => self.type_from_ref(inner),
            TypeRef::NonNull(inner, _) => self.type_from_ref(inner),
        }
    }

    fn fun_sig(&self, f: &FunDecl) -> Type {
        let params = f.params.iter().map(|p| self.type_from_ref(&p.ty)).collect();
        let ret = f.ret.as_ref().map(|t| self.type_from_ref(t)).unwrap_or(Type::Unit);
        Type::Func(params, Box::new(ret))
    }

    fn lookup_var(&mut self, name: &str, span: Span) -> Type {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.vars.get(name) {
                return t.clone();
            }
        }
        if let Some(t) = self.functions.get(name) {
            return t.clone();
        }
        self.diags.push(Diagnostic::new(format!("undefined symbol '{name}'"), span));
        Type::Unknown
    }

    fn define_var(&mut self, name: &str, ty: Type, span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.vars.contains_key(name) {
                self.diags.push(Diagnostic::new(format!("redefinition of '{name}'"), span));
            } else {
                scope.vars.insert(name.to_string(), ty);
            }
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn unify(&mut self, expected: &Type, actual: &Type, span: Span) {
        if self.permissive {
            return;
        }
        if matches!(expected, Type::Unknown | Type::Any) || matches!(actual, Type::Unknown | Type::Any) {
            return;
        }
        if expected != actual {
            self.diags.push(Diagnostic::new(
                format!("type mismatch: expected {:?}, got {:?}", expected, actual),
                span,
            ));
        }
    }

    fn join_types(&self, a: Type, b: Type) -> Type {
        if a == b {
            a
        } else if a == Type::Nothing {
            b
        } else if b == Type::Nothing {
            a
        } else {
            Type::Any
        }
    }

    fn expect_number(&mut self, t: Type, span: Span) -> Type {
        if self.permissive {
            return Type::Unknown;
        }
        match t {
            Type::Int | Type::UInt | Type::Float => Type::Float,
            Type::Unknown | Type::Any => Type::Unknown,
            _ => {
                self.diags.push(Diagnostic::new("expected numeric type", span));
                Type::Unknown
            }
        }
    }

    fn expect_int(&mut self, t: Type, span: Span) -> Type {
        if self.permissive {
            return Type::Unknown;
        }
        match t {
            Type::Int | Type::UInt => t,
            Type::Unknown | Type::Any => Type::Unknown,
            _ => {
                self.diags.push(Diagnostic::new("expected integer type", span));
                Type::Unknown
            }
        }
    }

    fn span_of(&self, expr: &Expr) -> Span {
        match expr {
            Expr::Literal(_, s) => *s,
            Expr::Ident(_, s) => *s,
            Expr::StructLit { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
            Expr::Assign { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::Member { span, .. } => *span,
            Expr::Index { span, .. } => *span,
            Expr::If { span, .. } => *span,
            Expr::Match { span, .. } => *span,
            Expr::Block(b) => b.span,
            Expr::Array(_, s) => *s,
            Expr::Tensor(_, s) => *s,
            Expr::Interpolated { span, .. } => *span,
        }
    }

    fn is_nogc_function(&self, name: &str) -> bool {
        self.nogc_functions.get(name).copied().unwrap_or(false)
    }
}
