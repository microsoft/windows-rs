pub type Alias = Scalar;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Member {
    pub storage: u64,
}
#[cfg(target_arch = "x86")]
impl Member {
    pub fn value(&self) -> Alias {
        ((((self.storage) & 15) as i32) << 28) >> 28
    }
    pub fn set_value(&mut self, value: Alias) {
        self.storage = (self.storage & !15) | ((value as u64) & 15);
    }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Member {
    pub fn value(&self) -> Alias {
        (self.storage) & 15
    }
    pub fn set_value(&mut self, value: Alias) {
        self.storage = (self.storage & !15) | (value & 15);
    }
}
#[cfg(target_arch = "x86")]
pub type Scalar = i32;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type Scalar = u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Storage {
    pub _bitfield: Alias,
}
#[cfg(target_arch = "x86")]
impl Storage {
    pub fn value(&self) -> i32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_value(&mut self, value: i32) {
        self._bitfield = ((self._bitfield as u32 & !15) | (value as u32 & 15)) as i32;
    }
    pub fn flag(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_flag(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as i32) << 4);
    }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Storage {
    pub fn value(&self) -> u64 {
        (self._bitfield << 60) >> 60
    }
    pub fn set_value(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn flag(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_flag(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u64) << 4);
    }
}
