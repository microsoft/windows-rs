#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifierEnumerateResource(process: super::super::Foundation::HANDLE, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext: *mut ::core::ffi::c_void) -> u32;
}
#[repr(C)]
pub struct AVRF_BACKTRACE_INFORMATION(i32);
#[repr(C)]
pub struct AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK(i32);
#[repr(C)]
pub struct AVRF_HANDLE_OPERATION(i32);
#[repr(C)]
pub struct AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK(i32);
#[repr(C)]
pub struct AVRF_HEAP_ALLOCATION(i32);
pub const AVRF_MAX_TRACES: u32 = 32u32;
#[repr(C)]
pub struct AVRF_RESOURCE_ENUMERATE_CALLBACK(i32);
#[repr(C)]
pub struct VERIFIER_ENUM_RESOURCE_FLAGS(i32);
#[repr(C)]
pub struct eAvrfResourceTypes(i32);
#[repr(C)]
pub struct eHANDLE_TRACE_OPERATIONS(i32);
#[repr(C)]
pub struct eHeapAllocationState(i32);
#[repr(C)]
pub struct eHeapEnumerationLevel(i32);
#[repr(C)]
pub struct eUserAllocationState(i32);
