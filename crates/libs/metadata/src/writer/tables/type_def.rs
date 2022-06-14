use super::*;

#[derive(Default)]
pub struct TypeDef {
    pub flags: TypeAttributes,
    pub name: String,
    pub namespace: String,
    pub extends: TypeName,
    pub field_list: Vec<Field>,
    pub method_list: Vec<MethodDef>,
    pub(crate) field_index: usize,
    pub(crate) method_index: usize,
}

impl TypeDef {
    pub fn module() -> Self {
        Self { name: "<Module>".to_string(), ..Default::default() }
    }

    pub fn new(type_name: TypeName) -> Self {
        Self { name: type_name.name, namespace: type_name.namespace, ..Default::default() }
    }
}
