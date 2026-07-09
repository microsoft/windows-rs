#[repr(C)]
#[derive(Clone, Copy)]
pub struct IN_ADDR {
    pub S_un: IN_ADDR_0,
}
impl Default for IN_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IN_ADDR_0 {
    pub S_un_b: IN_ADDR_0_0,
    pub S_un_w: IN_ADDR_0_1,
    pub S_addr: u32,
}
impl Default for IN_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IN_ADDR_0_0 {
    pub s_b1: u8,
    pub s_b2: u8,
    pub s_b3: u8,
    pub s_b4: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IN_ADDR_0_1 {
    pub s_w1: u16,
    pub s_w2: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIN_ADDR(pub *mut IN_ADDR);
impl LPIN_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPIN_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIN_ADDR(pub *mut IN_ADDR);
impl PIN_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIN_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
