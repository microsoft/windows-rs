#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayFieldAligned {
    pub First: u8,
    pub Reserved: [u8; 3],
    pub _padding: [u8; 4],
    pub Value: u32,
}
impl Default for ArrayFieldAligned {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldAligned {
    pub First: u32,
    pub _padding: [u8; 4],
    pub Second: u32,
    pub _padding2: [u8; 4],
    pub Third: u32,
    pub Pointer: *mut core::ffi::c_void,
}
impl Default for FieldAligned {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverAligned {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmmFrame {
    pub Xmm0: M128A,
    pub Flags: u32,
}
