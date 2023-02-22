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
impl ::core::default::Default for ExtendedIsolationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ExtendedIsolationState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ExtendedIsolationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedIsolationState").field(&self.0).finish()
    }
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
impl ::core::default::Default for FailureCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FailureCategory {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FailureCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FailureCategory").field(&self.0).finish()
    }
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
impl ::core::default::Default for FixupState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FixupState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FixupState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FixupState").field(&self.0).finish()
    }
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
impl ::core::default::Default for IsolationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IsolationState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IsolationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolationState").field(&self.0).finish()
    }
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
impl ::core::default::Default for NapNotifyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NapNotifyType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NapNotifyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NapNotifyType").field(&self.0).finish()
    }
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
impl ::core::default::Default for NapTracingLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NapTracingLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NapTracingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NapTracingLevel").field(&self.0).finish()
    }
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
impl ::core::default::Default for RemoteConfigurationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RemoteConfigurationType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RemoteConfigurationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteConfigurationType").field(&self.0).finish()
    }
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
impl ::core::fmt::Debug for CorrelationId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CorrelationId").field("connId", &self.connId).field("timeStamp", &self.timeStamp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CorrelationId {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CorrelationId {
    fn eq(&self, other: &Self) -> bool {
        self.connId == other.connId && self.timeStamp == other.timeStamp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CorrelationId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for CountedString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CountedString").field("length", &self.length).field("string", &self.string).finish()
    }
}
impl ::windows::core::TypeKind for CountedString {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CountedString {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.string == other.string
    }
}
impl ::core::cmp::Eq for CountedString {}
impl ::core::default::Default for CountedString {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for FailureCategoryMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FailureCategoryMapping").field("mappingCompliance", &self.mappingCompliance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FailureCategoryMapping {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FailureCategoryMapping {
    fn eq(&self, other: &Self) -> bool {
        self.mappingCompliance == other.mappingCompliance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for FixupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FixupInfo").field("state", &self.state).field("percentage", &self.percentage).field("resultCodes", &self.resultCodes).field("fixupMsgId", &self.fixupMsgId).finish()
    }
}
impl ::windows::core::TypeKind for FixupInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FixupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage && self.resultCodes == other.resultCodes && self.fixupMsgId == other.fixupMsgId
    }
}
impl ::core::cmp::Eq for FixupInfo {}
impl ::core::default::Default for FixupInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for Ipv4Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Ipv4Address").field("addr", &self.addr).finish()
    }
}
impl ::windows::core::TypeKind for Ipv4Address {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for Ipv4Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::core::cmp::Eq for Ipv4Address {}
impl ::core::default::Default for Ipv4Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for Ipv6Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Ipv6Address").field("addr", &self.addr).finish()
    }
}
impl ::windows::core::TypeKind for Ipv6Address {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for Ipv6Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::core::cmp::Eq for Ipv6Address {}
impl ::core::default::Default for Ipv6Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for IsolationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolationInfo").field("isolationState", &self.isolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IsolationInfo {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolationInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for IsolationInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolationInfoEx").field("isolationState", &self.isolationState).field("extendedIsolationState", &self.extendedIsolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IsolationInfoEx {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolationInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.extendedIsolationState == other.extendedIsolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolationInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for NapComponentRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NapComponentRegistrationInfo").field("id", &self.id).field("friendlyName", &self.friendlyName).field("description", &self.description).field("version", &self.version).field("vendorName", &self.vendorName).field("infoClsid", &self.infoClsid).field("configClsid", &self.configClsid).field("registrationDate", &self.registrationDate).field("componentType", &self.componentType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NapComponentRegistrationInfo {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NapComponentRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.friendlyName == other.friendlyName && self.description == other.description && self.version == other.version && self.vendorName == other.vendorName && self.infoClsid == other.infoClsid && self.configClsid == other.configClsid && self.registrationDate == other.registrationDate && self.componentType == other.componentType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NapComponentRegistrationInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for NetworkSoH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NetworkSoH").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for NetworkSoH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NetworkSoH {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for NetworkSoH {}
impl ::core::default::Default for NetworkSoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for PrivateData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrivateData").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for PrivateData {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PrivateData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for PrivateData {}
impl ::core::default::Default for PrivateData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for ResultCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResultCodes").field("count", &self.count).field("results", &self.results).finish()
    }
}
impl ::windows::core::TypeKind for ResultCodes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ResultCodes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.results == other.results
    }
}
impl ::core::cmp::Eq for ResultCodes {}
impl ::core::default::Default for ResultCodes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for SoH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SoH").field("count", &self.count).field("attributes", &self.attributes).finish()
    }
}
impl ::windows::core::TypeKind for SoH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SoH {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.attributes == other.attributes
    }
}
impl ::core::cmp::Eq for SoH {}
impl ::core::default::Default for SoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for SoHAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SoHAttribute").field("type", &self.r#type).field("size", &self.size).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for SoHAttribute {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SoHAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.value == other.value
    }
}
impl ::core::cmp::Eq for SoHAttribute {}
impl ::core::default::Default for SoHAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for SystemHealthAgentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SystemHealthAgentState").field("id", &self.id).field("shaResultCodes", &self.shaResultCodes).field("failureCategory", &self.failureCategory).field("fixupInfo", &self.fixupInfo).finish()
    }
}
impl ::windows::core::TypeKind for SystemHealthAgentState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SystemHealthAgentState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.shaResultCodes == other.shaResultCodes && self.failureCategory == other.failureCategory && self.fixupInfo == other.fixupInfo
    }
}
impl ::core::cmp::Eq for SystemHealthAgentState {}
impl ::core::default::Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
