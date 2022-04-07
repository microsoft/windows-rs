use super::*;

#[derive(Clone)]
pub enum TypeOrMethodDef<'a> {
    TypeDef(TypeDef<'a>),
    MethodDef(MethodDef<'a>),
}

impl<'a> TypeOrMethodDef<'a> {
    pub fn encode(&self) -> usize {
        (match self {
            Self::TypeDef(value) => ((value.0.key.row + 1) << 1),
            Self::MethodDef(value) => ((value.0.key.row + 1) << 1) | 1,
        }) as _
    }
}

impl<'a> Decode<'a> for TypeOrMethodDef<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 1) - 1), (code >> 1) - 1);
        match kind {
            0 => Self::TypeDef(TypeDef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEDEF, file)), Vec::new())),
            1 => Self::MethodDef(MethodDef(Row::new(scope, ScopeKey::new(row, TABLE_METHODDEF, file)))),
            _ => unimplemented!(),
        }
    }
}
