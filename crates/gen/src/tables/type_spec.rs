use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeSpec(pub Row);

impl TypeSpec {
    // TODO: find uses of TypeSpec::blob and replace with TypeSpec::signature?
    pub fn blob(&self) -> Blob {
        self.0.blob(0)
    }
}

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeSpec").field("row", &self.0).finish()
    }
}
