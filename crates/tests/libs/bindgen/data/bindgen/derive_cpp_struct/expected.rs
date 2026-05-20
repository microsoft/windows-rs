#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
