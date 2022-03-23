

#[derive(Default)]
pub struct TypeDef {
    pub name: String,
    pub namespace: String,
}

impl TypeDef {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self { name: name.to_string(), namespace: namespace.to_string(), ..Default::default() }
    }
}
