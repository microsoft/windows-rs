use super::*;

#[derive(Default)]
pub struct TypeDef {
    pub flags: TypeAttributes,
    pub type_name: TypeName,
    pub extends: Option<TypeRef>,
    pub field_list: Vec<Field>,
    pub method_list: Vec<MethodDef>,
    pub(crate) field_index: usize,
    pub(crate) method_index: usize,
    pub(crate) extends_index: TypeDefOrRef,
}

impl TypeDef {
    pub fn module() -> Self {
        Self::new(TypeName::new("", "<Module>"))
    }

    pub fn new(type_name: TypeName) -> Self {
        Self { type_name, ..Default::default() }
    }
}
