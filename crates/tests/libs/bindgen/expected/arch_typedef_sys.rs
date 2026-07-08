#[cfg(target_arch = "x86")]
pub type ArchScalar = i16;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type ArchScalar = i32;
