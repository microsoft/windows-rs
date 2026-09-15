pub const CONST_PTR: *const ITEM = core::ptr::without_provenance::<ITEM>((-1i32) as usize);
pub const INVALID_SOCKET: SOCKET = 18446744073709551615u64 as usize;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ITEM {
    pub value: i32,
}
pub const MUT_PTR: *mut ITEM = core::ptr::without_provenance_mut::<ITEM>(1usize);
#[cfg(target_arch = "x86")]
pub const PTR_MAX: usize = 4294967295;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub const PTR_MAX: usize = 18446744073709551615u64 as usize;
pub const PTR_SENTINEL: usize = 18446744073709551615u64 as usize;
pub const SIGNED_FROM_U32: isize = 4294967295u32 as isize;
pub const SIGNED_FROM_U64: isize = 18446744073709551615u64 as isize;
#[cfg(target_arch = "x86")]
pub const SIGNED_MAX: isize = 2147483647;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub const SIGNED_MAX: isize = 9223372036854775807i64 as isize;
pub const SIGNED_SENTINEL: isize = -1;
pub const SMALL_ISIZE: isize = -4096;
pub const SMALL_USIZE: usize = 4096;
pub type SOCKET = usize;
pub const UNSIGNED_FROM_I32: usize = -1i32 as usize;
pub const UNSIGNED_FROM_I64: usize = -1i64 as usize;
pub const VOID_PTR: *mut core::ffi::c_void =
    core::ptr::without_provenance_mut::<core::ffi::c_void>(0usize);
