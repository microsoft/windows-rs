use super::*;

impl std::fmt::Debug for ClassLayout<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ClassLayout")
            .field(&self.packing_size())
            .finish()
    }
}

impl ClassLayout<'_> {
    pub fn packing_size(&self) -> usize {
        self.usize(0)
    }
}
