pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
pub const extendedIsolationStateInfected: ExtendedIsolationState = 2i32;
pub const extendedIsolationStateNoData: ExtendedIsolationState = 0i32;
pub const extendedIsolationStateTransition: ExtendedIsolationState = 1i32;
pub const extendedIsolationStateUnknown: ExtendedIsolationState = 3i32;
pub const failureCategoryClientCommunication: FailureCategory = 3i32;
pub const failureCategoryClientComponent: FailureCategory = 2i32;
pub const failureCategoryCount: u32 = 5u32;
pub const failureCategoryNone: FailureCategory = 0i32;
pub const failureCategoryOther: FailureCategory = 1i32;
pub const failureCategoryServerCommunication: FailureCategory = 5i32;
pub const failureCategoryServerComponent: FailureCategory = 4i32;
pub const fixupStateCouldNotUpdate: FixupState = 2i32;
pub const fixupStateInProgress: FixupState = 1i32;
pub const fixupStateSuccess: FixupState = 0i32;
pub const freshSoHRequest: u32 = 1u32;
pub const isolationStateInProbation: IsolationState = 2i32;
pub const isolationStateNotRestricted: IsolationState = 1i32;
pub const isolationStateRestrictedAccess: IsolationState = 3i32;
pub const maxConnectionCountPerEnforcer: u32 = 20u32;
pub const maxEnforcerCount: u32 = 20u32;
pub const maxNetworkSoHSize: u32 = 4000u32;
pub const maxPrivateDataSize: u32 = 200u32;
pub const maxSoHAttributeCount: u32 = 100u32;
pub const maxSoHAttributeSize: u32 = 4000u32;
pub const maxStringLength: u32 = 1024u32;
pub const maxSystemHealthEntityCount: u32 = 20u32;
pub const minNetworkSoHSize: u32 = 12u32;
pub const napNotifyTypeQuarState: NapNotifyType = 2i32;
pub const napNotifyTypeServiceState: NapNotifyType = 1i32;
pub const napNotifyTypeUnknown: NapNotifyType = 0i32;
pub const percentageNotSupported: u32 = 101u32;
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = 2i32;
pub const remoteConfigTypeMachine: RemoteConfigurationType = 1i32;
pub const shaFixup: u32 = 1u32;
pub const tracingLevelAdvanced: NapTracingLevel = 2i32;
pub const tracingLevelBasic: NapTracingLevel = 1i32;
pub const tracingLevelDebug: NapTracingLevel = 3i32;
pub const tracingLevelUndefined: NapTracingLevel = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExtendedIsolationState(pub i32);
impl windows_core::TypeKind for ExtendedIsolationState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FailureCategory(pub i32);
impl windows_core::TypeKind for FailureCategory {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FixupState(pub i32);
impl windows_core::TypeKind for FixupState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IsolationState(pub i32);
impl windows_core::TypeKind for IsolationState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NapNotifyType(pub i32);
impl windows_core::TypeKind for NapNotifyType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NapTracingLevel(pub i32);
impl windows_core::TypeKind for NapTracingLevel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoteConfigurationType(pub i32);
impl windows_core::TypeKind for RemoteConfigurationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrelationId {
    pub connId: windows_core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
impl Default for CorrelationId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CorrelationId {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CountedString {
    pub length: u16,
    pub string: windows_core::PWSTR,
}
impl Default for CountedString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CountedString {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
impl Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FailureCategoryMapping {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl Default for FixupInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FixupInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl Default for Ipv4Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Ipv4Address {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl Default for Ipv6Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Ipv6Address {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
impl Default for IsolationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IsolationInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
impl Default for IsolationInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IsolationInfoEx {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: windows_core::GUID,
    pub configClsid: windows_core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
impl Default for NapComponentRegistrationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NapComponentRegistrationInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for NetworkSoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NetworkSoH {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for PrivateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PrivateData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut windows_core::HRESULT,
}
impl Default for ResultCodes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ResultCodes {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl Default for SoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SoH {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl Default for SoHAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SoHAttribute {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SystemHealthAgentState {
    type TypeKind = windows_core::CloneType;
}
