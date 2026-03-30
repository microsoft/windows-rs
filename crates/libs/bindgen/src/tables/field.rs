use super::*;

pub trait FieldExt {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type;
}

impl FieldExt for Field {
    fn field_type(&self, enclosing: Option<&CppStruct>, reader: &Reader) -> Type {
        let ty = Type::from_metadata_type(&self.ty(), enclosing, &[], reader);
        if self.has_attribute("ConstAttribute") {
            ty.to_const_type().to_const_ptr()
        } else {
            ty
        }
    }
}
