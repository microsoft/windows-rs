#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchNest {
    pub Anonymous: ArchNest_0,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchNest_0 {
    pub value: u32,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchNest {
    pub Anonymous: ArchNest_0,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchNest_0 {
    pub value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Outer {
    pub header: u32,
    pub Anonymous: Outer_0,
    pub tail: u16,
}
impl Default for Outer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union Outer_0 {
    pub integer: i32,
    pub nested: Outer_0_0,
}
impl Default for Outer_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Outer_0_0 {
    pub value: f32,
}
