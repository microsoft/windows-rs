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
#[repr(transparent)]
pub struct VERIFIER_ENUM_RESOURCE_FLAGS(pub u32);
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(2u32);
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(1u32);
impl ::core::marker::Copy for VERIFIER_ENUM_RESOURCE_FLAGS {}
impl ::core::clone::Clone for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct eAvrfResourceTypes(pub i32);
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = eAvrfResourceTypes(0i32);
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = eAvrfResourceTypes(1i32);
pub const AvrfResourceMax: eAvrfResourceTypes = eAvrfResourceTypes(2i32);
impl ::core::marker::Copy for eAvrfResourceTypes {}
impl ::core::clone::Clone for eAvrfResourceTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct eHANDLE_TRACE_OPERATIONS(pub i32);
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(0i32);
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(1i32);
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(2i32);
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(3i32);
impl ::core::marker::Copy for eHANDLE_TRACE_OPERATIONS {}
impl ::core::clone::Clone for eHANDLE_TRACE_OPERATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct eHeapAllocationState(pub i32);
pub const HeapFullPageHeap: eHeapAllocationState = eHeapAllocationState(1073741824i32);
pub const HeapMetadata: eHeapAllocationState = eHeapAllocationState(-2147483648i32);
pub const HeapStateMask: eHeapAllocationState = eHeapAllocationState(-65536i32);
impl ::core::marker::Copy for eHeapAllocationState {}
impl ::core::clone::Clone for eHeapAllocationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct eHeapEnumerationLevel(pub i32);
pub const HeapEnumerationEverything: eHeapEnumerationLevel = eHeapEnumerationLevel(0i32);
pub const HeapEnumerationStop: eHeapEnumerationLevel = eHeapEnumerationLevel(-1i32);
impl ::core::marker::Copy for eHeapEnumerationLevel {}
impl ::core::clone::Clone for eHeapEnumerationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct eUserAllocationState(pub i32);
pub const AllocationStateUnknown: eUserAllocationState = eUserAllocationState(0i32);
pub const AllocationStateBusy: eUserAllocationState = eUserAllocationState(1i32);
pub const AllocationStateFree: eUserAllocationState = eUserAllocationState(2i32);
impl ::core::marker::Copy for eUserAllocationState {}
impl ::core::clone::Clone for eUserAllocationState {
    fn clone(&self) -> Self {
        *self
    }
}
