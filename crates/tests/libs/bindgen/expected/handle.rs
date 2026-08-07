#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Handle(pub *mut core::ffi::c_void);
pub const INVALID: Handle = Handle(-1 as _);
