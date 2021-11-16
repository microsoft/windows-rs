#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type ISceSvcAttachmentData = *mut ::core::ffi::c_void;
pub type ISceSvcAttachmentPersistInfo = *mut ::core::ffi::c_void;
pub type PFSCE_FREE_INFO = unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32;
pub type PFSCE_LOG_INFO = unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32;
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
pub const SCESTATUS_SUCCESS: i32 = 0i32;
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[repr(C)]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_INFO {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_LINE {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::core::ffi::c_void,
    pub pfQueryInfo: PFSCE_QUERY_INFO,
    pub pfSetInfo: PFSCE_SET_INFO,
    pub pfFreeInfo: PFSCE_FREE_INFO,
    pub pfLogInfo: PFSCE_LOG_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCESVC_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_INFO {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_LINE {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
pub type SCESVC_INFO_TYPE = i32;
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = 0i32;
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = 1i32;
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = 2i32;
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = 3i32;
pub type SCE_LOG_ERR_LEVEL = u32;
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = 0u32;
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = 1u32;
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = 2u32;
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = 3u32;
pub const cNodetypeSceAnalysisServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1736462535, data2: 8184, data3: 4561, data4: [175, 251, 0, 192, 79, 185, 132, 249] };
pub const cNodetypeSceEventLog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752903832, data2: 19443, data3: 4561, data4: [140, 48, 0, 192, 79, 185, 132, 249] };
pub const cNodetypeSceTemplateServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 614987543, data2: 7948, data3: 4561, data4: [175, 251, 0, 192, 79, 185, 132, 249] };
