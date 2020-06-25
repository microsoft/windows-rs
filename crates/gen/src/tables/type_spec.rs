use crate::blob::Blob;
use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeSpec(pub Row);

impl TypeSpec {
    pub fn sig<'a>(&self, reader: &'a TypeReader) -> Blob<'a> {
        reader.blob(self.0, 0)
    }
}
