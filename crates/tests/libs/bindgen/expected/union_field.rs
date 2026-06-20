#[repr(C)]
pub union Inner {
    pub offset: u32,
    pub text: core::mem::ManuallyDrop<windows_core::HSTRING>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for Inner {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Outer {
    pub kind: u32,
    pub value: Inner,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for Outer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
