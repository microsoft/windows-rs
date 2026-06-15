#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: Self = Self(0);
    pub const SuppressExceptions: Self = Self(1);
    pub const ForceExceptions: Self = Self(2);
    pub const UseSetErrorInfo: Self = Self(4);
    pub const SuppressSetErrorInfo: Self = Self(8);
}
impl ErrorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
