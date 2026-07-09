#[cfg(target_arch = "x86")]
pub type ArchCallback = Option<unsafe extern "system" fn(code: i16) -> i16>;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type ArchCallback = Option<unsafe extern "system" fn(code: i32) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UsesCallback {
    pub handler: ArchCallback,
}
