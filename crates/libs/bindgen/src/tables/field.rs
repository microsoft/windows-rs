use super::*;

pub trait FieldExt {
    fn field_type(&self, enclosing: Option<&CppStruct>) -> Type;
}

impl FieldExt for Field {
    fn field_type(&self, enclosing: Option<&CppStruct>) -> Type {
        let ty = Type::from_metadata_type(&self.ty(), enclosing, &[]);
        if self.has_attribute("ConstAttribute") {
            ty.to_const_type().to_const_ptr()
        } else {
            ty
        }
    }
}
