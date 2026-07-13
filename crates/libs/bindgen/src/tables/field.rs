use super::*;

pub trait FieldExt {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type;
}

impl FieldExt for Field {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type {
        Type::from_metadata_type(&self.ty(), enclosing, &[], reader)
    }
}
