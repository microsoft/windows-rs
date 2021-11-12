#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_SCOPEID_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_SCOPEID_USER: u32 = 2u32;
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
