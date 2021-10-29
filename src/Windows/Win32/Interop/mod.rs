#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Architecture(pub i32);
pub const None: Architecture = Architecture(0i32);
pub const X86: Architecture = Architecture(1i32);
pub const X64: Architecture = Architecture(2i32);
pub const Arm64: Architecture = Architecture(4i32);
pub const All: Architecture = Architecture(7i32);
impl ::std::convert::From<i32> for Architecture {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Architecture {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for Architecture {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for Architecture {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for Architecture {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for Architecture {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for Architecture {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
