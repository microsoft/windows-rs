pub type NativePtr = *const u8;
pub type NativePtrAlias = NativePtr;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Struct {
    pub field: NativePtrAlias,
    pub other: i32,
}
