#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferCloseLevel(hlevelhandle: super::SAFER_LEVEL_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferComputeTokenFromLevel(levelhandle: super::SAFER_LEVEL_HANDLE, inaccesstoken: super::super::Foundation::HANDLE, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferGetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *mut ::core::ffi::c_void, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut ::core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferIdentifyLevel(dwnumproperties: u32, pcodeproperties: *const SAFER_CODE_PROPERTIES_V2, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferRecordEventLogEntry(hlevel: super::SAFER_LEVEL_HANDLE, sztargetpath: super::super::Foundation::PWSTR, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferSetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const ::core::ffi::c_void, dwinbuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferiIsExecutableFileType(szfullpathname: super::super::Foundation::PWSTR, bfromshellexecute: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOL;
}
