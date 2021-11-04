#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddERExcludedApplicationA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(szapplication: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddERExcludedApplicationA(szapplication: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddERExcludedApplicationA(szapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddERExcludedApplicationW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(wszapplication: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddERExcludedApplicationW(wszapplication: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddERExcludedApplicationW(wszapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EFaultRepRetVal(pub i32);
pub const frrvOk: EFaultRepRetVal = EFaultRepRetVal(0i32);
pub const frrvOkManifest: EFaultRepRetVal = EFaultRepRetVal(1i32);
pub const frrvOkQueued: EFaultRepRetVal = EFaultRepRetVal(2i32);
pub const frrvErr: EFaultRepRetVal = EFaultRepRetVal(3i32);
pub const frrvErrNoDW: EFaultRepRetVal = EFaultRepRetVal(4i32);
pub const frrvErrTimeout: EFaultRepRetVal = EFaultRepRetVal(5i32);
pub const frrvLaunchDebugger: EFaultRepRetVal = EFaultRepRetVal(6i32);
pub const frrvOkHeadless: EFaultRepRetVal = EFaultRepRetVal(7i32);
pub const frrvErrAnotherInstance: EFaultRepRetVal = EFaultRepRetVal(8i32);
pub const frrvErrNoMemory: EFaultRepRetVal = EFaultRepRetVal(9i32);
pub const frrvErrDoubleFault: EFaultRepRetVal = EFaultRepRetVal(10i32);
impl ::std::convert::From<i32> for EFaultRepRetVal {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EFaultRepRetVal {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HREPORT(pub isize);
impl ::std::default::Default for HREPORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HREPORT {}
unsafe impl ::windows::runtime::Abi for HREPORT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HREPORTSTORE(pub isize);
impl ::std::default::Default for HREPORTSTORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HREPORTSTORE {}
unsafe impl ::windows::runtime::Abi for HREPORTSTORE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = unsafe extern "system" fn(pcontext: *const ::std::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut super::super::Foundation::BOOL, pwszdebuggerlaunch: super::super::Foundation::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = unsafe extern "system" fn(pcontext: *const ::std::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut super::super::Foundation::BOOL, pwszeventname: super::super::Foundation::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = unsafe extern "system" fn(pcontext: *const ::std::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: super::super::Foundation::PWSTR, pchname: *mut u32, pwszvalue: super::super::Foundation::PWSTR, pchvalue: *mut u32) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct REPORT_STORE_TYPES(pub i32);
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(0i32);
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(1i32);
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(2i32);
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(3i32);
pub const E_STORE_INVALID: REPORT_STORE_TYPES = REPORT_STORE_TYPES(4i32);
impl ::std::convert::From<i32> for REPORT_STORE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REPORT_STORE_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal;
        }
        ::std::mem::transmute(ReportFault(::std::mem::transmute(pep), ::std::mem::transmute(dwopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_CONSENT(pub i32);
pub const WerConsentNotAsked: WER_CONSENT = WER_CONSENT(1i32);
pub const WerConsentApproved: WER_CONSENT = WER_CONSENT(2i32);
pub const WerConsentDenied: WER_CONSENT = WER_CONSENT(3i32);
pub const WerConsentAlwaysPrompt: WER_CONSENT = WER_CONSENT(4i32);
pub const WerConsentMax: WER_CONSENT = WER_CONSENT(5i32);
impl ::std::convert::From<i32> for WER_CONSENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_CONSENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_DUMP_CUSTOM_OPTIONS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl WER_DUMP_CUSTOM_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_DUMP_CUSTOM_OPTIONS")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMask == other.dwMask
            && self.dwDumpFlags == other.dwDumpFlags
            && self.bOnlyThisThread == other.bOnlyThisThread
            && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags
            && self.dwOtherThreadFlags == other.dwOtherThreadFlags
            && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags
            && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags
            && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags
            && self.dwOtherModuleFlags == other.dwOtherModuleFlags
            && self.wzPreferredModuleList == other.wzPreferredModuleList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_DUMP_CUSTOM_OPTIONS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_DUMP_CUSTOM_OPTIONS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMask == other.dwMask
            && self.dwDumpFlags == other.dwDumpFlags
            && self.bOnlyThisThread == other.bOnlyThisThread
            && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags
            && self.dwOtherThreadFlags == other.dwOtherThreadFlags
            && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags
            && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags
            && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags
            && self.dwOtherModuleFlags == other.dwOtherModuleFlags
            && self.wzPreferredModuleList == other.wzPreferredModuleList
            && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags
            && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_DUMP_CUSTOM_OPTIONS_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
    pub pvDumpKey: *mut ::std::ffi::c_void,
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub dwThreadID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_DUMP_CUSTOM_OPTIONS_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V3")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .field("pvDumpKey", &self.pvDumpKey)
            .field("hSnapshot", &self.hSnapshot)
            .field("dwThreadID", &self.dwThreadID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMask == other.dwMask
            && self.dwDumpFlags == other.dwDumpFlags
            && self.bOnlyThisThread == other.bOnlyThisThread
            && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags
            && self.dwOtherThreadFlags == other.dwOtherThreadFlags
            && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags
            && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags
            && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags
            && self.dwOtherModuleFlags == other.dwOtherModuleFlags
            && self.wzPreferredModuleList == other.wzPreferredModuleList
            && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags
            && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
            && self.pvDumpKey == other.pvDumpKey
            && self.hSnapshot == other.hSnapshot
            && self.dwThreadID == other.dwThreadID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_DUMP_CUSTOM_OPTIONS_V3 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_MASK_START: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_DUMP_TYPE(pub i32);
pub const WerDumpTypeNone: WER_DUMP_TYPE = WER_DUMP_TYPE(0i32);
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = WER_DUMP_TYPE(1i32);
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = WER_DUMP_TYPE(2i32);
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = WER_DUMP_TYPE(3i32);
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = WER_DUMP_TYPE(4i32);
pub const WerDumpTypeMax: WER_DUMP_TYPE = WER_DUMP_TYPE(5i32);
impl ::std::convert::From<i32> for WER_DUMP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_DUMP_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
pub struct WER_EXCEPTION_INFORMATION {
    pub pExceptionPointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
    pub bClientPointers: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for WER_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for WER_EXCEPTION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_EXCEPTION_INFORMATION").field("pExceptionPointers", &self.pExceptionPointers).field("bClientPointers", &self.bClientPointers).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for WER_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.pExceptionPointers == other.pExceptionPointers && self.bClientPointers == other.bClientPointers
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for WER_EXCEPTION_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_FAULT_REPORTING(pub u32);
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: WER_FAULT_REPORTING = WER_FAULT_REPORTING(4u32);
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: WER_FAULT_REPORTING = WER_FAULT_REPORTING(1u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE: WER_FAULT_REPORTING = WER_FAULT_REPORTING(2u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: WER_FAULT_REPORTING = WER_FAULT_REPORTING(8u32);
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: WER_FAULT_REPORTING = WER_FAULT_REPORTING(16u32);
impl ::std::convert::From<u32> for WER_FAULT_REPORTING {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_FAULT_REPORTING {
    type Abi = Self;
}
impl ::std::ops::BitOr for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WER_FAULT_REPORTING {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WER_FAULT_REPORTING {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WER_FAULT_REPORTING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_FILE(pub u32);
pub const WER_FILE_ANONYMOUS_DATA: WER_FILE = WER_FILE(2u32);
pub const WER_FILE_DELETE_WHEN_DONE: WER_FILE = WER_FILE(1u32);
impl ::std::convert::From<u32> for WER_FILE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_FILE {
    type Abi = Self;
}
impl ::std::ops::BitOr for WER_FILE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WER_FILE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WER_FILE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WER_FILE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WER_FILE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FILE_COMPRESSED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_FILE_TYPE(pub i32);
pub const WerFileTypeMicrodump: WER_FILE_TYPE = WER_FILE_TYPE(1i32);
pub const WerFileTypeMinidump: WER_FILE_TYPE = WER_FILE_TYPE(2i32);
pub const WerFileTypeHeapdump: WER_FILE_TYPE = WER_FILE_TYPE(3i32);
pub const WerFileTypeUserDocument: WER_FILE_TYPE = WER_FILE_TYPE(4i32);
pub const WerFileTypeOther: WER_FILE_TYPE = WER_FILE_TYPE(5i32);
pub const WerFileTypeTriagedump: WER_FILE_TYPE = WER_FILE_TYPE(6i32);
pub const WerFileTypeCustomDump: WER_FILE_TYPE = WER_FILE_TYPE(7i32);
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = WER_FILE_TYPE(8i32);
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = WER_FILE_TYPE(9i32);
pub const WerFileTypeMax: WER_FILE_TYPE = WER_FILE_TYPE(10i32);
impl ::std::convert::From<i32> for WER_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_FILE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P0: u32 = 0u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P3: u32 = 3u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P4: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P5: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P6: u32 = 6u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P7: u32 = 7u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P8: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P9: u32 = 9u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_REGISTER_FILE_TYPE(pub i32);
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(1i32);
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(2i32);
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(3i32);
impl ::std::convert::From<i32> for WER_REGISTER_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_REGISTER_FILE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_INFORMATION")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_INFORMATION_V3 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_INFORMATION_V3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_INFORMATION_V3")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_INFORMATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_INFORMATION_V3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_INFORMATION_V4 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_INFORMATION_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_INFORMATION_V4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_INFORMATION_V4")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_INFORMATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.hProcess == other.hProcess
            && self.wzConsentKey == other.wzConsentKey
            && self.wzFriendlyEventName == other.wzFriendlyEventName
            && self.wzApplicationName == other.wzApplicationName
            && self.wzApplicationPath == other.wzApplicationPath
            && self.wzDescription == other.wzDescription
            && self.hwndParent == other.hwndParent
            && self.wzNamespacePartner == other.wzNamespacePartner
            && self.wzNamespaceGroup == other.wzNamespaceGroup
            && self.rgbApplicationIdentity == other.rgbApplicationIdentity
            && self.hSnapshot == other.hSnapshot
            && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_INFORMATION_V4 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_INFORMATION_V4 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_INFORMATION_V5 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
    pub submitResultMax: WER_SUBMIT_RESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_INFORMATION_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_INFORMATION_V5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_INFORMATION_V5")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .field("submitResultMax", &self.submitResultMax)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_INFORMATION_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.hProcess == other.hProcess
            && self.wzConsentKey == other.wzConsentKey
            && self.wzFriendlyEventName == other.wzFriendlyEventName
            && self.wzApplicationName == other.wzApplicationName
            && self.wzApplicationPath == other.wzApplicationPath
            && self.wzDescription == other.wzDescription
            && self.hwndParent == other.hwndParent
            && self.wzNamespacePartner == other.wzNamespacePartner
            && self.wzNamespaceGroup == other.wzNamespaceGroup
            && self.rgbApplicationIdentity == other.rgbApplicationIdentity
            && self.hSnapshot == other.hSnapshot
            && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken
            && self.submitResultMax == other.submitResultMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_INFORMATION_V5 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_INFORMATION_V5 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::runtime::GUID,
    pub ReportId: ::windows::runtime::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_METADATA_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_METADATA_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_METADATA_V1").field("Signature", &self.Signature).field("BucketId", &self.BucketId).field("ReportId", &self.ReportId).field("CreationTime", &self.CreationTime).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_METADATA_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_METADATA_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::runtime::GUID,
    pub ReportId: ::windows::runtime::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows::runtime::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_METADATA_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_METADATA_V2")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes && self.CabId == other.CabId && self.ReportStatus == other.ReportStatus && self.ReportIntegratorId == other.ReportIntegratorId && self.NumberOfFiles == other.NumberOfFiles && self.SizeOfFileNames == other.SizeOfFileNames && self.FileNames == other.FileNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_METADATA_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::runtime::GUID,
    pub ReportId: ::windows::runtime::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows::runtime::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: super::super::Foundation::PWSTR,
    pub FriendlyEventName: [u16; 128],
    pub ApplicationName: [u16; 128],
    pub ApplicationPath: [u16; 260],
    pub Description: [u16; 512],
    pub BucketIdString: [u16; 260],
    pub LegacyBucketId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl WER_REPORT_METADATA_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WER_REPORT_METADATA_V3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_METADATA_V3")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .field("FriendlyEventName", &self.FriendlyEventName)
            .field("ApplicationName", &self.ApplicationName)
            .field("ApplicationPath", &self.ApplicationPath)
            .field("Description", &self.Description)
            .field("BucketIdString", &self.BucketIdString)
            .field("LegacyBucketId", &self.LegacyBucketId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WER_REPORT_METADATA_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature
            && self.BucketId == other.BucketId
            && self.ReportId == other.ReportId
            && self.CreationTime == other.CreationTime
            && self.SizeInBytes == other.SizeInBytes
            && self.CabId == other.CabId
            && self.ReportStatus == other.ReportStatus
            && self.ReportIntegratorId == other.ReportIntegratorId
            && self.NumberOfFiles == other.NumberOfFiles
            && self.SizeOfFileNames == other.SizeOfFileNames
            && self.FileNames == other.FileNames
            && self.FriendlyEventName == other.FriendlyEventName
            && self.ApplicationName == other.ApplicationName
            && self.ApplicationPath == other.ApplicationPath
            && self.Description == other.Description
            && self.BucketIdString == other.BucketIdString
            && self.LegacyBucketId == other.LegacyBucketId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WER_REPORT_METADATA_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WER_REPORT_METADATA_V3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub struct WER_REPORT_PARAMETER {
    pub Name: [u16; 129],
    pub Value: [u16; 260],
}
impl WER_REPORT_PARAMETER {}
impl ::std::default::Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WER_REPORT_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_PARAMETER").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for WER_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for WER_REPORT_PARAMETER {}
unsafe impl ::windows::runtime::Abi for WER_REPORT_PARAMETER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub struct WER_REPORT_SIGNATURE {
    pub EventName: [u16; 65],
    pub Parameters: [WER_REPORT_PARAMETER; 10],
}
impl WER_REPORT_SIGNATURE {}
impl ::std::default::Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WER_REPORT_SIGNATURE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WER_REPORT_SIGNATURE").field("EventName", &self.EventName).field("Parameters", &self.Parameters).finish()
    }
}
impl ::std::cmp::PartialEq for WER_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.EventName == other.EventName && self.Parameters == other.Parameters
    }
}
impl ::std::cmp::Eq for WER_REPORT_SIGNATURE {}
unsafe impl ::windows::runtime::Abi for WER_REPORT_SIGNATURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_REPORT_TYPE(pub i32);
pub const WerReportNonCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(0i32);
pub const WerReportCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(1i32);
pub const WerReportApplicationCrash: WER_REPORT_TYPE = WER_REPORT_TYPE(2i32);
pub const WerReportApplicationHang: WER_REPORT_TYPE = WER_REPORT_TYPE(3i32);
pub const WerReportKernel: WER_REPORT_TYPE = WER_REPORT_TYPE(4i32);
pub const WerReportInvalid: WER_REPORT_TYPE = WER_REPORT_TYPE(5i32);
impl ::std::convert::From<i32> for WER_REPORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_REPORT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_REPORT_UI(pub i32);
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = WER_REPORT_UI(1i32);
pub const WerUIIconFilePath: WER_REPORT_UI = WER_REPORT_UI(2i32);
pub const WerUIConsentDlgHeader: WER_REPORT_UI = WER_REPORT_UI(3i32);
pub const WerUIConsentDlgBody: WER_REPORT_UI = WER_REPORT_UI(4i32);
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(5i32);
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(6i32);
pub const WerUICloseText: WER_REPORT_UI = WER_REPORT_UI(7i32);
pub const WerUICloseDlgHeader: WER_REPORT_UI = WER_REPORT_UI(8i32);
pub const WerUICloseDlgBody: WER_REPORT_UI = WER_REPORT_UI(9i32);
pub const WerUICloseDlgButtonText: WER_REPORT_UI = WER_REPORT_UI(10i32);
pub const WerUIMax: WER_REPORT_UI = WER_REPORT_UI(11i32);
impl ::std::convert::From<i32> for WER_REPORT_UI {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_REPORT_UI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub exceptionRecord: super::Diagnostics::Debug::EXCEPTION_RECORD,
    pub context: super::Diagnostics::Debug::CONTEXT,
    pub pwszReportId: super::super::Foundation::PWSTR,
    pub bIsFatal: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl WER_RUNTIME_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for WER_RUNTIME_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for WER_RUNTIME_EXCEPTION_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_SUBMIT_FLAGS(pub u32);
pub const WER_SUBMIT_ADD_REGISTERED_DATA: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(16u32);
pub const WER_SUBMIT_HONOR_RECOVERY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1u32);
pub const WER_SUBMIT_HONOR_RESTART: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2u32);
pub const WER_SUBMIT_NO_ARCHIVE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(256u32);
pub const WER_SUBMIT_NO_CLOSE_UI: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(64u32);
pub const WER_SUBMIT_NO_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(128u32);
pub const WER_SUBMIT_OUTOFPROCESS: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(32u32);
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1024u32);
pub const WER_SUBMIT_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4u32);
pub const WER_SUBMIT_SHOW_DEBUG: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8u32);
pub const WER_SUBMIT_START_MINIMIZED: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(512u32);
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2048u32);
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4096u32);
pub const WER_SUBMIT_REPORT_MACHINE_ID: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8192u32);
impl ::std::convert::From<u32> for WER_SUBMIT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_SUBMIT_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WER_SUBMIT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WER_SUBMIT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WER_SUBMIT_RESULT(pub i32);
pub const WerReportQueued: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(1i32);
pub const WerReportUploaded: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(2i32);
pub const WerReportDebug: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(3i32);
pub const WerReportFailed: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(4i32);
pub const WerDisabled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(5i32);
pub const WerReportCancelled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(6i32);
pub const WerDisabledQueue: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(7i32);
pub const WerReportAsync: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(8i32);
pub const WerCustomAction: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(9i32);
pub const WerThrottled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(10i32);
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(11i32);
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(12i32);
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(13i32);
impl ::std::convert::From<i32> for WER_SUBMIT_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WER_SUBMIT_RESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerAddExcludedApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pwzexename: Param0, ballusers: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerAddExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        WerAddExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerFreeString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszstr: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerFreeString(pwszstr: super::super::Foundation::PWSTR);
        }
        ::std::mem::transmute(WerFreeString(pwszstr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerGetFlags<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> ::windows::runtime::Result<WER_FAULT_REPORTING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerGetFlags(hprocess: super::super::Foundation::HANDLE, pdwflags: *mut WER_FAULT_REPORTING) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_FAULT_REPORTING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerGetFlags(hprocess.into_param().abi(), &mut result__).from_abi::<WER_FAULT_REPORTING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows::runtime::HRESULT;
        }
        WerRegisterAdditionalProcess(::std::mem::transmute(processid), ::std::mem::transmute(captureextrainfoforthreadid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRegisterAppLocalDump<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(localappdatarelativepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterAppLocalDump(localappdatarelativepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerRegisterAppLocalDump(localappdatarelativepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRegisterCustomMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(key: Param0, value: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterCustomMetadata(key: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerRegisterCustomMetadata(key.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerRegisterExcludedMemoryBlock(address: *const ::std::ffi::c_void, size: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterExcludedMemoryBlock(address: *const ::std::ffi::c_void, size: u32) -> ::windows::runtime::HRESULT;
        }
        WerRegisterExcludedMemoryBlock(::std::mem::transmute(address), ::std::mem::transmute(size)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRegisterFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzfile: Param0, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterFile(pwzfile: super::super::Foundation::PWSTR, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows::runtime::HRESULT;
        }
        WerRegisterFile(pwzfile.into_param().abi(), ::std::mem::transmute(regfiletype), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerRegisterMemoryBlock(pvaddress: *const ::std::ffi::c_void, dwsize: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterMemoryBlock(pvaddress: *const ::std::ffi::c_void, dwsize: u32) -> ::windows::runtime::HRESULT;
        }
        WerRegisterMemoryBlock(::std::mem::transmute(pvaddress), ::std::mem::transmute(dwsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRegisterRuntimeExceptionModule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszoutofprocesscallbackdll: Param0, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRemoveExcludedApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pwzexename: Param0, ballusers: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRemoveExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        WerRemoveExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn WerReportAddDump<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hreporthandle: Param0, hprocess: Param1, hthread: Param2, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportAddDump(hreporthandle: HREPORT, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        WerReportAddDump(hreporthandle.into_param().abi(), hprocess.into_param().abi(), hthread.into_param().abi(), ::std::mem::transmute(dumptype), ::std::mem::transmute(pexceptionparam), ::std::mem::transmute(pdumpcustomoptions), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportAddFile<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreporthandle: Param0, pwzpath: Param1, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportAddFile(hreporthandle: HREPORT, pwzpath: super::super::Foundation::PWSTR, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows::runtime::HRESULT;
        }
        WerReportAddFile(hreporthandle.into_param().abi(), pwzpath.into_param().abi(), ::std::mem::transmute(repfiletype), ::std::mem::transmute(dwfileflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerReportCloseHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>>(hreporthandle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportCloseHandle(hreporthandle: HREPORT) -> ::windows::runtime::HRESULT;
        }
        WerReportCloseHandle(hreporthandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzeventtype: Param0, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION) -> ::windows::runtime::Result<HREPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportCreate(pwzeventtype: super::super::Foundation::PWSTR, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION, phreporthandle: *mut HREPORT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HREPORT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerReportCreate(pwzeventtype.into_param().abi(), ::std::mem::transmute(reptype), ::std::mem::transmute(preportinformation), &mut result__).from_abi::<HREPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportHang<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndhungapp: Param0, pwzhungapplicationname: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportHang(hwndhungapp: super::super::Foundation::HWND, pwzhungapplicationname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerReportHang(hwndhungapp.into_param().abi(), pwzhungapplicationname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportSetParameter<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreporthandle: Param0, dwparamid: u32, pwzname: Param2, pwzvalue: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSetParameter(hreporthandle: HREPORT, dwparamid: u32, pwzname: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerReportSetParameter(hreporthandle.into_param().abi(), ::std::mem::transmute(dwparamid), pwzname.into_param().abi(), pwzvalue.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportSetUIOption<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreporthandle: Param0, repuitypeid: WER_REPORT_UI, pwzvalue: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSetUIOption(hreporthandle: HREPORT, repuitypeid: WER_REPORT_UI, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerReportSetUIOption(hreporthandle.into_param().abi(), ::std::mem::transmute(repuitypeid), pwzvalue.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerReportSubmit<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORT>>(hreporthandle: Param0, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS) -> ::windows::runtime::Result<WER_SUBMIT_RESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSubmit(hreporthandle: HREPORT, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_SUBMIT_RESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerReportSubmit(hreporthandle.into_param().abi(), ::std::mem::transmute(consent), ::std::mem::transmute(dwflags), &mut result__).from_abi::<WER_SUBMIT_RESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows::runtime::HRESULT;
        }
        WerSetFlags(::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerStoreClose<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreClose(hreportstore: HREPORTSTORE);
        }
        ::std::mem::transmute(WerStoreClose(hreportstore.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreGetFirstReportKey<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetFirstReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreGetFirstReportKey(hreportstore.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreGetNextReportKey<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetNextReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreGetNextReportKey(hreportstore.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerStoreGetReportCount<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetReportCount(hreportstore: HREPORTSTORE, pdwreportcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreGetReportCount(hreportstore.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerStoreGetSizeOnDisk<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetSizeOnDisk(hreportstore: HREPORTSTORE, pqwsizeinbytes: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreGetSizeOnDisk(hreportstore.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES) -> ::windows::runtime::Result<HREPORTSTORE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES, phreportstore: *mut HREPORTSTORE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HREPORTSTORE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreOpen(::std::mem::transmute(repstoretype), &mut result__).from_abi::<HREPORTSTORE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerStorePurge() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStorePurge() -> ::windows::runtime::HRESULT;
        }
        WerStorePurge().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV1<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows::runtime::Result<WER_REPORT_METADATA_V1> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV1(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V1) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_REPORT_METADATA_V1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreQueryReportMetadataV1(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), &mut result__).from_abi::<WER_REPORT_METADATA_V1>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV2<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows::runtime::Result<WER_REPORT_METADATA_V2> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV2(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V2) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_REPORT_METADATA_V2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreQueryReportMetadataV2(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), &mut result__).from_abi::<WER_REPORT_METADATA_V2>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV3<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows::runtime::Result<WER_REPORT_METADATA_V3> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV3(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V3) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_REPORT_METADATA_V3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreQueryReportMetadataV3(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), &mut result__).from_abi::<WER_REPORT_METADATA_V3>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreUploadReport<'a, Param0: ::windows::runtime::IntoParam<'a, HREPORTSTORE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hreportstore: Param0, pszreportkey: Param1, dwflags: u32) -> ::windows::runtime::Result<WER_SUBMIT_RESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreUploadReport(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, dwflags: u32, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WER_SUBMIT_RESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WerStoreUploadReport(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), ::std::mem::transmute(dwflags), &mut result__).from_abi::<WER_SUBMIT_RESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterAdditionalProcess(::std::mem::transmute(processid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerUnregisterAppLocalDump() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterAppLocalDump() -> ::windows::runtime::HRESULT;
        }
        WerUnregisterAppLocalDump().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerUnregisterCustomMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(key: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterCustomMetadata(key: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterCustomMetadata(key.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerUnregisterExcludedMemoryBlock(address: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterExcludedMemoryBlock(address: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterExcludedMemoryBlock(::std::mem::transmute(address)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerUnregisterFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzfilepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterFile(pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterFile(pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
#[inline]
pub unsafe fn WerUnregisterMemoryBlock(pvaddress: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterMemoryBlock(pvaddress: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterMemoryBlock(::std::mem::transmute(pvaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerUnregisterRuntimeExceptionModule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszoutofprocesscallbackdll: Param0, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfn_ADDEREXCLUDEDAPPLICATIONA = unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> EFaultRepRetVal;
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type pfn_ADDEREXCLUDEDAPPLICATIONW = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR) -> EFaultRepRetVal;
#[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel", feature = "Win32_System_SystemServices"))]
pub type pfn_REPORTFAULT = unsafe extern "system" fn(param0: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, param1: u32) -> EFaultRepRetVal;
