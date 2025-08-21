use super::*;

impl std::fmt::Debug for ClassLayout<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ClassLayout")
            .field(&self.packing_size())
            .finish()
    }
}

impl ClassLayout<'_> {
    pub fn packing_size(&self) -> u16 {
        self.usize(0).try_into().unwrap()
    }

    pub fn class_size(&self) -> u32 {
        self.usize(1).try_into().unwrap()
    }
}
