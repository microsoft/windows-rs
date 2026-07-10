#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut u8,
}
impl Default for CERTTRANSBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: u32,
    pub SeekOperator: i32,
    pub SortOrder: i32,
    pub pbValue: *mut u8,
    pub cbValue: u32,
}
impl Default for CERTVIEWRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
