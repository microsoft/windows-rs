pub const Execute: Flags = Flags(4);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Flags(pub u32);
impl Flags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const None: Flags = Flags(0);
pub const Read: Flags = Flags(1);
pub const Write: Flags = Flags(2);
