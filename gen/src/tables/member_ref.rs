use crate::codes::MemberRefParent;
use crate::reader::Reader;
use crate::row::Row;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef(pub Row);

impl MemberRef {
    pub fn parent(self, reader: &Reader) -> MemberRefParent {
        reader.decode(self.0, 0)
    }

    pub fn name(self, reader: &Reader) -> &str {
        reader.str(self.0, 1)
    }
}
