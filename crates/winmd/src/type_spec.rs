use super::*;
macros::table!(TypeSpec);

impl TypeSpec {
    pub fn sig(&self) -> Blob {
        self.reader.blob(self.row, 0)
    }
}
