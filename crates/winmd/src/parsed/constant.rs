use super::*;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub Row);

impl Constant {
    pub fn value_type(self, reader: &TypeReader) -> ElementType {
        ElementType::from_code(reader.u32(self.0, 0))
    }

    pub fn value(self, reader: &TypeReader) -> Blob {
        reader.blob(self.0, 2)
    }
}
