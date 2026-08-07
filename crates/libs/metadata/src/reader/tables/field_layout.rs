use super::*;

impl std::fmt::Debug for FieldLayout<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("FieldLayout").field(&self.offset()).finish()
    }
}

impl FieldLayout<'_> {
    pub fn offset(&self) -> u32 {
        self.usize(0).try_into().unwrap()
    }

    pub fn field(&self) -> Field<'_> {
        self.row(1)
    }
}
