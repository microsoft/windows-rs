#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Flags {
    pub _bitfield: u8,
}
impl Flags {
    pub fn HardwareInterface(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_HardwareInterface(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn FilterInterface(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_FilterInterface(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn ConnectorPresent(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ConnectorPresent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Kind(&self) -> u8 {
        (self._bitfield << 2) >> 5
    }
    pub fn set_Kind(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Signed {
    pub _bitfield: i32,
}
impl Signed {
    pub fn Delta(&self) -> i32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_Delta(&mut self, value: i32) {
        self._bitfield = ((self._bitfield as u32 & !15) | (value as u32 & 15)) as i32;
    }
    pub fn Flag(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Flag(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as i32) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Wide {
    pub _bitfield: u32,
}
impl Wide {
    pub fn Low(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_Low(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Mid(&self) -> u32 {
        (self._bitfield << 8) >> 12
    }
    pub fn set_Mid(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1048575 << 4)) | ((value & 1048575) << 4);
    }
    pub fn High(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_High(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
