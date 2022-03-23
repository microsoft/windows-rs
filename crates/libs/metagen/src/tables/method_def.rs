#[derive(Default)]
pub struct MethodDef {
    pub name: String,
}

impl MethodDef {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
