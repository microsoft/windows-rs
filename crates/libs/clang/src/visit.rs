#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct VisitResult(i32);

impl VisitResult {
    pub const Break: Self = Self(0);
    pub const Continue: Self = Self(1);
    pub const Recurse: Self = Self(2);
}
