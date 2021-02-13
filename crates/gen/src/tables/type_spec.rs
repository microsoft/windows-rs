use super::*;
macros::table!(TypeSpec);

impl TypeSpec {
    // TODO: find uses of TypeSpec::blob and replace with TypeSpec::signature?
    pub fn blob(&self) -> Blob {
        self.reader.blob(self.row, 0)
    }
}
