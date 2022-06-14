#[derive(Default)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
}

impl TypeName {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self { namespace: namespace.to_string(), name: name.to_string() }
    }
}
