use super::TypeDef;
use crate::reader::Reader;
use crate::row::Row;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeRef(pub Row);

impl TypeRef {
    pub fn name(self, reader: &Reader) -> (&str, &str) {
        (reader.str(self.0, 2), reader.str(self.0, 1))
    }

    pub fn resolve(self, reader: &Reader) -> TypeDef {
        reader.resolve(self.name(reader))
    }
}
