pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = 2u32;
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = 1u32;
pub const AVRF_MAX_TRACES: u32 = 32u32;
pub const AllocationStateBusy: eUserAllocationState = 1i32;
pub const AllocationStateFree: eUserAllocationState = 2i32;
pub const AllocationStateUnknown: eUserAllocationState = 0i32;
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = 1i32;
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = 0i32;
pub const AvrfResourceMax: eAvrfResourceTypes = 2i32;
pub const HeapEnumerationEverything: eHeapEnumerationLevel = 0i32;
pub const HeapEnumerationStop: eHeapEnumerationLevel = -1i32;
pub const HeapFullPageHeap: eHeapAllocationState = 1073741824i32;
pub const HeapMetadata: eHeapAllocationState = -2147483648i32;
pub const HeapStateMask: eHeapAllocationState = -65536i32;
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = 3i32;
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = 2i32;
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = 1i32;
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VERIFIER_ENUM_RESOURCE_FLAGS(pub u32);
impl windows_core::TypeKind for VERIFIER_ENUM_RESOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct eAvrfResourceTypes(pub i32);
impl windows_core::TypeKind for eAvrfResourceTypes {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct eHANDLE_TRACE_OPERATIONS(pub i32);
impl windows_core::TypeKind for eHANDLE_TRACE_OPERATIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct eHeapAllocationState(pub i32);
impl windows_core::TypeKind for eHeapAllocationState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct eHeapEnumerationLevel(pub i32);
impl windows_core::TypeKind for eHeapEnumerationLevel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct eUserAllocationState(pub i32);
impl windows_core::TypeKind for eUserAllocationState {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVRF_BACKTRACE_INFORMATION {
    pub Depth: u32,
    pub Index: u32,
    pub ReturnAddresses: [u64; 32],
}
impl Default for AVRF_BACKTRACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVRF_BACKTRACE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVRF_HANDLE_OPERATION {
    pub Handle: u64,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub OperationType: u32,
    pub Spare0: u32,
    pub BackTraceInformation: AVRF_BACKTRACE_INFORMATION,
}
impl Default for AVRF_HANDLE_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVRF_HANDLE_OPERATION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for AVRF_HEAP_ALLOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVRF_HEAP_ALLOCATION {
    type TypeKind = windows_core::CopyType;
}
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(resourcedescription: *mut core::ffi::c_void, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
