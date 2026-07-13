#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type ArchEnum = i32;
#[cfg(target_arch = "aarch64")]
pub type ArchEnum = i32;
#[cfg(target_arch = "aarch64")]
pub const Arm64Only: ArchEnum = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const First: ArchEnum = 1;
#[cfg(target_arch = "aarch64")]
pub const First: ArchEnum = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const X64Only: ArchEnum = 4;
