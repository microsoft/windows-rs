use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeSpec(pub Row);

impl TypeSpec {
    pub fn sig<'a>(&self, reader: &'a Reader) -> Blob<'a> {
        reader.blob(self.0, 0)
    }
}
