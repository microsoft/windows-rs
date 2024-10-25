pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = 5u32;
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = 2u32;
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = 3u32;
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = 4u32;
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = 1i32;
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = 0i32;
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = 2i32;
pub const COMPRESS_RAW: u32 = 536870912u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COMPRESS_ALGORITHM(pub u32);
impl windows_core::TypeKind for COMPRESS_ALGORITHM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for COMPRESS_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COMPRESS_ALLOCATION_ROUTINES {
    type TypeKind = windows_core::CloneType;
}
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_COMPRESS_FREE = Option<unsafe extern "system" fn(usercontext: *const core::ffi::c_void, memory: *const core::ffi::c_void)>;
