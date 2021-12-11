#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
    pub fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
    pub fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
    pub fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateDNSIncident(hostname: super::super::Foundation::PWSTR, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation', 'Win32_Networking_WinSock'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NdfCreateGroupingIncident(cloudname: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR, invitation: super::super::Foundation::PWSTR, addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateIncident(helperclassname: super::super::Foundation::PWSTR, celt: u32, attributes: *const HELPER_ATTRIBUTE, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
    pub fn NdfCreateNetConnectionIncident(handle: *mut *mut ::core::ffi::c_void, id: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreatePnrpIncident(cloudname: super::super::Foundation::PWSTR, peername: super::super::Foundation::PWSTR, diagnosepublish: super::super::Foundation::BOOL, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateSharingIncident(uncpath: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncident(url: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncidentEx(url: super::super::Foundation::PWSTR, usewinhttp: super::super::Foundation::BOOL, modulename: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation', 'Win32_Networking_WinSock', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
    pub fn NdfCreateWinSockIncident(sock: super::super::Networking::WinSock::SOCKET, host: super::super::Foundation::PWSTR, port: u16, appid: super::super::Foundation::PWSTR, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfExecuteDiagnosis(handle: *const ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfGetTraceFile(handle: *const ::core::ffi::c_void, tracefilelocation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type ATTRIBUTE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_INVALID: ATTRIBUTE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_INT8: ATTRIBUTE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_UINT8: ATTRIBUTE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_INT16: ATTRIBUTE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_UINT16: ATTRIBUTE_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_INT32: ATTRIBUTE_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_UINT32: ATTRIBUTE_TYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_INT64: ATTRIBUTE_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_UINT64: ATTRIBUTE_TYPE = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_STRING: ATTRIBUTE_TYPE = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_GUID: ATTRIBUTE_TYPE = 11i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = 12i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = 13i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = 14i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DF_IMPERSONATION: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DF_TRACELESS: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type DIAGNOSIS_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_REJECTED: DIAGNOSIS_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_DEFERRED: DIAGNOSIS_STATUS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [super::super::Foundation::CHAR; 126],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAG_SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAG_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl ::core::marker::Copy for DiagnosticsInfo {}
impl ::core::clone::Clone for DiagnosticsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: super::super::Foundation::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: super::super::Foundation::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: super::super::Foundation::PWSTR,
    pub Guid: ::windows_sys::core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYPOTHESIS {
    pub pwszClassName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYPOTHESIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HelperAttributeInfo {
    pub pwszName: super::super::Foundation::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HelperAttributeInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HelperAttributeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HypothesisResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type INetDiagExtensibleHelper = *mut ::core::ffi::c_void;
pub type INetDiagHelper = *mut ::core::ffi::c_void;
pub type INetDiagHelperEx = *mut ::core::ffi::c_void;
pub type INetDiagHelperInfo = *mut ::core::ffi::c_void;
pub type INetDiagHelperUtilFactory = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LIFE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_ERROR_START: u32 = 63744u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_BAD_PARAM: ::windows_sys::core::HRESULT = -2146895611i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_CANCELLED: ::windows_sys::core::HRESULT = -2146895614i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_DISABLED: ::windows_sys::core::HRESULT = -2146895612i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_LENGTH_EXCEEDED: ::windows_sys::core::HRESULT = -2146895616i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_NOHELPERCLASS: ::windows_sys::core::HRESULT = -2146895615i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_PROBLEM_PRESENT: ::windows_sys::core::HRESULT = -2146895608i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_UNKNOWN: ::windows_sys::core::HRESULT = -2146895609i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_E_VALIDATION: ::windows_sys::core::HRESULT = -2146895610i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for OCTET_STRING {}
impl ::core::clone::Clone for OCTET_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type PROBLEM_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_INVALID: PROBLEM_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_LOW_HEALTH: PROBLEM_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RCF_ISCONFIRMED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RCF_ISLEAF: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type REPAIR_RISK = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RR_NOROLLBACK: REPAIR_RISK = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RR_ROLLBACK: REPAIR_RISK = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RR_NORISK: REPAIR_RISK = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type REPAIR_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_SYSTEM: REPAIR_SCOPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_USER: REPAIR_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_APPLICATION: REPAIR_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_PROCESS: REPAIR_SCOPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type REPAIR_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_REPAIRED: REPAIR_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_UNREPAIRED: REPAIR_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_DEFERRED: REPAIR_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RS_USER_ACTION: REPAIR_STATUS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_REPRO: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_RESERVED_CA: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_RESERVED_LNI: u32 = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_UI_ONLY: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_USER_ACTION: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const RF_WORKAROUND: u32 = 536870912u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RepairInfo {
    pub guid: ::windows_sys::core::GUID,
    pub pwszClassName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RepairInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RepairInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RepairInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RepairInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RootCauseInfo {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub rootCauseID: ::windows_sys::core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: ::windows_sys::core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RootCauseInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RootCauseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ShellCommandInfo {
    pub pwszOperation: super::super::Foundation::PWSTR,
    pub pwszFile: super::super::Foundation::PWSTR,
    pub pwszParameters: super::super::Foundation::PWSTR,
    pub pwszDirectory: super::super::Foundation::PWSTR,
    pub nShowCmd: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ShellCommandInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ShellCommandInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub type UI_INFO_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const UIT_INVALID: UI_INFO_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const UIT_NONE: UI_INFO_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const UIT_HELP_PANE: UI_INFO_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework'*"]
pub const UIT_DUI: UI_INFO_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UiInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetworkDiagnosticsFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union UiInfo_0 {
    pub pwzNull: super::super::Foundation::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: super::super::Foundation::PWSTR,
    pub pwzDui: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UiInfo_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UiInfo_0 {
    fn clone(&self) -> Self {
        *self
    }
}
