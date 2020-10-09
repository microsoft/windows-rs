use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct MemberRef(pub winmd::Row);

impl MemberRef {
    pub fn parent(self, reader: &TypeReader) -> winmd::MemberRefParent {
        reader.decode(self.0, 0)
    }

    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 1)
    }
}
