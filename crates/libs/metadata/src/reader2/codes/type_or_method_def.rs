use super::*;

#[derive(Clone)]
pub enum TypeOrMethodDef<'a> {
    TypeDef(TypeDef<'a>),
}

impl<'a> TypeOrMethodDef<'a> {
    pub fn encode(&self) -> usize {
        (match self {
            Self::TypeDef(value) => ((value.0.key.row + 1) << 1),
        }) as _
    }
}
