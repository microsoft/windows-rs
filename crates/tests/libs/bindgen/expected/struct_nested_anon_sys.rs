#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ArchNest {
    pub flags: u32,
    pub Anonymous: ArchNest_0,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ArchNest_0 {
    pub lo: u32,
    pub hi: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ArchUnionNest {
    pub kind: u32,
    pub Anonymous: ArchUnionNest_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ArchUnionNest {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union ArchUnionNest_0 {
    pub value: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ArchUnionNest_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeepNest {
    pub a: u32,
    pub Anonymous: DeepNest_0,
}
impl Default for DeepNest {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeepNest_0 {
    pub b: u32,
    pub Anonymous: DeepNest_0_0,
}
impl Default for DeepNest_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeepNest_0_0 {
    pub c: i32,
    pub d: f32,
}
impl Default for DeepNest_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NamedNest {
    pub Anonymous: NamedNest_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NamedNest_0 {
    pub point: Point,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OrderNest {
    pub Zed: OrderNest_0,
    pub Alpha: OrderNest_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OrderNest_0 {
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OrderNest_1 {
    pub value: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Outer {
    pub header: u32,
    pub Anonymous: Outer_0,
    pub tail: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Outer_0 {
    pub x: i32,
    pub y: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PackedNest {
    pub a: u8,
    pub Anonymous: PackedNest_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PackedNest_0 {
    pub b: i32,
    pub c: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WithUnion {
    pub kind: u32,
    pub Anonymous: WithUnion_0,
}
impl Default for WithUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WithUnion_0 {
    pub as_int: i32,
    pub as_float: f32,
}
impl Default for WithUnion_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
