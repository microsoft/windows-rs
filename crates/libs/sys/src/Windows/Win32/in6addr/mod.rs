#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IN6_ADDR {
    pub u: IN6_ADDR_0,
}
#[cfg(feature = "shared")]
impl Default for IN6_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union IN6_ADDR_0 {
    pub Byte: [super::UCHAR; 16],
    pub Word: [super::USHORT; 8],
}
#[cfg(feature = "shared")]
impl Default for IN6_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type LPIN6_ADDR = *mut IN6_ADDR;
#[cfg(feature = "shared")]
pub type PIN6_ADDR = *mut IN6_ADDR;
