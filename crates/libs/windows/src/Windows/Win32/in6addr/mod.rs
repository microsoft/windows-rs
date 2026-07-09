#[repr(C)]
#[derive(Clone, Copy)]
pub struct IN6_ADDR {
    pub u: IN6_ADDR_0,
}
impl Default for IN6_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IN6_ADDR_0 {
    pub Byte: [u8; 16],
    pub Word: [u16; 8],
}
impl Default for IN6_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIN6_ADDR(pub *mut IN6_ADDR);
impl LPIN6_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPIN6_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIN6_ADDR(pub *mut IN6_ADDR);
impl PIN6_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIN6_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
