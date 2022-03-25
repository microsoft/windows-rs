use super::*;

#[derive(Default)]
pub struct MethodDef {
    pub name: String,
    pub signature: Vec<u8>,
    pub param_list: Vec<Param>,
    pub(crate) param_index: usize,
}

impl MethodDef {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }
}
