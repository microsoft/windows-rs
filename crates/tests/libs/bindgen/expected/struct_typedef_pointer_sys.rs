pub type NativePtr = *const u8;
pub type NativePtrAlias = NativePtr;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Struct {
    pub field: NativePtrAlias,
    pub other: i32,
}
impl Default for Struct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
