#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryCount: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const freshSoHRequest: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxConnectionCountPerEnforcer: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxEnforcerCount: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxNetworkSoHSize: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxPrivateDataSize: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSoHAttributeCount: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSoHAttributeSize: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxStringLength: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const maxSystemHealthEntityCount: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const minNetworkSoHSize: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const percentageNotSupported: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const shaFixup: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExtendedIsolationState(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateNoData: ExtendedIsolationState = ExtendedIsolationState(0i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateTransition: ExtendedIsolationState = ExtendedIsolationState(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateInfected: ExtendedIsolationState = ExtendedIsolationState(2i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const extendedIsolationStateUnknown: ExtendedIsolationState = ExtendedIsolationState(3i32);
impl ::core::marker::Copy for ExtendedIsolationState {}
impl ::core::clone::Clone for ExtendedIsolationState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ExtendedIsolationState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FailureCategory(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryNone: FailureCategory = FailureCategory(0i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryOther: FailureCategory = FailureCategory(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryClientComponent: FailureCategory = FailureCategory(2i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryClientCommunication: FailureCategory = FailureCategory(3i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryServerComponent: FailureCategory = FailureCategory(4i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const failureCategoryServerCommunication: FailureCategory = FailureCategory(5i32);
impl ::core::marker::Copy for FailureCategory {}
impl ::core::clone::Clone for FailureCategory {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FailureCategory {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FixupState(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateSuccess: FixupState = FixupState(0i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateInProgress: FixupState = FixupState(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const fixupStateCouldNotUpdate: FixupState = FixupState(2i32);
impl ::core::marker::Copy for FixupState {}
impl ::core::clone::Clone for FixupState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FixupState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsolationState(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateNotRestricted: IsolationState = IsolationState(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateInProbation: IsolationState = IsolationState(2i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const isolationStateRestrictedAccess: IsolationState = IsolationState(3i32);
impl ::core::marker::Copy for IsolationState {}
impl ::core::clone::Clone for IsolationState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IsolationState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NapNotifyType(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeUnknown: NapNotifyType = NapNotifyType(0i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeServiceState: NapNotifyType = NapNotifyType(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const napNotifyTypeQuarState: NapNotifyType = NapNotifyType(2i32);
impl ::core::marker::Copy for NapNotifyType {}
impl ::core::clone::Clone for NapNotifyType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NapNotifyType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NapTracingLevel(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelUndefined: NapTracingLevel = NapTracingLevel(0i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelBasic: NapTracingLevel = NapTracingLevel(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelAdvanced: NapTracingLevel = NapTracingLevel(2i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const tracingLevelDebug: NapTracingLevel = NapTracingLevel(3i32);
impl ::core::marker::Copy for NapTracingLevel {}
impl ::core::clone::Clone for NapTracingLevel {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NapTracingLevel {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteConfigurationType(pub i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const remoteConfigTypeMachine: RemoteConfigurationType = RemoteConfigurationType(1i32);
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = RemoteConfigurationType(2i32);
impl ::core::marker::Copy for RemoteConfigurationType {}
impl ::core::clone::Clone for RemoteConfigurationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RemoteConfigurationType {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CorrelationId {
    pub connId: ::windows::core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CorrelationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CorrelationId {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct CountedString {
    pub length: u16,
    pub string: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CountedString {}
impl ::core::clone::Clone for CountedString {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CountedString {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FailureCategoryMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FailureCategoryMapping {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl ::core::marker::Copy for FixupInfo {}
impl ::core::clone::Clone for FixupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FixupInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl ::core::marker::Copy for Ipv4Address {}
impl ::core::clone::Clone for Ipv4Address {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Ipv4Address {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl ::core::marker::Copy for Ipv6Address {}
impl ::core::clone::Clone for Ipv6Address {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Ipv6Address {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IsolationInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IsolationInfoEx {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: ::windows::core::GUID,
    pub configClsid: ::windows::core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NapComponentRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NapComponentRegistrationInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for NetworkSoH {}
impl ::core::clone::Clone for NetworkSoH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NetworkSoH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for PrivateData {}
impl ::core::clone::Clone for PrivateData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrivateData {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut ::windows::core::HRESULT,
}
impl ::core::marker::Copy for ResultCodes {}
impl ::core::clone::Clone for ResultCodes {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ResultCodes {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl ::core::marker::Copy for SoH {}
impl ::core::clone::Clone for SoH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SoH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl ::core::marker::Copy for SoHAttribute {}
impl ::core::clone::Clone for SoHAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SoHAttribute {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_NetworkAccessProtection\"`*"]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl ::core::marker::Copy for SystemHealthAgentState {}
impl ::core::clone::Clone for SystemHealthAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SystemHealthAgentState {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
