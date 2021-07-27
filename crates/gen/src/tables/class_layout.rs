use super::*;

#[derive(Clone)]
pub struct ClassLayout(pub Row);

impl ClassLayout {
    pub fn packing_size(&self) -> u32 {
        self.0.u32(0)
    }

    pub fn class_size(&self) -> u32 {
        self.0.u32(1)
    }
}
