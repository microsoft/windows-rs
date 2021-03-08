use super::*;
macros::table!(TypeSpec);

impl TypeSpec {
    // TODO: find uses of TypeSpec::blob and replace with TypeSpec::signature?
    pub fn blob(&self) -> Blob {
        self.reader.blob(self.row, 0)
    }
}

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeSpec")
        .field("row", &self.row)
        .finish()
    }
}
