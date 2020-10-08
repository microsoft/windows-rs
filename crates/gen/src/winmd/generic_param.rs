use crate::row::Row;
use crate::TypeReader;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct GenericParam(pub Row);

impl GenericParam {
    pub fn name<'a>(&self, reader: &'a TypeReader) -> &'a str {
        reader.str(self.0, 3)
    }
}
