#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color(pub i32);
impl Color {
    pub const Red: Self = Self(1);
    pub const Green: Self = Self(2);
    pub const Blue: Self = Self(3);
}
