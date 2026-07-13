pub type Enum = i32;
pub const First: Enum = 1;
pub const Second: Enum = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
