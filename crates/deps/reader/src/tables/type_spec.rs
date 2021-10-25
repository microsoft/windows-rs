use super::*;

#[derive(Clone)]
pub struct TypeSpec(pub Row);

impl TypeSpec {
    pub fn blob(&self) -> Blob {
        self.0.blob(0)
    }
}
