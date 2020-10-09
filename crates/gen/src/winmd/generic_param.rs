use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct GenericParam(pub winmd::Row);

impl GenericParam {
    pub fn name<'a>(&self, reader: &'a TypeReader) -> &'a str {
        reader.str(self.0, 3)
    }
}
