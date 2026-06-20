#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Plain {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
