#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub struct AVRF_BACKTRACE_INFORMATION {
    pub Depth: u32,
    pub Index: u32,
    pub ReturnAddresses: [u64; 32],
}
impl AVRF_BACKTRACE_INFORMATION {}
impl ::core::default::Default for AVRF_BACKTRACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AVRF_BACKTRACE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AVRF_BACKTRACE_INFORMATION").field("Depth", &self.Depth).field("Index", &self.Index).field("ReturnAddresses", &self.ReturnAddresses).finish()
    }
}
impl ::core::cmp::PartialEq for AVRF_BACKTRACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Depth == other.Depth && self.Index == other.Index && self.ReturnAddresses == other.ReturnAddresses
    }
}
impl ::core::cmp::Eq for AVRF_BACKTRACE_INFORMATION {}
unsafe impl ::windows::core::Abi for AVRF_BACKTRACE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub struct AVRF_HANDLE_OPERATION {
    pub Handle: u64,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub OperationType: u32,
    pub Spare0: u32,
    pub BackTraceInformation: AVRF_BACKTRACE_INFORMATION,
}
impl AVRF_HANDLE_OPERATION {}
impl ::core::default::Default for AVRF_HANDLE_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AVRF_HANDLE_OPERATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AVRF_HANDLE_OPERATION").field("Handle", &self.Handle).field("ProcessId", &self.ProcessId).field("ThreadId", &self.ThreadId).field("OperationType", &self.OperationType).field("Spare0", &self.Spare0).field("BackTraceInformation", &self.BackTraceInformation).finish()
    }
}
impl ::core::cmp::PartialEq for AVRF_HANDLE_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.ProcessId == other.ProcessId && self.ThreadId == other.ThreadId && self.OperationType == other.OperationType && self.Spare0 == other.Spare0 && self.BackTraceInformation == other.BackTraceInformation
    }
}
impl ::core::cmp::Eq for AVRF_HANDLE_OPERATION {}
unsafe impl ::windows::core::Abi for AVRF_HANDLE_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
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
impl AVRF_HEAP_ALLOCATION {}
impl ::core::default::Default for AVRF_HEAP_ALLOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AVRF_HEAP_ALLOCATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AVRF_HEAP_ALLOCATION")
            .field("HeapHandle", &self.HeapHandle)
            .field("UserAllocation", &self.UserAllocation)
            .field("UserAllocationSize", &self.UserAllocationSize)
            .field("Allocation", &self.Allocation)
            .field("AllocationSize", &self.AllocationSize)
            .field("UserAllocationState", &self.UserAllocationState)
            .field("HeapState", &self.HeapState)
            .field("HeapContext", &self.HeapContext)
            .field("BackTraceInformation", &self.BackTraceInformation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVRF_HEAP_ALLOCATION {
    fn eq(&self, other: &Self) -> bool {
        self.HeapHandle == other.HeapHandle && self.UserAllocation == other.UserAllocation && self.UserAllocationSize == other.UserAllocationSize && self.Allocation == other.Allocation && self.AllocationSize == other.AllocationSize && self.UserAllocationState == other.UserAllocationState && self.HeapState == other.HeapState && self.HeapContext == other.HeapContext && self.BackTraceInformation == other.BackTraceInformation
    }
}
impl ::core::cmp::Eq for AVRF_HEAP_ALLOCATION {}
unsafe impl ::windows::core::Abi for AVRF_HEAP_ALLOCATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub const AVRF_MAX_TRACES: u32 = 32u32;
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = unsafe extern "system" fn(resourcedescription: *mut ::core::ffi::c_void, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32;
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VERIFIER_ENUM_RESOURCE_FLAGS(pub u32);
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(2u32);
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(1u32);
impl ::core::convert::From<u32> for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifierEnumerateResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: ::core::option::Option<AVRF_RESOURCE_ENUMERATE_CALLBACK>, enumerationcontext: *mut ::core::ffi::c_void) -> u32 {
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
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct eAvrfResourceTypes(pub i32);
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = eAvrfResourceTypes(0i32);
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = eAvrfResourceTypes(1i32);
pub const AvrfResourceMax: eAvrfResourceTypes = eAvrfResourceTypes(2i32);
impl ::core::convert::From<i32> for eAvrfResourceTypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for eAvrfResourceTypes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct eHANDLE_TRACE_OPERATIONS(pub i32);
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(0i32);
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(1i32);
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(2i32);
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(3i32);
impl ::core::convert::From<i32> for eHANDLE_TRACE_OPERATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for eHANDLE_TRACE_OPERATIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct eHeapAllocationState(pub i32);
pub const HeapFullPageHeap: eHeapAllocationState = eHeapAllocationState(1073741824i32);
pub const HeapMetadata: eHeapAllocationState = eHeapAllocationState(-2147483648i32);
pub const HeapStateMask: eHeapAllocationState = eHeapAllocationState(-65536i32);
impl ::core::convert::From<i32> for eHeapAllocationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for eHeapAllocationState {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct eHeapEnumerationLevel(pub i32);
pub const HeapEnumerationEverything: eHeapEnumerationLevel = eHeapEnumerationLevel(0i32);
pub const HeapEnumerationStop: eHeapEnumerationLevel = eHeapEnumerationLevel(-1i32);
impl ::core::convert::From<i32> for eHeapEnumerationLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for eHeapEnumerationLevel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct eUserAllocationState(pub i32);
pub const AllocationStateUnknown: eUserAllocationState = eUserAllocationState(0i32);
pub const AllocationStateBusy: eUserAllocationState = eUserAllocationState(1i32);
pub const AllocationStateFree: eUserAllocationState = eUserAllocationState(2i32);
impl ::core::convert::From<i32> for eUserAllocationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for eUserAllocationState {
    type Abi = Self;
}
