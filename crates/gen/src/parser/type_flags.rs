pub struct TypeFlags(pub u32);

impl TypeFlags {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100_0000_0000_0000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b10_0000 != 0
    }
    pub fn explicit(&self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}
