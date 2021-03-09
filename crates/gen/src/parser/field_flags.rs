#[derive(Default)]
pub struct FieldFlags(pub u32);

impl FieldFlags {
    pub fn literal(&self) -> bool {
        self.0 & 0b100_0000 != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}
