#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifierEnumerateResource(process: super::super::Foundation::HANDLE, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext: *mut ::core::ffi::c_void) -> u32;
}
#[repr(C)]
pub struct AVRF_BACKTRACE_INFORMATION {
    pub Depth: u32,
    pub Index: u32,
    pub ReturnAddresses: [u64; 32],
}
impl ::core::marker::Copy for AVRF_BACKTRACE_INFORMATION {}
impl ::core::clone::Clone for AVRF_BACKTRACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
#[repr(C)]
pub struct AVRF_HANDLE_OPERATION {
    pub Handle: u64,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub OperationType: u32,
    pub Spare0: u32,
    pub BackTraceInformation: AVRF_BACKTRACE_INFORMATION,
}
impl ::core::marker::Copy for AVRF_HANDLE_OPERATION {}
impl ::core::clone::Clone for AVRF_HANDLE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
#[repr(C)]
pub struct AVRF_HEAP_ALLOCATION {
    pub HeapHandle: u64,
    pub UserAllocation: u64,
    pub UserAllocationSize: u64,
    pub Allocation: u64,
    pub AllocationSize: u64,
    pub UserAllocationState: u32,
    pub HeapState: u32,
    pub HeapContext: u64,
    pub BackTraceInformation: *mut AVRF_BACKTRACE_INFORMATION,
}
impl ::core::marker::Copy for AVRF_HEAP_ALLOCATION {}
impl ::core::clone::Clone for AVRF_HEAP_ALLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AVRF_MAX_TRACES: u32 = 32u32;
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = unsafe extern "system" fn(resourcedescription: *mut ::core::ffi::c_void, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: u32 = 2u32;
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: u32 = 1u32;
pub const AvrfResourceHeapAllocation: i32 = 0i32;
pub const AvrfResourceHandleTrace: i32 = 1i32;
pub const AvrfResourceMax: i32 = 2i32;
pub const OperationDbUnused: i32 = 0i32;
pub const OperationDbOPEN: i32 = 1i32;
pub const OperationDbCLOSE: i32 = 2i32;
pub const OperationDbBADREF: i32 = 3i32;
pub const HeapFullPageHeap: i32 = 1073741824i32;
pub const HeapMetadata: i32 = -2147483648i32;
pub const HeapStateMask: i32 = -65536i32;
pub const HeapEnumerationEverything: i32 = 0i32;
pub const HeapEnumerationStop: i32 = -1i32;
pub const AllocationStateUnknown: i32 = 0i32;
pub const AllocationStateBusy: i32 = 1i32;
pub const AllocationStateFree: i32 = 2i32;
