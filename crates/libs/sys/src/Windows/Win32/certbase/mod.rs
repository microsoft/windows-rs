#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: super::DWORD,
    pub SeekOperator: super::LONG,
    pub SortOrder: super::LONG,
    pub pbValue: *mut super::BYTE,
    pub cbValue: super::DWORD,
}
