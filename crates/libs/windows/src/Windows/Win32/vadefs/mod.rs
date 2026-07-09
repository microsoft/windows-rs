#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct va_list(pub *mut i8);
impl va_list {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for va_list {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
