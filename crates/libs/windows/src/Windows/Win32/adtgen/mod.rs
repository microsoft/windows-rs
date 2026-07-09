pub const APF_AuditFailure: u32 = 0;
pub const APF_AuditSuccess: u32 = 1;
pub const APF_ValidFlags: u32 = 1;
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
pub const AP_AccessMask: u32 = 512;
pub const AP_ClientLogonId: u32 = 512;
pub const AP_Filespec: u32 = 256;
pub const AP_FormatHex: u32 = 256;
pub const AP_ParamTypeBits: u32 = 8;
pub const AP_ParamTypeMask: u32 = 255;
pub const AP_PrimaryLogonId: u32 = 256;
pub const AP_SidAsLogonId: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AUDIT_HANDLE(pub *mut core::ffi::c_void);
impl AUDIT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for AUDIT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: windows_core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: super::winnt::ACCESS_MASK,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUDIT_OBJECT_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: u32,
    pub Flags: u32,
    pub Anonymous: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUDIT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_0 {
    pub Data0: usize,
    pub String: windows_core::PWSTR,
    pub u: usize,
    pub psid: *mut super::winnt::SID,
    pub pguid: *mut windows_core::GUID,
    pub LogonId_LowPart: u32,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUDIT_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union AUDIT_PARAM_1 {
    pub Data1: usize,
    pub LogonId_HighPart: i32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUDIT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUDIT_PARAM_TYPE = i32;
pub const AUDIT_TYPE_LEGACY: u32 = 1;
pub const AUDIT_TYPE_WMI: u32 = 2;
pub const AUTHZP_WPD_EVENT: u32 = 16;
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: u32,
    pub dwFlags: u32,
    pub RefCount: i32,
    pub hAudit: usize,
    pub LinkId: super::winnt::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
#[cfg(feature = "Win32_winnt")]
impl Default for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
impl Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2;
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_HANDLE(pub *mut *mut core::ffi::c_void);
impl PAUDIT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAUDIT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_IP_ADDRESS(pub *mut AUDIT_IP_ADDRESS);
impl PAUDIT_IP_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_OBJECT_TYPE(pub *mut AUDIT_OBJECT_TYPE);
#[cfg(feature = "Win32_winnt")]
impl PAUDIT_OBJECT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAUDIT_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_OBJECT_TYPES(pub *mut AUDIT_OBJECT_TYPES);
#[cfg(feature = "Win32_winnt")]
impl PAUDIT_OBJECT_TYPES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAUDIT_OBJECT_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_PARAM(pub *mut AUDIT_PARAM);
#[cfg(feature = "Win32_winnt")]
impl PAUDIT_PARAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAUDIT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_PARAMS(pub *mut AUDIT_PARAMS);
#[cfg(feature = "Win32_winnt")]
impl PAUDIT_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAUDIT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUTHZ_AUDIT_EVENT_TYPE_LEGACY(pub *mut AUTHZ_AUDIT_EVENT_TYPE_LEGACY);
impl PAUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUTHZ_AUDIT_EVENT_TYPE_OLD(pub *mut AUTHZ_AUDIT_EVENT_TYPE_OLD);
#[cfg(feature = "Win32_winnt")]
impl PAUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUTHZ_AUDIT_EVENT_TYPE_UNION(pub *mut AUTHZ_AUDIT_EVENT_TYPE_UNION);
impl PAUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
