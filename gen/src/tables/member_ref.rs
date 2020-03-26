use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef(pub Row);

impl MemberRef {
    pub fn parent(&self, reader: &Reader) -> MemberRefParent {
        reader.decode(self.0, 0)
    }

    pub fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(self.0, 1)
    }
}
