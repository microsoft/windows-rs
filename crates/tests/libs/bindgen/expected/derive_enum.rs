#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Enum(pub i32);
pub const First: Enum = Enum(1);
pub const Second: Enum = Enum(2);
