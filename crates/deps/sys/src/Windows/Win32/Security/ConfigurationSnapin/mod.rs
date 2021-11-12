#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISceSvcAttachmentData(i32);
pub struct ISceSvcAttachmentPersistInfo(i32);
pub struct PFSCE_FREE_INFO(i32);
pub struct PFSCE_LOG_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFSCE_QUERY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFSCE_SET_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PF_ConfigAnalyzeService(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PF_UpdateService(i32);
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
pub struct SCESVC_ANALYSIS_INFO(i32);
pub struct SCESVC_ANALYSIS_LINE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO(i32);
pub struct SCESVC_CONFIGURATION_INFO(i32);
pub struct SCESVC_CONFIGURATION_LINE(i32);
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
pub struct SCESVC_INFO_TYPE(i32);
pub struct SCE_LOG_ERR_LEVEL(i32);
pub const cNodetypeSceAnalysisServices: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1736462535, data2: 8184, data3: 4561, data4: [175, 251, 0, 192, 79, 185, 132, 249] };
pub const cNodetypeSceEventLog: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 752903832, data2: 19443, data3: 4561, data4: [140, 48, 0, 192, 79, 185, 132, 249] };
pub const cNodetypeSceTemplateServices: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 614987543, data2: 7948, data3: 4561, data4: [175, 251, 0, 192, 79, 185, 132, 249] };
