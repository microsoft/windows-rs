pub const APC_LEVEL: i32 = 1;
pub const DISPATCH_LEVEL: i32 = 2;
#[cfg(target_arch = "x86")]
pub const HIGH_LEVEL: i32 = 31;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HIGH_LEVEL: i32 = 15;
pub const PASSIVE_LEVEL: i32 = 0;
