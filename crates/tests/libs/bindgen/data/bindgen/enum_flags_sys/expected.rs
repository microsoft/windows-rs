#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: Self = Self(0);
    pub const SuppressExceptions: Self = Self(1);
    pub const ForceExceptions: Self = Self(2);
    pub const UseSetErrorInfo: Self = Self(4);
    pub const SuppressSetErrorInfo: Self = Self(8);
}
