#[repr(C, packed(8))]
#[derive(Clone, Copy)]
pub struct FILE {
    pub _Placeholder: *mut core::ffi::c_void,
}
impl Default for FILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
