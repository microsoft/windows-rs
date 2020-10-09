use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub winmd::Row);

impl Constant {
    pub fn value_type(self, reader: &TypeReader) -> u32 {
        reader.u32(self.0, 0)
    }

    pub fn value(self, reader: &TypeReader) -> winmd::Blob {
        reader.blob(self.0, 2)
    }
}
