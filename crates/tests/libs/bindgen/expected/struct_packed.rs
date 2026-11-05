pub type Callback = Option<unsafe extern "system" fn(value: u32)>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Floating {
    pub tag: u8,
    pub value: f64,
}
#[repr(C, packed(1))]
#[derive(Default)]
pub struct NonCopy {
    pub object: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[repr(C)]
#[derive(Default)]
pub struct NonCopyOuter {
    pub value: NonCopy,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Outer {
    pub values: [Packed; 2],
}
impl Default for Outer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Packed {
    pub tag: u8,
    pub value: u64,
    pub words: [u32; 3],
}
impl Default for Packed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WithCallback {
    pub callback: Callback,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WithUnion {
    pub tag: u8,
    pub value: WithUnion_0,
}
impl Default for WithUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WithUnion_0 {
    pub integer: u32,
    pub floating: f32,
}
impl Default for WithUnion_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
