use super::*;

#[derive(Clone)]
pub struct TypeSpec(pub Row);

impl TypeSpec {
    // TODO: find uses of TypeSpec::blob and replace with TypeSpec::signature?
    pub fn blob(&self) -> Blob {
        self.0.blob(0)
    }
}
