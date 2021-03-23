use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClassLayout(pub Row);

impl ClassLayout {
    pub fn packing_size(&self) -> u32 {
        self.0.u32(0)
    }

    pub fn class_size(&self) -> u32 {
        self.0.u32(1)
    }
}

impl std::fmt::Debug for ClassLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassLayout").field("row", &self.0).finish()
    }
}
