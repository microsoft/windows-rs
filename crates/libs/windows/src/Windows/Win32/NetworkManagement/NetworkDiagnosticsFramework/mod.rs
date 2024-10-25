pub const AT_BOOLEAN: ATTRIBUTE_TYPE = 1i32;
pub const AT_GUID: ATTRIBUTE_TYPE = 11i32;
pub const AT_INT16: ATTRIBUTE_TYPE = 4i32;
pub const AT_INT32: ATTRIBUTE_TYPE = 6i32;
pub const AT_INT64: ATTRIBUTE_TYPE = 8i32;
pub const AT_INT8: ATTRIBUTE_TYPE = 2i32;
pub const AT_INVALID: ATTRIBUTE_TYPE = 0i32;
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = 12i32;
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = 14i32;
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = 13i32;
pub const AT_STRING: ATTRIBUTE_TYPE = 10i32;
pub const AT_UINT16: ATTRIBUTE_TYPE = 5i32;
pub const AT_UINT32: ATTRIBUTE_TYPE = 7i32;
pub const AT_UINT64: ATTRIBUTE_TYPE = 9i32;
pub const AT_UINT8: ATTRIBUTE_TYPE = 3i32;
pub const DF_IMPERSONATION: u32 = 2147483648u32;
pub const DF_TRACELESS: u32 = 1073741824u32;
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = 1i32;
pub const DS_DEFERRED: DIAGNOSIS_STATUS = 4i32;
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = 3i32;
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = 0i32;
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = 5i32;
pub const DS_REJECTED: DIAGNOSIS_STATUS = 2i32;
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
pub const NDF_ERROR_START: u32 = 63744u32;
pub const NDF_E_BAD_PARAM: windows_core::HRESULT = 0x8008F905_u32 as _;
pub const NDF_E_CANCELLED: windows_core::HRESULT = 0x8008F902_u32 as _;
pub const NDF_E_DISABLED: windows_core::HRESULT = 0x8008F904_u32 as _;
pub const NDF_E_LENGTH_EXCEEDED: windows_core::HRESULT = 0x8008F900_u32 as _;
pub const NDF_E_NOHELPERCLASS: windows_core::HRESULT = 0x8008F901_u32 as _;
pub const NDF_E_PROBLEM_PRESENT: windows_core::HRESULT = 0x8008F908_u32 as _;
pub const NDF_E_UNKNOWN: windows_core::HRESULT = 0x8008F907_u32 as _;
pub const NDF_E_VALIDATION: windows_core::HRESULT = 0x8008F906_u32 as _;
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = 4i32;
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = 16i32;
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = 8i32;
pub const PT_INVALID: PROBLEM_TYPE = 0i32;
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = 2i32;
pub const PT_LOW_HEALTH: PROBLEM_TYPE = 1i32;
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = 32i32;
pub const RCF_ISCONFIRMED: u32 = 2u32;
pub const RCF_ISLEAF: u32 = 1u32;
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
pub const RF_REPRO: u32 = 2097152u32;
pub const RF_RESERVED: u32 = 1073741824u32;
pub const RF_RESERVED_CA: u32 = 2147483648u32;
pub const RF_RESERVED_LNI: u32 = 65536u32;
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
pub const RF_UI_ONLY: u32 = 16777216u32;
pub const RF_USER_ACTION: u32 = 268435456u32;
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
pub const RF_WORKAROUND: u32 = 536870912u32;
pub const RR_NORISK: REPAIR_RISK = 2i32;
pub const RR_NOROLLBACK: REPAIR_RISK = 0i32;
pub const RR_ROLLBACK: REPAIR_RISK = 1i32;
pub const RS_APPLICATION: REPAIR_SCOPE = 2i32;
pub const RS_DEFERRED: REPAIR_STATUS = 3i32;
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = 0i32;
pub const RS_PROCESS: REPAIR_SCOPE = 3i32;
pub const RS_REPAIRED: REPAIR_STATUS = 1i32;
pub const RS_SYSTEM: REPAIR_SCOPE = 0i32;
pub const RS_UNREPAIRED: REPAIR_STATUS = 2i32;
pub const RS_USER: REPAIR_SCOPE = 1i32;
pub const RS_USER_ACTION: REPAIR_STATUS = 4i32;
pub const UIT_DUI: UI_INFO_TYPE = 4i32;
pub const UIT_HELP_PANE: UI_INFO_TYPE = 3i32;
pub const UIT_INVALID: UI_INFO_TYPE = 0i32;
pub const UIT_NONE: UI_INFO_TYPE = 1i32;
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = 2i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ATTRIBUTE_TYPE(pub i32);
impl windows_core::TypeKind for ATTRIBUTE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DIAGNOSIS_STATUS(pub i32);
impl windows_core::TypeKind for DIAGNOSIS_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROBLEM_TYPE(pub i32);
impl windows_core::TypeKind for PROBLEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REPAIR_RISK(pub i32);
impl windows_core::TypeKind for REPAIR_RISK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REPAIR_SCOPE(pub i32);
impl windows_core::TypeKind for REPAIR_SCOPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REPAIR_STATUS(pub i32);
impl windows_core::TypeKind for REPAIR_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UI_INFO_TYPE(pub i32);
impl windows_core::TypeKind for UI_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [i8; 126],
}
impl Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DIAG_SOCKADDR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl Default for DiagnosticsInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DiagnosticsInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
impl Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HELPER_ATTRIBUTE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub PWStr: windows_core::PWSTR,
    pub Guid: windows_core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
impl Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HELPER_ATTRIBUTE_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HYPOTHESIS {
    pub pwszClassName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
impl Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HYPOTHESIS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelperAttributeInfo {
    pub pwszName: windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
impl Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HelperAttributeInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
impl Default for HypothesisResult {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HypothesisResult {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
impl Default for LIFE_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LIFE_TIME {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OCTET_STRING {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RepairInfo {
    pub guid: windows_core::GUID,
    pub pwszClassName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
impl Default for RepairInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RepairInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
impl Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RepairInfoEx {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootCauseInfo {
    pub pwszDescription: windows_core::PWSTR,
    pub rootCauseID: windows_core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: windows_core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RootCauseInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellCommandInfo {
    pub pwszOperation: windows_core::PWSTR,
    pub pwszFile: windows_core::PWSTR,
    pub pwszParameters: windows_core::PWSTR,
    pub pwszDirectory: windows_core::PWSTR,
    pub nShowCmd: u32,
}
impl Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ShellCommandInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
impl Default for UiInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UiInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union UiInfo_0 {
    pub pwzNull: windows_core::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: windows_core::PWSTR,
    pub pwzDui: windows_core::PWSTR,
}
impl Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UiInfo_0 {
    type TypeKind = windows_core::CloneType;
}
