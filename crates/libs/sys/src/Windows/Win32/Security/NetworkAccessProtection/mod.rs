pub const ComponentTypeEnforcementClientRp: u32 = 2;
pub const ComponentTypeEnforcementClientSoH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CorrelationId {
    pub connId: windows_sys::core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CountedString {
    pub length: u16,
    pub string: windows_sys::core::PWSTR,
}
impl Default for CountedString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ExtendedIsolationState = i32;
pub type FailureCategory = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [windows_sys::core::BOOL; 5],
}
impl Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
pub type FixupState = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl Default for Ipv4Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl Default for Ipv6Address {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
pub type IsolationState = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: windows_sys::core::GUID,
    pub configClsid: windows_sys::core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
pub type NapNotifyType = i32;
pub type NapTracingLevel = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for NetworkSoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl Default for PrivateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RemoteConfigurationType = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut windows_sys::core::HRESULT,
}
impl Default for ResultCodes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl Default for SoH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
pub const extendedIsolationStateInfected: ExtendedIsolationState = 2;
pub const extendedIsolationStateNoData: ExtendedIsolationState = 0;
pub const extendedIsolationStateTransition: ExtendedIsolationState = 1;
pub const extendedIsolationStateUnknown: ExtendedIsolationState = 3;
pub const failureCategoryClientCommunication: FailureCategory = 3;
pub const failureCategoryClientComponent: FailureCategory = 2;
pub const failureCategoryCount: u32 = 5;
pub const failureCategoryNone: FailureCategory = 0;
pub const failureCategoryOther: FailureCategory = 1;
pub const failureCategoryServerCommunication: FailureCategory = 5;
pub const failureCategoryServerComponent: FailureCategory = 4;
pub const fixupStateCouldNotUpdate: FixupState = 2;
pub const fixupStateInProgress: FixupState = 1;
pub const fixupStateSuccess: FixupState = 0;
pub const freshSoHRequest: u32 = 1;
pub const isolationStateInProbation: IsolationState = 2;
pub const isolationStateNotRestricted: IsolationState = 1;
pub const isolationStateRestrictedAccess: IsolationState = 3;
pub const maxConnectionCountPerEnforcer: u32 = 20;
pub const maxEnforcerCount: u32 = 20;
pub const maxNetworkSoHSize: u32 = 4000;
pub const maxPrivateDataSize: u32 = 200;
pub const maxSoHAttributeCount: u32 = 100;
pub const maxSoHAttributeSize: u32 = 4000;
pub const maxStringLength: u32 = 1024;
pub const maxSystemHealthEntityCount: u32 = 20;
pub const minNetworkSoHSize: u32 = 12;
pub const napNotifyTypeQuarState: NapNotifyType = 2;
pub const napNotifyTypeServiceState: NapNotifyType = 1;
pub const napNotifyTypeUnknown: NapNotifyType = 0;
pub const percentageNotSupported: u32 = 101;
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = 2;
pub const remoteConfigTypeMachine: RemoteConfigurationType = 1;
pub const shaFixup: u32 = 1;
pub const tracingLevelAdvanced: NapTracingLevel = 2;
pub const tracingLevelBasic: NapTracingLevel = 1;
pub const tracingLevelDebug: NapTracingLevel = 3;
pub const tracingLevelUndefined: NapTracingLevel = 0;
