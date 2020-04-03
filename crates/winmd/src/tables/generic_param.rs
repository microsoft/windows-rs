use crate::reader::Reader;
use crate::row::Row;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericParam(pub Row);

impl GenericParam {
    pub fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(self.0, 3)
    }
}
