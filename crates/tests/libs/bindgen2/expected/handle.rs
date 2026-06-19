#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle(pub *mut core::ffi::c_void);
impl Handle {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for Handle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INVALID: Handle = Handle(-1 as _);
