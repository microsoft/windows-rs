#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct __time64_t(pub i64);
pub type time_t = __time64_t;
