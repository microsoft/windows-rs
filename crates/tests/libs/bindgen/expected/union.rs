#[repr(C)]
#[derive(Clone, Copy)]
pub union Value {
    pub i: i32,
    pub f: f32,
    pub p: *mut u8,
}
impl Default for Value {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
