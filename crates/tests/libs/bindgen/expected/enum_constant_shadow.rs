#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const Aligned: Pool = 4;
#[cfg(target_arch = "aarch64")]
pub const Aligned: Pool = 516;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const Default: Pool = 0;
#[cfg(target_arch = "aarch64")]
pub const Default: Pool = 512;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type Partial = i32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PartialValue: Partial = 20;
#[cfg(target_arch = "aarch64")]
pub const PartialValue: Partial = 21;
pub type Pool = i32;
pub const Replaced: i32 = 10;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scoped(pub i32);
impl Scoped {
    pub const ScopedValue: Self = Self(40);
}
pub const ScopedValue: i32 = 41;
pub const Unshadowed: Pool = 8;
