use crate::codes::MemberRefParent;
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct MemberRef(pub Row);

impl MemberRef {
    pub fn parent(self, reader: &TypeReader) -> MemberRefParent {
        reader.decode(self.0, 0)
    }

    pub fn name(self, reader: &TypeReader) -> &str {
        reader.str(self.0, 1)
    }
}
