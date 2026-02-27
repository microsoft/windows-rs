use super::*;

pub trait FieldExt {
    fn field_type(&self, enclosing: Option<&CppStruct>) -> Type;
}

impl FieldExt for Field {
    fn field_type(&self, enclosing: Option<&CppStruct>) -> Type {
        let mut blob = self.blob(2);
        let prolog = blob.read_u8();
        debug_assert_eq!(prolog, 0x6);
        blob.read_modifiers();

        let ty = Type::from_blob(&mut blob, enclosing, &[]);

        if self.has_attribute("ConstAttribute") {
            ty.to_const_type().to_const_ptr()
        } else {
            ty
        }
    }
}
