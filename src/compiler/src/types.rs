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
    Optional(Box<Type>),
    Generic(String, Vec<Type>), // Generic name, type arguments
    Parameter(String), // Generic parameter name
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
    pub params: Vec<String>,
    pub constraints: HashMap<String, Vec<Type>>,
}

impl GenericContext {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            constraints: HashMap::new(),
        }
    }
}
