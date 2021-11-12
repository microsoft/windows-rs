#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CorrelationId(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CountedString(i32);
#[repr(transparent)]
pub struct ExtendedIsolationState(pub i32);
pub const extendedIsolationStateNoData: ExtendedIsolationState = ExtendedIsolationState(0i32);
pub const extendedIsolationStateTransition: ExtendedIsolationState = ExtendedIsolationState(1i32);
pub const extendedIsolationStateInfected: ExtendedIsolationState = ExtendedIsolationState(2i32);
pub const extendedIsolationStateUnknown: ExtendedIsolationState = ExtendedIsolationState(3i32);
#[repr(transparent)]
pub struct FailureCategory(pub i32);
pub const failureCategoryNone: FailureCategory = FailureCategory(0i32);
pub const failureCategoryOther: FailureCategory = FailureCategory(1i32);
pub const failureCategoryClientComponent: FailureCategory = FailureCategory(2i32);
pub const failureCategoryClientCommunication: FailureCategory = FailureCategory(3i32);
pub const failureCategoryServerComponent: FailureCategory = FailureCategory(4i32);
pub const failureCategoryServerCommunication: FailureCategory = FailureCategory(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FailureCategoryMapping(i32);
#[repr(C)]
pub struct FixupInfo(i32);
#[repr(transparent)]
pub struct FixupState(pub i32);
pub const fixupStateSuccess: FixupState = FixupState(0i32);
pub const fixupStateInProgress: FixupState = FixupState(1i32);
pub const fixupStateCouldNotUpdate: FixupState = FixupState(2i32);
#[repr(C)]
pub struct Ipv4Address(i32);
#[repr(C)]
pub struct Ipv6Address(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IsolationInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IsolationInfoEx(i32);
#[repr(transparent)]
pub struct IsolationState(pub i32);
pub const isolationStateNotRestricted: IsolationState = IsolationState(1i32);
pub const isolationStateInProbation: IsolationState = IsolationState(2i32);
pub const isolationStateRestrictedAccess: IsolationState = IsolationState(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NapComponentRegistrationInfo(i32);
#[repr(transparent)]
pub struct NapNotifyType(pub i32);
pub const napNotifyTypeUnknown: NapNotifyType = NapNotifyType(0i32);
pub const napNotifyTypeServiceState: NapNotifyType = NapNotifyType(1i32);
pub const napNotifyTypeQuarState: NapNotifyType = NapNotifyType(2i32);
#[repr(transparent)]
pub struct NapTracingLevel(pub i32);
pub const tracingLevelUndefined: NapTracingLevel = NapTracingLevel(0i32);
pub const tracingLevelBasic: NapTracingLevel = NapTracingLevel(1i32);
pub const tracingLevelAdvanced: NapTracingLevel = NapTracingLevel(2i32);
pub const tracingLevelDebug: NapTracingLevel = NapTracingLevel(3i32);
#[repr(C)]
pub struct NetworkSoH(i32);
#[repr(C)]
pub struct PrivateData(i32);
#[repr(transparent)]
pub struct RemoteConfigurationType(pub i32);
pub const remoteConfigTypeMachine: RemoteConfigurationType = RemoteConfigurationType(1i32);
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = RemoteConfigurationType(2i32);
#[repr(C)]
pub struct ResultCodes(i32);
#[repr(C)]
pub struct SoH(i32);
#[repr(C)]
pub struct SoHAttribute(i32);
#[repr(C)]
pub struct SystemHealthAgentState(i32);
