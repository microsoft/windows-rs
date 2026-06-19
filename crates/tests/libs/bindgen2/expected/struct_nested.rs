#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Outer {
    pub position: Inner,
    pub size: Inner,
}
