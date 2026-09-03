#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Double(pub f64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Float(pub f32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Handle(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Integer(pub i32);
