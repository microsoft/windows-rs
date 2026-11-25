#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for CERTTRANSBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: super::DWORD,
    pub SeekOperator: super::LONG,
    pub SortOrder: super::LONG,
    pub pbValue: *mut super::BYTE,
    pub cbValue: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for CERTVIEWRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
