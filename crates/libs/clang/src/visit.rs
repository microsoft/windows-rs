#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Visit(i32);

impl Visit {
    pub const Break: Self = Self(0);
    pub const Continue: Self = Self(1);
    pub const Recurse: Self = Self(2);
}
