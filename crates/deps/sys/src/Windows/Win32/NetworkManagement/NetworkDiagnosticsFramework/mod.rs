#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateDNSIncident(hostname: super::super::Foundation::PWSTR, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NdfCreateGroupingIncident(cloudname: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR, invitation: super::super::Foundation::PWSTR, addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateIncident(helperclassname: super::super::Foundation::PWSTR, celt: u32, attributes: *const HELPER_ATTRIBUTE, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCreateNetConnectionIncident(handle: *mut *mut ::core::ffi::c_void, id: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreatePnrpIncident(cloudname: super::super::Foundation::PWSTR, peername: super::super::Foundation::PWSTR, diagnosepublish: super::super::Foundation::BOOL, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateSharingIncident(uncpath: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncident(url: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncidentEx(url: super::super::Foundation::PWSTR, usewinhttp: super::super::Foundation::BOOL, modulename: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
    pub fn NdfCreateWinSockIncident(sock: super::super::Networking::WinSock::SOCKET, host: super::super::Foundation::PWSTR, port: u16, appid: super::super::Foundation::PWSTR, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfExecuteDiagnosis(handle: *const ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfGetTraceFile(handle: *const ::core::ffi::c_void, tracefilelocation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows_sys::core::HRESULT;
}
pub struct ATTRIBUTE_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const DF_IMPERSONATION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const DF_TRACELESS: u32 = 1073741824u32;
pub struct DIAGNOSIS_STATUS(i32);
pub struct DIAG_SOCKADDR(i32);
pub struct DiagnosticsInfo(i32);
pub struct HELPER_ATTRIBUTE(i32);
pub struct HYPOTHESIS(i32);
pub struct HelperAttributeInfo(i32);
pub struct HypothesisResult(i32);
pub struct INetDiagExtensibleHelper(i32);
pub struct INetDiagHelper(i32);
pub struct INetDiagHelperEx(i32);
pub struct INetDiagHelperInfo(i32);
pub struct INetDiagHelperUtilFactory(i32);
pub struct LIFE_TIME(i32);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_ERROR_START: u32 = 63744u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_BAD_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895611i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895614i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895612i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_LENGTH_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895616i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_NOHELPERCLASS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895615i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_PROBLEM_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895608i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895609i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_VALIDATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146895610i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
pub struct OCTET_STRING(i32);
pub struct PROBLEM_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISCONFIRMED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISLEAF: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
pub struct REPAIR_RISK(i32);
pub struct REPAIR_SCOPE(i32);
pub struct REPAIR_STATUS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_REPRO: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED_CA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED_LNI: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_UI_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_USER_ACTION: u32 = 268435456u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_WORKAROUND: u32 = 536870912u32;
pub struct RepairInfo(i32);
pub struct RepairInfoEx(i32);
pub struct RootCauseInfo(i32);
pub struct ShellCommandInfo(i32);
pub struct UI_INFO_TYPE(i32);
pub struct UiInfo(i32);
