pub type Alias = Wrapped;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Bits {
    pub storage: u16,
}
impl Bits {
    pub fn signed(&self) -> Alias {
        Wrapped((((self.storage & 15) as i16) << 12) >> 12)
    }
    pub fn set_signed(&mut self, value: Alias) {
        self.storage = (self.storage & !15) | ((value.0 as u16) & 15);
    }
    pub fn mode(&self) -> ModeAlias {
        Mode(((((self.storage >> 4) & 7) as i16) << 13) >> 13)
    }
    pub fn set_mode(&mut self, value: ModeAlias) {
        self.storage = (self.storage & !(7 << 4)) | (((value.0 as u16) & 7) << 4);
    }
    pub fn enabled(&self) -> bool {
        ((self.storage >> 7) & 1) != 0
    }
    pub fn set_enabled(&mut self, value: bool) {
        self.storage = (self.storage & !(1 << 7)) | (((value as u16) & 1) << 7);
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Mode(pub i16);
impl Mode {
    pub const Negative: Self = Self(-1);
    pub const Positive: Self = Self(1);
}
pub type ModeAlias = Mode;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Wrapped(pub i16);
