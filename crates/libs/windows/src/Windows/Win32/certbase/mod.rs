#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: u32,
    pub SeekOperator: i32,
    pub SortOrder: i32,
    pub pbValue: *mut u8,
    pub cbValue: u32,
}
