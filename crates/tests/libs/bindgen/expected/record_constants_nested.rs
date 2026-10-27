#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchHolder(pub u8);
#[cfg(target_arch = "x86")]
impl ArchHolder {
    pub const Value: ArchHolder_E = -1;
    pub const First: i32 = -1;
}
#[cfg(target_arch = "x86")]
pub type ArchHolder_E = i32;
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchHolder(pub u8);
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl ArchHolder {
    pub const Value: ArchHolder_E = 4294967295;
    pub const First: u32 = 4294967295;
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type ArchHolder_E = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Consumer {
    pub kind: Wrapper_Kind,
    pub state: Holder_E,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Holder(pub u8);
impl Holder {
    pub const Value: Holder_E = Holder_E(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Holder_E(pub u32);
impl Holder_E {
    pub const One: Self = Self(1);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Outer {
    pub data: i32,
}
impl Outer {
    #[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
    pub const Default: Outer_Mode = Outer_Mode(1);
    #[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
    pub const Enabled: i32 = 1;
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type Outer_E = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Outer_Mode(pub u32);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Outer_Mode {
    pub const Read: Self = Self(1);
    pub const Write: Self = Self(2);
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Outer_Mode {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl core::ops::BitOr for Outer_Mode {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl core::ops::BitAnd for Outer_Mode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl core::ops::BitOrAssign for Outer_Mode {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl core::ops::BitAndAssign for Outer_Mode {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl core::ops::Not for Outer_Mode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Wrapper {
    pub Value: i32,
}
impl Wrapper {
    pub const One: i32 = 1;
}
pub type Wrapper_Kind = i32;
