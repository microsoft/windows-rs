#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IN_ADDR {
    pub S_un: IN_ADDR_0,
}
#[cfg(feature = "shared")]
impl Default for IN_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union IN_ADDR_0 {
    pub S_un_b: IN_ADDR_0_0,
    pub S_un_w: IN_ADDR_0_1,
    pub S_addr: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IN_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IN_ADDR_0_0 {
    pub s_b1: super::UCHAR,
    pub s_b2: super::UCHAR,
    pub s_b3: super::UCHAR,
    pub s_b4: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IN_ADDR_0_1 {
    pub s_w1: super::USHORT,
    pub s_w2: super::USHORT,
}
#[cfg(feature = "shared")]
pub type LPIN_ADDR = *mut IN_ADDR;
#[cfg(feature = "shared")]
pub type PIN_ADDR = *mut IN_ADDR;
