use super::*;

#[derive(Default)]
pub struct TypeDef {
    pub flags: u32,
    pub name: String,
    pub namespace: String,
    pub field_list: Vec<Field>,
    pub method_list: Vec<MethodDef>,
    pub(crate) field_index: usize,
    pub(crate) method_index: usize,
}

impl TypeDef {
    pub fn module() -> Self {
        Self { name: "<Module>".to_string(), ..Default::default() }
    }

    pub fn winrt_interface(name: &str, namespace: &str) -> Self {
        Self { name: name.to_string(), namespace: namespace.to_string(), ..Default::default() }
    }
}
