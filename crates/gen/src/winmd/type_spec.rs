use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeSpec(pub winmd::Row);

impl TypeSpec {
    pub fn sig<'a>(&self, reader: &'a TypeReader) -> winmd::Blob<'a> {
        reader.blob(self.0, 0)
    }
}
