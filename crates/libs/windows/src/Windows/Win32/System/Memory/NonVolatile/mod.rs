#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NV_MEMORY_RANGE {
    type TypeKind = windows_core::CopyType;
}
