use super::*;

#[derive(Clone)]
pub enum AttributeType<'a> {
    MemberRef(MemberRef<'a>),
}

impl<'a> Decode<'a> for AttributeType<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            3 => Self::MemberRef(MemberRef(Row::new(scope, ScopeKey::new(row, TABLE_MEMBERREF, file)))),
            _ => unimplemented!(),
        }
    }
}
