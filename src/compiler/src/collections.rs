use crate::sema::{Type, Sema};

#[derive(Debug, Clone)]
pub enum CollectionAbstraction {
    FixedArray(Box<Type>, usize),
    StackList(Box<Type>),
}

pub struct CollectionSpecializer<'a> {
    _sema: &'a mut Sema,
}

impl<'a> CollectionSpecializer<'a> {
    pub fn new(sema: &'a mut Sema) -> Self {
        Self { _sema: sema }
    }

    pub fn specialize_type(&mut self, ty: &Type) -> Option<CollectionAbstraction> {
        match ty {
            Type::Array(inner) => {
                // In @nogc mode, we might want to specialize arrays to fixed size if known
                Some(CollectionAbstraction::StackList(inner.clone()))
            }
            _ => None,
        }
    }
}
