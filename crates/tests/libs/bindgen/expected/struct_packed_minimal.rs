pub type USHORT = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub union Version {
    pub value: u64,
    pub parts: Version_Parts,
}
impl Default for Version {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Version_Parts {
    pub revision: USHORT,
    pub build: USHORT,
    pub minor: USHORT,
    pub major: USHORT,
}
