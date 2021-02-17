pub struct MethodFlags(pub u32);

impl MethodFlags {
    pub fn special(&self) -> bool {
        self.0 & 0b1000_0000_0000 != 0
    }
}
