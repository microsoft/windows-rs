use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub Row);

impl Constant {
    pub fn value_type(&self, reader: &Reader) -> u32 {
        reader.u32(self.0, 0)
    }

    pub fn value<'a>(&self, reader: &'a Reader) -> Blob<'a> {
        reader.blob(self.0, 2)
    }
}
