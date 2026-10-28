#[cfg(target_arch = "x86")]
windows_link::link!("test.dll" "system" fn GetReached(index : Shared, record : *const Record) -> Shared);
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_link::link!("test.dll" "system" fn GetReached(index : Shared) -> Wide);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct Record {
    pub value: i32,
}
pub type Shared = i32;
pub type Wide = isize;
