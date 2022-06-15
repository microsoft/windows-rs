#[derive(Default)]
pub struct Field {
    pub name: String,
    pub signature: Vec<u8>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }
}
