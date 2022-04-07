use super::*;

#[derive(Clone)]
pub enum AttributeType<'a> {
    MethodDef(MethodDef<'a>),
    MemberRef(MemberRef<'a>),
}

impl<'a> AttributeType<'a> {
    pub fn encode(&self) -> usize {
        (match self {
            Self::MethodDef(value) => ((value.0.key.row + 1) << 3) | 2,
            Self::MemberRef(value) => ((value.0.key.row + 1) << 3) | 3,
        }) as _
    }
}

impl<'a> Decode<'a> for AttributeType<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            2 => Self::MethodDef(MethodDef(Row::new(scope, ScopeKey::new(row, TABLE_METHODDEF, file)))),
            3 => Self::MemberRef(MemberRef(Row::new(scope, ScopeKey::new(row, TABLE_MEMBERREF, file)))),
            _ => unimplemented!(),
        }
    }
}
