#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_CALLBACK(pub *mut core::ffi::c_void);
impl Default for HCS_CALLBACK {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}
impl HCS_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
