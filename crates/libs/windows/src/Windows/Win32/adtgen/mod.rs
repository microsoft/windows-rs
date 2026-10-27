pub const APF_AuditFailure: i32 = 0;
pub const APF_AuditSuccess: i32 = 1;
pub const APF_ValidFlags: i32 = 1;
pub const APT_Guid: AUDIT_PARAM_TYPE = 9;
pub const APT_Int64: AUDIT_PARAM_TYPE = 11;
pub const APT_IpAddress: AUDIT_PARAM_TYPE = 12;
pub const APT_LogonId: AUDIT_PARAM_TYPE = 6;
pub const APT_LogonIdWithSid: AUDIT_PARAM_TYPE = 13;
pub const APT_Luid: AUDIT_PARAM_TYPE = 8;
pub const APT_None: AUDIT_PARAM_TYPE = 1;
pub const APT_ObjectTypeList: AUDIT_PARAM_TYPE = 7;
pub const APT_Pointer: AUDIT_PARAM_TYPE = 4;
pub const APT_Sid: AUDIT_PARAM_TYPE = 5;
pub const APT_String: AUDIT_PARAM_TYPE = 2;
pub const APT_Time: AUDIT_PARAM_TYPE = 10;
pub const APT_Ulong: AUDIT_PARAM_TYPE = 3;
pub const AP_AccessMask: i32 = 512;
pub const AP_ClientLogonId: i32 = 512;
pub const AP_Filespec: i32 = 256;
pub const AP_FormatHex: i32 = 256;
pub const AP_ParamTypeBits: i32 = 8;
pub const AP_ParamTypeMask: i32 = 255;
pub const AP_PrimaryLogonId: i32 = 256;
pub const AP_SidAsLogonId: i32 = 256;
pub type AUDIT_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [super::BYTE; 128],
}
#[cfg(feature = "shared")]
impl Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: windows_core::GUID,
    pub Flags: super::USHORT,
    pub Level: super::USHORT,
    pub AccessMask: super::ACCESS_MASK,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: super::USHORT,
    pub Flags: super::USHORT,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
    pub Anonymous: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUDIT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_0 {
    pub Data0: super::ULONG_PTR,
    pub String: windows_core::PWSTR,
    pub u: super::ULONG_PTR,
    pub psid: *mut super::SID,
    pub pguid: *mut windows_core::GUID,
    pub LogonId_LowPart: super::ULONG,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUDIT_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_1 {
    pub Data1: super::ULONG_PTR,
    pub LogonId_HighPart: super::LONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIT_PARAMS {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
    pub Count: super::USHORT,
    pub Parameters: *mut AUDIT_PARAM,
}
pub type AUDIT_PARAM_TYPE = i32;
pub const AUDIT_TYPE_LEGACY: i32 = 1;
pub const AUDIT_TYPE_WMI: i32 = 2;
pub const AUTHZP_WPD_EVENT: i32 = 16;
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: super::USHORT,
    pub AuditId: super::USHORT,
    pub ParameterCount: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: super::ULONG,
    pub dwFlags: super::DWORD,
    pub RefCount: super::LONG,
    pub hAudit: super::ULONG_PTR,
    pub LinkId: super::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
#[cfg(feature = "shared")]
impl Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: i32 = 2;
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: i32 = 2;
pub type PAUDIT_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PAUDIT_IP_ADDRESS = *mut AUDIT_IP_ADDRESS;
#[cfg(feature = "shared")]
pub type PAUDIT_OBJECT_TYPE = *mut AUDIT_OBJECT_TYPE;
#[cfg(feature = "shared")]
pub type PAUDIT_OBJECT_TYPES = *mut AUDIT_OBJECT_TYPES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUDIT_PARAM = *mut AUDIT_PARAM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUDIT_PARAMS = *mut AUDIT_PARAMS;
#[cfg(feature = "shared")]
pub type PAUTHZ_AUDIT_EVENT_TYPE_LEGACY = *mut AUTHZ_AUDIT_EVENT_TYPE_LEGACY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUTHZ_AUDIT_EVENT_TYPE_OLD = *mut AUTHZ_AUDIT_EVENT_TYPE_OLD;
#[cfg(feature = "shared")]
pub type PAUTHZ_AUDIT_EVENT_TYPE_UNION = *mut AUTHZ_AUDIT_EVENT_TYPE_UNION;
pub const _AUTHZ_SS_MAXSIZE: i32 = 128;
