use crate::ast::*;
use crate::diag::Diagnostic;
use crate::escape::{analyze_escape, EscapeResult};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::AddressSpace;
use std::collections::HashMap;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    diags: Vec<Diagnostic>,
    escape_map: HashMap<String, EscapeResult>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
            diags: Vec::new(),
            escape_map: HashMap::new(),
        }
    }

    pub fn emit_program(mut self, program: &Program) -> Result<Module<'ctx>, Vec<Diagnostic>> {
        self.escape_map = analyze_escape(program);
        for item in &program.items {
            if let Item::Fun(f) = item {
                self.declare_function(f);
            }
        }

        for item in &program.items {
            if let Item::Fun(f) = item {
                self.emit_function(f);
            }
        }

        if self.diags.is_empty() {
            Ok(self.module)
        } else {
            Err(self.diags)
        }
    }

    fn declare_function(&mut self, fun: &FunDecl) {
        let param_types: Vec<BasicMetadataTypeEnum> = fun
            .params
            .iter()
            .map(|p| BasicMetadataTypeEnum::from(self.llvm_type(&p.ty)))
            .collect();
        let fn_type = if let Some(ret) = &fun.ret {
            self.llvm_type(ret).fn_type(&param_types, false)
        } else {
            self.context.void_type().fn_type(&param_types, false)
        };
        self.module.add_function(&fun.name, fn_type, None);
    }

    fn emit_function(&mut self, fun: &FunDecl) {
        let func = match self.module.get_function(&fun.name) {
            Some(f) => f,
            None => return,
        };
        let _escapes = self.escape_map.get(&fun.name);
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        let mut returned = false;

        // Emit basic statement calls for the self-hosted driver.
        for stmt in &fun.body.stmts {
            if let Stmt::Expr(Expr::Call { callee, args, .. }, _) = stmt {
                let _ = self.emit_call(callee, args);
            }
        }

        if let Some(ret) = &fun.ret {
            if let Some(val) = self.try_emit_return(&fun.body, ret) {
                let _ = self.builder.build_return(Some(&val));
                returned = true;
            }
        }
        if !returned {
            if let Some(ret) = &fun.ret {
                if let Some(zero) = self.emit_zero(ret) {
                    let _ = self.builder.build_return(Some(&zero));
                    return;
                }
            }
            let _ = self.builder.build_return(None);
        }
    }

    fn emit_call(&mut self, callee: &Expr, args: &[Expr]) -> Option<BasicValueEnum<'ctx>> {
        // Minimal FFI: @import("symbol") calls an extern symbol with no args.
        if let Expr::Ident(name, _) = callee {
            if name == "@import" {
                if let Some(Expr::Literal(Literal::String(sym), _)) = args.get(0) {
                    let fn_ty = self.context.void_type().fn_type(&[], false);
                    let f = self.module.add_function(sym, fn_ty, None);
                    let _ = self.builder.build_call(f, &[], "ffi");
                    return None;
                }
            }
        }
        None
    }

    fn try_emit_return(&self, body: &Block, ret: &TypeRef) -> Option<BasicValueEnum<'ctx>> {
        if let Some(Expr::Literal(lit, _)) = body.tail.as_deref().map(|e| self.fold_expr(e)) {
            return self.emit_literal(lit, ret);
        }
        for stmt in body.stmts.iter().rev() {
            if let Stmt::Return(Some(expr), _) = stmt {
                if let Expr::Literal(lit, _) = self.fold_expr(expr) {
                    return self.emit_literal(lit, ret);
                }
            }
        }
        None
    }

    fn fold_expr(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::Unary { op, expr, span } => {
                let inner = self.fold_expr(expr);
                if let Expr::Literal(lit, _) = inner {
                    return match (op, lit.clone()) {
                        (UnaryOp::Neg, Literal::Int(v)) => Expr::Literal(Literal::Int(-v), *span),
                        (UnaryOp::Neg, Literal::Float(v)) => Expr::Literal(Literal::Float(-v), *span),
                        (UnaryOp::Pos, Literal::Int(v)) => Expr::Literal(Literal::Int(v), *span),
                        (UnaryOp::Pos, Literal::Float(v)) => Expr::Literal(Literal::Float(v), *span),
                        (UnaryOp::Not, Literal::Bool(v)) => Expr::Literal(Literal::Bool(!v), *span),
                        _ => Expr::Unary { op: *op, expr: Box::new(Expr::Literal(lit, *span)), span: *span },
                    };
                }
                Expr::Unary { op: *op, expr: Box::new(inner), span: *span }
            }
            Expr::Binary { left, op, right, span } => {
                let l = self.fold_expr(left);
                let r = self.fold_expr(right);
                if let (Expr::Literal(lit_l, _), Expr::Literal(lit_r, _)) = (&l, &r) {
                    if let Some(lit) = self.fold_binary(lit_l, *op, lit_r) {
                        return Expr::Literal(lit, *span);
                    }
                }
                Expr::Binary { left: Box::new(l), op: *op, right: Box::new(r), span: *span }
            }
            _ => expr.clone(),
        }
    }

    fn fold_binary(&self, l: &Literal, op: BinaryOp, r: &Literal) -> Option<Literal> {
        match (l, r) {
            (Literal::Int(a), Literal::Int(b)) => match op {
                BinaryOp::Add => Some(Literal::Int(a + b)),
                BinaryOp::Sub => Some(Literal::Int(a - b)),
                BinaryOp::Mul => Some(Literal::Int(a * b)),
                BinaryOp::Div => Some(Literal::Int(a / b)),
                BinaryOp::Mod => Some(Literal::Int(a % b)),
                BinaryOp::Eq => Some(Literal::Bool(a == b)),
                BinaryOp::NotEq => Some(Literal::Bool(a != b)),
                BinaryOp::Lt => Some(Literal::Bool(a < b)),
                BinaryOp::LtEq => Some(Literal::Bool(a <= b)),
                BinaryOp::Gt => Some(Literal::Bool(a > b)),
                BinaryOp::GtEq => Some(Literal::Bool(a >= b)),
                _ => None,
            },
            (Literal::Float(a), Literal::Float(b)) => match op {
                BinaryOp::Add => Some(Literal::Float(a + b)),
                BinaryOp::Sub => Some(Literal::Float(a - b)),
                BinaryOp::Mul => Some(Literal::Float(a * b)),
                BinaryOp::Div => Some(Literal::Float(a / b)),
                BinaryOp::Eq => Some(Literal::Bool(a == b)),
                BinaryOp::NotEq => Some(Literal::Bool(a != b)),
                BinaryOp::Lt => Some(Literal::Bool(a < b)),
                BinaryOp::LtEq => Some(Literal::Bool(a <= b)),
                BinaryOp::Gt => Some(Literal::Bool(a > b)),
                BinaryOp::GtEq => Some(Literal::Bool(a >= b)),
                _ => None,
            },
            (Literal::Bool(a), Literal::Bool(b)) => match op {
                BinaryOp::And => Some(Literal::Bool(*a && *b)),
                BinaryOp::Or => Some(Literal::Bool(*a || *b)),
                BinaryOp::Eq => Some(Literal::Bool(a == b)),
                BinaryOp::NotEq => Some(Literal::Bool(a != b)),
                _ => None,
            },
            _ => None,
        }
    }

    fn emit_literal(&self, lit: Literal, _ret: &TypeRef) -> Option<BasicValueEnum<'ctx>> {
        match lit {
            Literal::Int(v) => Some(self.context.i64_type().const_int(v as u64, true).as_basic_value_enum()),
            Literal::Float(v) => Some(self.context.f64_type().const_float(v).as_basic_value_enum()),
            Literal::Bool(v) => Some(self.context.bool_type().const_int(v as u64, false).as_basic_value_enum()),
            _ => None,
        }
    }

    fn emit_zero(&self, ret: &TypeRef) -> Option<BasicValueEnum<'ctx>> {
        match ret {
            TypeRef::Named(name, _) => match name.as_str() {
                "Int" | "UInt" => Some(self.context.i64_type().const_int(0, false).as_basic_value_enum()),
                "Float" => Some(self.context.f64_type().const_float(0.0).as_basic_value_enum()),
                "Bool" => Some(self.context.bool_type().const_int(0, false).as_basic_value_enum()),
                _ => None,
            },
            _ => None,
        }
    }

    fn llvm_type(&self, ty: &TypeRef) -> BasicTypeEnum<'ctx> {
        match ty {
            TypeRef::Named(name, _) => match name.as_str() {
                "Int" => self.context.i64_type().as_basic_type_enum(),
                "UInt" => self.context.i64_type().as_basic_type_enum(),
                "Float" => self.context.f64_type().as_basic_type_enum(),
                "Bool" => self.context.bool_type().as_basic_type_enum(),
                "Char" => self.context.i32_type().as_basic_type_enum(),
                _ => self.context.i8_type().ptr_type(AddressSpace::default()).as_basic_type_enum(),
            },
            TypeRef::Tuple(_, _) => self.context.i8_type().ptr_type(AddressSpace::default()).as_basic_type_enum(),
            TypeRef::Array(_, _) => self.context.i8_type().ptr_type(AddressSpace::default()).as_basic_type_enum(),
            TypeRef::Tensor { .. } => self.context.i8_type().ptr_type(AddressSpace::default()).as_basic_type_enum(),
            TypeRef::Optional(inner, _) => self.llvm_type(inner),
            TypeRef::NonNull(inner, _) => self.llvm_type(inner),
        }
    }
}
