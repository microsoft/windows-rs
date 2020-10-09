use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeRef(pub winmd::Row);

impl TypeRef {
    pub fn name(self, reader: &TypeReader) -> (&str, &str) {
        (reader.str(self.0, 2), reader.str(self.0, 1))
    }

    pub fn resolve(self, reader: &TypeReader) -> winmd::TypeDef {
        reader.resolve_type_def(self.name(reader))
    }
}
