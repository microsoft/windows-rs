use super::*;

#[derive(Default)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub constant: Option<Value>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }
}
