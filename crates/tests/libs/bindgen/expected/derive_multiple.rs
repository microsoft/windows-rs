#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum(pub i32);
pub const First: Enum = Enum(1);
pub const Second: Enum = Enum(2);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
