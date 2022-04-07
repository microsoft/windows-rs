use super::*;

#[derive(Clone)]
pub enum MemberRefParent<'a> {
    TypeRef(TypeRef<'a>),
}

impl<'a> Decode<'a> for MemberRefParent<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            1 => Self::TypeRef(TypeRef(Row::new(scope, ScopeKey::new(row, TABLE_TYPEREF, file)))),
            _ => unimplemented!(),
        }
    }
}
