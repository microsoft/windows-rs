#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockFirst: LockNumber = 0 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockLast: LockNumber = 13 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LockNumber(pub u64);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type LockNumber = i32;
