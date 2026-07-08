windows_core::link!("test.dll" "system" fn GetProc() -> Proc);
#[cfg(target_arch = "x86")]
pub type Proc = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type Proc = Option<unsafe extern "system" fn() -> isize>;
