#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
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
impl ::core::fmt::Debug for AVRF_BACKTRACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVRF_BACKTRACE_INFORMATION").field("Depth", &self.Depth).field("Index", &self.Index).field("ReturnAddresses", &self.ReturnAddresses).finish()
    }
}
unsafe impl ::windows::core::Abi for AVRF_BACKTRACE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AVRF_BACKTRACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVRF_BACKTRACE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AVRF_BACKTRACE_INFORMATION {}
impl ::core::default::Default for AVRF_BACKTRACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
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
impl ::core::fmt::Debug for AVRF_HANDLE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVRF_HANDLE_OPERATION").field("Handle", &self.Handle).field("ProcessId", &self.ProcessId).field("ThreadId", &self.ThreadId).field("OperationType", &self.OperationType).field("Spare0", &self.Spare0).field("BackTraceInformation", &self.BackTraceInformation).finish()
    }
}
unsafe impl ::windows::core::Abi for AVRF_HANDLE_OPERATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AVRF_HANDLE_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVRF_HANDLE_OPERATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AVRF_HANDLE_OPERATION {}
impl ::core::default::Default for AVRF_HANDLE_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
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
impl ::core::fmt::Debug for AVRF_HEAP_ALLOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVRF_HEAP_ALLOCATION").field("HeapHandle", &self.HeapHandle).field("UserAllocation", &self.UserAllocation).field("UserAllocationSize", &self.UserAllocationSize).field("Allocation", &self.Allocation).field("AllocationSize", &self.AllocationSize).field("UserAllocationState", &self.UserAllocationState).field("HeapState", &self.HeapState).field("HeapContext", &self.HeapContext).field("BackTraceInformation", &self.BackTraceInformation).finish()
    }
}
unsafe impl ::windows::core::Abi for AVRF_HEAP_ALLOCATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AVRF_HEAP_ALLOCATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVRF_HEAP_ALLOCATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AVRF_HEAP_ALLOCATION {}
impl ::core::default::Default for AVRF_HEAP_ALLOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AVRF_MAX_TRACES: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(resourcedescription: *mut ::core::ffi::c_void, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type VERIFIER_ENUM_RESOURCE_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifierEnumerateResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifierEnumerateResource(process: super::super::Foundation::HANDLE, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: ::windows::core::RawPtr, enumerationcontext: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(VerifierEnumerateResource(process.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(resourcetype), ::core::mem::transmute(resourcecallback), ::core::mem::transmute(enumerationcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type eAvrfResourceTypes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AvrfResourceMax: eAvrfResourceTypes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type eHANDLE_TRACE_OPERATIONS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type eHeapAllocationState = i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const HeapFullPageHeap: eHeapAllocationState = 1073741824i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const HeapMetadata: eHeapAllocationState = -2147483648i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const HeapStateMask: eHeapAllocationState = -65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type eHeapEnumerationLevel = i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const HeapEnumerationEverything: eHeapEnumerationLevel = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const HeapEnumerationStop: eHeapEnumerationLevel = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub type eUserAllocationState = i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AllocationStateUnknown: eUserAllocationState = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AllocationStateBusy: eUserAllocationState = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationVerifier'*"]
pub const AllocationStateFree: eUserAllocationState = 2i32;
