use super::*;

pub enum MemberRefParent {
    TypeRef(TypeRef),
}

impl Decode for MemberRefParent {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            1 => Self::TypeRef(TypeRef(ScopeKey::new(row, TABLE_TYPEREF, file))),
            _ => unimplemented!(),
        }
    }
}
