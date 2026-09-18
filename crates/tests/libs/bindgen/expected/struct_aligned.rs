#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayFieldAligned {
    pub First: u8,
    pub Reserved: [u8; 3],
    pub _alignment: [u64; 0],
    pub Value: u32,
}
impl Default for ArrayFieldAligned {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type Callback = Option<unsafe extern "system" fn()>;
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CallbackFieldAligned {
    pub Callback: Callback,
    pub First: u32,
    pub _alignment: [u64; 0],
    pub Value: u32,
}
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FieldAligned {
    pub First: u32,
    pub _alignment: [u64; 0],
    pub Second: u32,
    pub _alignment2: [u64; 0],
    pub Third: u32,
    pub Pointer: *mut core::ffi::c_void,
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
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerFieldAligned {
    pub Pointer: *mut core::ffi::c_void,
    pub First: u32,
    pub _alignment: [u64; 0],
    pub Value: u32,
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WideFieldAligned {
    pub _alignment: u8,
    pub First: u32,
    pub _alignment2: [u128; 0],
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmmFrame {
    pub Xmm0: M128A,
    pub Flags: u32,
}
