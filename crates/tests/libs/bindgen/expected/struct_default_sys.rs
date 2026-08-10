#[repr(C)]
#[derive(Clone, Copy)]
pub struct AliasArray {
    pub values: BigArrayAlias,
}
impl Default for AliasArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BigArray = [u16; 64];
pub type BigArrayAlias = BigArray;
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color(pub i32);
impl Color {
    pub const Red: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirectArray {
    pub values: [u32; 4],
}
impl Default for DirectArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Plain {
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Scoped {
    pub color: Color,
}
impl Default for Scoped {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnionField {
    pub value: Value,
}
impl Default for UnionField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union Value {
    pub number: u32,
}
impl Default for Value {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
