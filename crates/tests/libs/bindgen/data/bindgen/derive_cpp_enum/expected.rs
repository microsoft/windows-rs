#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct WAIT_EVENT(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_ERROR(pub u32);
