use crate::ast::*;
use crate::diag::{Diagnostic, Span};
use crate::borrowck::BorrowChecker;
use crate::lifetime::LifetimeChecker;
use crate::moveck::MoveChecker;
use std::collections::HashMap;

use crate::types::Type;
use crate::interface::InterfaceSystem;

#[derive(Default)]
pub(crate) struct Scope {
    pub(crate) vars: HashMap<String, Type>,
}

pub struct Sema {
    pub(crate) scopes: Vec<Scope>,
    pub(crate) diags: Vec<Diagnostic>,
    pub(crate) functions: HashMap<String, Type>,
    pub(crate) extensions: HashMap<String, Vec<(Type, Type)>>, // Receiver type -> Vec<(Method Name, Method Sig)>
    pub(crate) interface_system: InterfaceSystem,
    pub(crate) sealed_types: HashMap<String, SealedDecl>,
    pub(crate) structs: HashMap<String, StructDecl>,
    pub(crate) fun_decls: HashMap<String, FunDecl>,
    pub(crate) nogc_functions: HashMap<String, bool>,
    pub(crate) templates: crate::templates::TemplateSystem,
    pub(crate) permissive: bool,
}

impl Sema {
    pub fn new() -> Self {
        let mut s = Self {
            scopes: Vec::new(),
            diags: Vec::new(),
            functions: HashMap::new(),
            extensions: HashMap::new(),
            interface_system: InterfaceSystem::new(),
            sealed_types: HashMap::new(),
            structs: HashMap::new(),
            fun_decls: HashMap::new(),
            nogc_functions: HashMap::new(),
            templates: crate::templates::TemplateSystem::new(),
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
        s.define_builtin("uiWindowDemo", Type::Func(vec![], Box::new(Type::Int)));
        s
    }

    pub fn check_program(mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        // Predeclare types (structs/enums/aliases/interfaces/sealed)
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    self.structs.insert(s.name.clone(), s.clone());
                    self.define_builtin(&s.name, Type::Named(s.name.clone()));
                }
                Item::Enum(e) => self.define_builtin(&e.name, Type::Named(e.name.clone())),
                Item::TypeAlias(t) => self.define_builtin(&t.name, Type::Named(t.name.clone())),
                Item::Interface(i) => {
                    self.interface_system.interfaces.insert(i.name.clone(), i.clone());
                    self.define_builtin(&i.name, Type::Named(i.name.clone()));
                }
                Item::Sealed(s) => {
                    self.sealed_types.insert(s.name.clone(), s.clone());
                    self.define_builtin(&s.name, Type::Named(s.name.clone()));
                    // Pre-register items inside sealed class
                    for nested in &s.items {
                        match nested {
                            Item::Struct(st) => {
                                self.structs.insert(st.name.clone(), st.clone());
                                self.define_builtin(&st.name, Type::Named(st.name.clone()));
                            }
                            Item::Enum(e) => self.define_builtin(&e.name, Type::Named(e.name.clone())),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        for item in &program.items {
            if let Item::Fun(f) = item {
                let sig = self.fun_sig(f);
                if let Some(recv) = &f.receiver {
                    let recv_ty = self.type_from_ref(recv);
                    let entry = self.extensions.entry(f.name.clone()).or_insert_with(Vec::new);
                    entry.push((recv_ty, sig));
                } else {
                    self.functions.insert(f.name.clone(), sig);
                    self.fun_decls.insert(f.name.clone(), f.clone());
                    self.nogc_functions.insert(f.name.clone(), f.nogc);
                }
            }
        }

        for item in &program.items {
            self.check_item(item);
        }

        self.validate_nogc(program);

        if self.diags.is_empty() {
            Ok(())
        } else {
            Err(self.diags)
        }
    }

    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Fun(f) => self.check_fun(f),
            Item::Struct(s) => self.check_struct(s),
            Item::Enum(_) => {}
            Item::TypeAlias(_) => {}
            Item::View(_) => {}
            Item::Resource(_) => {}
            Item::Interface(i) => self.check_interface(i),
            Item::Sealed(s) => self.check_sealed(s),
            Item::Const(v) => {
                let ty = self.check_expr(&v.value);
                let final_ty = if let Some(ann) = &v.ty {
                    let ann_ty = self.type_from_ref(ann);
                    self.unify(&ann_ty, &ty, v.span);
                    ann_ty
                } else {
                    ty
                };
                self.define_var(&v.name, final_ty, v.span);
            }
            Item::Stmt(s) => {
                self.check_stmt(s);
            }
        }
    }

    fn check_struct(&mut self, s: &StructDecl) {
        self.push_scope();
        for p in &s.generic_params {
            self.define_var(&p.name, Type::Parameter(p.name.clone()), p.span);
        }
        for interface_ref in &s.implements {
            let interface_ty = self.type_from_ref(interface_ref);
            if let Type::Named(name) = interface_ty {
                if let Some(interface) = self.interface_system.interfaces.get(&name).cloned() {
                    for method in &interface.methods {
                        // Check if struct has this method as an extension
                        let has_method = self.extensions.get(&method.name).map(|exts| {
                            exts.iter().any(|(recv, _)| recv == &Type::Named(s.name.clone()))
                        }).unwrap_or(false);
                        
                        if !has_method {
                            self.diags.push(Diagnostic::error(format!("struct '{}' does not implement method '{}' from interface '{}'", s.name, method.name, name), s.span));
                        }
                    }
                } else {
                    self.diags.push(Diagnostic::error(format!("interface '{}' not found", name), s.span));
                }
            }
        }
        self.pop_scope();
    }

    fn check_interface(&mut self, i: &InterfaceDecl) {
        self.push_scope();
        for p in &i.generic_params {
            self.define_var(&p.name, Type::Parameter(p.name.clone()), p.span);
        }
        // In a more complete implementation, we would also validate parameters/returns of method signatures here
        self.pop_scope();
    }

    fn check_sealed(&mut self, s: &SealedDecl) {
        self.push_scope();
        for p in &s.generic_params {
            self.define_var(&p.name, Type::Parameter(p.name.clone()), p.span);
        }
        
        let mut enforcer = crate::sealed::SealedEnforcer::new(self);
        enforcer.validate_sealed_decl(s);

        for item in &s.items {
            self.check_item(item);
        }
        self.pop_scope();
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
            // Relax return check for main and common Int returners that end in Stmt
            if (fun.name == "main" || ret_ty == Type::Int) && body_ty == Type::Unit {
                // OK: Codegen will handle default return 0 for Int
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
                let final_ty = if let Some(ann) = &v.ty {
                    let ann_ty = self.type_from_ref(ann);
                    self.unify(&ann_ty, &ty, v.span);
                    ann_ty
                } else {
                    ty
                };
                self.define_var(&v.name, final_ty, v.span);
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
                    self.check_stmt_with(stmt, nogc)
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
                        self.diags.push(Diagnostic::error("for-in expects array", *span));
                        Type::Unknown
                    }
                };
                self.push_scope();
                self.define_var(name, elem, *span);
                self.check_block_with(body, nogc);
                self.pop_scope();
                Type::Unit
            }
            Stmt::Match(expr, arms, _span) => {
                let actual_ty = self.check_expr_with(expr, nogc);
                let mut ty = Type::Nothing;
                for arm in arms {
                    self.push_scope();
                    {
                        let mut pc = crate::pattern::PatternChecker::new(self);
                        pc.check_pattern(&arm.pat, &actual_ty);
                    }
                    let arm_ty = self.check_expr_with(&arm.body, nogc);
                    self.pop_scope();
                    ty = self.join_types(ty, arm_ty);
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
                        self.diags.push(Diagnostic::error("allocation not allowed in @nogc", *span));
                    }
                }
                self.type_of_literal(l)
            }
            Expr::Ident(name, span) => {
                if name == "null" {
                    return Type::Nothing;
                }
                self.lookup_var(name, *span)
            }
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
                
                // Allow re-typing if the target is currently Nothing (likely initialized from null)
                if lt == Type::Nothing {
                    if let Expr::Ident(name, _) = &**left {
                        for scope in self.scopes.iter_mut().rev() {
                            if scope.vars.contains_key(name) {
                                scope.vars.insert(name.clone(), rt.clone());
                                return rt;
                            }
                        }
                    }
                }

                // Handle Optional: T can be assigned to T?
                if let Type::Optional(inner) = lt.clone() {
                    if &**inner == &rt || rt == Type::Nothing {
                        return lt.clone();
                    }
                }

                self.unify(&lt, &rt, *span);
                lt
            }
            Expr::Call { callee, args, span } => {
                if let Expr::Ident(name, _) = &**callee {
                    if name == "@import" || name == "@bridge" {
                        if args.is_empty() {
                            self.diags.push(Diagnostic::error("FFI call requires a string argument", *span));
                        } else if !matches!(args[0], Expr::Literal(Literal::String(_), _)) {
                            self.diags.push(Diagnostic::error("FFI call requires string literal", self.span_of(&args[0])));
                        }
                    }
                    if nogc && !self.is_nogc_function(name) {
                        self.diags.push(Diagnostic::error("call to non-@nogc function in @nogc", *span));
                    }
                }
                
                // Handle Extension Functions / Methods
                if let Expr::Member { target, name, span: m_span } = &**callee {
                    let target_ty = self.check_expr_with(target, nogc);
                    if let Some(sig) = crate::extension::resolve_extension_method(self, &target_ty, name) {
                        if let Type::Func(params, ret) = sig {
                            if params.len() != args.len() {
                                self.diags.push(Diagnostic::error("argument count mismatch", *span));
                            }
                            for (arg, p) in args.iter().zip(params.iter()) {
                                let at = self.check_expr_with(arg, nogc);
                                self.unify(p, &at, self.span_of(arg));
                            }
                            return *ret;
                        }
                    }
                    if !self.permissive {
                        self.diags.push(Diagnostic::error(format!("no method '{}' found for type {:?}", name, target_ty), *m_span));
                    }
                    return Type::Unknown;
                }

                let mut ct = self.check_expr_with(callee, nogc);

                // Handle Generic Function Instantiation
                if let Type::Generic(name, gen_args) = ct.clone() {
                    // 1. Check constraints if we have a definition
                    if let Some(f_decl) = self.lookup_fun_decl(&name) {
                        if f_decl.generic_params.len() != gen_args.len() {
                            self.diags.push(Diagnostic::error(format!("generic argument count mismatch for '{}'", name), *span));
                        } else {
                            for (param, arg) in f_decl.generic_params.iter().zip(gen_args.iter()) {
                                for constraint in &param.constraints {
                                    let constraint_ty = self.type_from_ref(constraint);
                                    self.check_satisfies(arg, &constraint_ty, self.span_of(callee));
                                }
                            }
                        }
                    }
                    
                    if let Some(f_ty) = self.functions.get(&name) {
                        // For now, this is a placeholder for actual monomorphization
                        ct = f_ty.clone();
                    }
                }

                match ct {
                    Type::Func(params, ret) => {
                        if params.len() != args.len() {
                            self.diags.push(Diagnostic::error("argument count mismatch", *span));
                        }
                        for (arg, p) in args.iter().zip(params.iter()) {
                            let at = self.check_expr_with(arg, nogc);
                            self.unify(p, &at, self.span_of(arg));
                        }
                        *ret
                    }
                    Type::Unknown | Type::Any => Type::Unknown,
                    _ => {
                        self.diags.push(Diagnostic::error("call to non-function", *span));
                        Type::Unknown
                    }
                }
            }
            Expr::Member { target, name, span } => {
                let mut target_ty = self.check_expr_with(target, nogc);
                
                // If target is Optional, check the inner type and warn
                if let Type::Optional(inner) = target_ty {
                    self.diags.push(Diagnostic::warning("accessing member of optional type; should check for null first", *span));
                    target_ty = *inner;
                }

                // 1. Check for struct fields
                if let Type::Named(s_name) = &target_ty {
                    if let Some(s_decl) = self.structs.get(s_name).cloned() {
                        if let Some(field) = s_decl.fields.iter().find(|f| &f.name == name) {
                            return self.type_from_ref(&field.ty);
                        }
                    }
                }

                // 2. Check if it's an extension function reference
                if let Some(sig) = crate::extension::resolve_extension_method(self, &target_ty, name) {
                    return sig;
                }
                if !self.permissive {
                    self.diags.push(Diagnostic::error(format!("unknown member '{}' for type {:?}", name, target_ty), *span));
                }
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
                        self.diags.push(Diagnostic::error("indexing non-array", *span));
                        Type::Unknown
                    }
                }
            }
            Expr::If { cond, then_block, else_block, span: _span } => {
                let ct = self.check_expr_with(cond, nogc);
                self.unify(&Type::Bool, &ct, self.span_of(cond));
                let tt = self.check_block_with(then_block, nogc);
                let et = self.check_block_with(else_block, nogc);
                self.join_types(tt, et)
            }
            Expr::Match { expr, arms, span: _span } => {
                let actual_ty = self.check_expr_with(expr, nogc);
                let mut ty = Type::Unknown;
                for arm in arms {
                    self.push_scope();
                    {
                        let mut pc = crate::pattern::PatternChecker::new(self);
                        pc.check_pattern(&arm.pat, &actual_ty);
                    }
                    let at = self.check_expr_with(&arm.body, nogc);
                    self.pop_scope();
                    ty = if ty == Type::Unknown { at } else { self.join_types(ty, at) };
                }
                ty
            }
            Expr::Block(b) => self.check_block_with(b, nogc),
            Expr::Array(items, _) => {
                if nogc {
                    self.diags.push(Diagnostic::error("allocation not allowed in @nogc", self.span_of(expr)));
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
                    self.diags.push(Diagnostic::error("allocation not allowed in @nogc", self.span_of(expr)));
                }
                Type::Tensor(Box::new(Type::Float))
            }
            Expr::Interpolated { parts, .. } => {
                if nogc {
                    self.diags.push(Diagnostic::error("allocation not allowed in @nogc", self.span_of(expr)));
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

    pub(crate) fn type_of_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Char(_) => Type::Char,
            Literal::Bool(_) => Type::Bool,
        }
    }

    pub fn is_nogc_function(&self, name: &str) -> bool {
        self.nogc_functions.get(name).copied().unwrap_or(false)
    }

    pub fn type_from_ref(&self, tr: &TypeRef) -> Type {
        match tr {
                "Any" => Type::Any,
                "Nothing" => Type::Nothing,
                _ => Type::Named(name.clone()),
            },
            TypeRef::Named(name, args, span) => {
                if args.is_empty() {
                    match name.as_str() {
                        "Int" => Type::Int,
                        "UInt" => Type::UInt,
                        "Float" => Type::Float,
                        "Bool" => Type::Bool,
                        "Char" => Type::Char,
                        "String" => Type::String,
                        "Void" | "Unit" => Type::Unit,
                        "Any" => Type::Any,
                        "Nothing" => Type::Nothing,
                        _ => Type::Named(name.clone()),
                    }
                } else {
                    let arg_tys = args.iter().map(|a| self.type_from_ref(a)).collect();
                    Type::Generic(name.clone(), arg_tys)
                }
            }
            TypeRef::Tuple(elems, _) => Type::Tuple(elems.iter().map(|e| self.type_from_ref(e)).collect()),
            TypeRef::Array(inner, _) => Type::Array(Box::new(self.type_from_ref(inner))),
            TypeRef::Tensor { elem, .. } => Type::Tensor(Box::new(self.type_from_ref(elem))),
            TypeRef::Optional(inner, _) => Type::Optional(Box::new(self.type_from_ref(inner))),
            TypeRef::NonNull(inner, _) => self.type_from_ref(inner),
        }
    }

    fn fun_sig(&mut self, f: &FunDecl) -> Type {
        self.push_scope();
        for p in &f.generic_params {
            self.define_var(&p.name, Type::Parameter(p.name.clone()), p.span);
        }
        let params = f.params.iter().map(|p| self.type_from_ref(&p.ty)).collect();
        let ret = f.ret.as_ref().map(|t| self.type_from_ref(t)).unwrap_or(Type::Unit);
        self.pop_scope();
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
        self.diags.push(Diagnostic::error(format!("undefined symbol '{name}'"), span));
        Type::Unknown
    }

    pub(crate) fn define_var(&mut self, name: &str, ty: Type, span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.vars.contains_key(name) {
                self.diags.push(Diagnostic::error(format!("redefinition of '{name}'"), span));
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

    pub(crate) fn unify(&mut self, expected: &Type, actual: &Type, span: Span) {
        if self.permissive {
            return;
        }
        if matches!(expected, Type::Unknown | Type::Any) || matches!(actual, Type::Unknown | Type::Any) {
            return;
        }
        
        // Handle Optional: T can be assigned to T?
        if let Type::Optional(inner) = expected {
            if &**inner == actual || matches!(actual, Type::Nothing) {
                return;
            }
        }

        if expected != actual {
            self.diags.push(Diagnostic::error(
                format!("type mismatch: expected {:?}, got {:?}", expected, actual),
                span,
            ));
        }
    }

    fn join_types(&self, a: Type, b: Type) -> Type {
        if a == b {
            a
        } else if a == Type::Nothing {
            if matches!(b, Type::Optional(_)) { b } else { Type::Optional(Box::new(b)) }
        } else if b == Type::Nothing {
            if matches!(a, Type::Optional(_)) { a } else { Type::Optional(Box::new(a)) }
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
                self.diags.push(Diagnostic::error("expected numeric type", span));
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
                self.diags.push(Diagnostic::error("expected integer type", span));
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

    fn lookup_fun_decl(&self, name: &str) -> Option<&FunDecl> {
        self.fun_decls.get(name)
    }

    fn check_satisfies(&mut self, ty: &Type, constraint: &Type, span: Span) {
        if let Type::Named(name) = constraint {
            if let Some(interface) = self.interface_system.interfaces.get(name) {
                // In a real implementation, we would check if ty implements all methods in interface
                // For now, assume it works if we have a struct definition for ty
                if let Type::Named(s_name) = ty {
                    if self.structs.contains_key(s_name) {
                        return;
                    }
                }
            }
        }
        if ty != constraint && !matches!(constraint, Type::Any) {
            self.diags.push(Diagnostic::error(format!("type {:?} does not satisfy constraint {:?}", ty, constraint), span));
        }
    }

    fn validate_nogc(&mut self, program: &Program) {
        for item in &program.items {
            if let Item::Fun(f) = item {
                if f.nogc {
                    {
                        let mut bck = BorrowChecker::new(self);
                        bck.check_block(&f.body);
                    }
                    {
                        let mut mck = MoveChecker::new(self);
                        mck.check_block(&f.body);
                    }
                    {
                        let mut lck = LifetimeChecker::new(self);
                        lck.check_block(&f.body);
                    }
                    {
                        let mut ngck = crate::nogc::NoGcChecker::new(self);
                        ngck.check_fun(f);
                    }
                }
            }
        }
    }
}
