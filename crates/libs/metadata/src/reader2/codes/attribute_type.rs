use super::*;

pub enum AttributeType {
    MemberRef(MemberRef),
}

impl Decode for AttributeType {
    fn decode(file: usize, code: usize) -> Self {
        let (kind, row) = (code & ((1 << 3) - 1), (code >> 3) - 1);
        match kind {
            3 => Self::MemberRef(MemberRef(ScopeKey::new(row, TABLE_MEMBERREF, file))),
            _ => unimplemented!(),
        }
    }
}
