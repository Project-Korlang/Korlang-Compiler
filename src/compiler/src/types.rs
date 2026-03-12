use std::collections::HashMap;
use crate::symbols::Symbol;

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
    Named(Symbol),
    Func(Vec<Type>, Box<Type>),
    Optional(Box<Type>),
    Generic(Symbol, Vec<Type>), // Generic name, type arguments
    Parameter(Symbol), // Generic parameter name
    Unknown,
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        matches!(self, Type::Int | Type::UInt | Type::Float | Type::Bool | Type::Char | Type::String | Type::Unit)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::UInt | Type::Float)
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, Type::Optional(_) | Type::Any)
    }
}

pub struct GenericContext {
    pub params: Vec<Symbol>,
    pub constraints: HashMap<Symbol, Vec<Type>>,
}

impl GenericContext {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            constraints: HashMap::new(),
        }
    }
}
