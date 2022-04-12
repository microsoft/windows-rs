use super::*;

pub enum TypeOrMethodDef {
    TypeDef(TypeDef),
}

impl TypeOrMethodDef {
    pub fn encode(&self) -> usize {
        (match self {
            Self::TypeDef(value) => ((value.0.row + 1) << 1),
        }) as _
    }
}
