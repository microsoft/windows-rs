#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
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
unsafe impl ::windows::core::Abi for CorrelationId {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CorrelationId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CorrelationId>()) == 0 }
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
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CountedString {
    pub length: u16,
    pub string: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CountedString {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CountedString {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CountedString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CountedString").field("length", &self.length).field("string", &self.string).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CountedString {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CountedString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CountedString>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CountedString {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CountedString {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type ExtendedIsolationState = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const extendedIsolationStateNoData: ExtendedIsolationState = 0i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const extendedIsolationStateTransition: ExtendedIsolationState = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const extendedIsolationStateInfected: ExtendedIsolationState = 2i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const extendedIsolationStateUnknown: ExtendedIsolationState = 3i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type FailureCategory = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryNone: FailureCategory = 0i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryOther: FailureCategory = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryClientComponent: FailureCategory = 2i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryClientCommunication: FailureCategory = 3i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryServerComponent: FailureCategory = 4i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const failureCategoryServerCommunication: FailureCategory = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
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
unsafe impl ::windows::core::Abi for FailureCategoryMapping {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FailureCategoryMapping {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FailureCategoryMapping>()) == 0 }
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
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for FixupInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FixupInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FixupInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for FixupInfo {}
impl ::core::default::Default for FixupInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type FixupState = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const fixupStateSuccess: FixupState = 0i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const fixupStateInProgress: FixupState = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const fixupStateCouldNotUpdate: FixupState = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for Ipv4Address {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Ipv4Address {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Ipv4Address>()) == 0 }
    }
}
impl ::core::cmp::Eq for Ipv4Address {}
impl ::core::default::Default for Ipv4Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for Ipv6Address {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Ipv6Address {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Ipv6Address>()) == 0 }
    }
}
impl ::core::cmp::Eq for Ipv6Address {}
impl ::core::default::Default for Ipv6Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
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
unsafe impl ::windows::core::Abi for IsolationInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolationInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolationInfo>()) == 0 }
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
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
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
unsafe impl ::windows::core::Abi for IsolationInfoEx {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolationInfoEx {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolationInfoEx>()) == 0 }
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
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type IsolationState = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const isolationStateNotRestricted: IsolationState = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const isolationStateInProbation: IsolationState = 2i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const isolationStateRestrictedAccess: IsolationState = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection', 'Win32_Foundation'*"]
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
unsafe impl ::windows::core::Abi for NapComponentRegistrationInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NapComponentRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NapComponentRegistrationInfo>()) == 0 }
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
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type NapNotifyType = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const napNotifyTypeUnknown: NapNotifyType = 0i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const napNotifyTypeServiceState: NapNotifyType = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const napNotifyTypeQuarState: NapNotifyType = 2i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type NapTracingLevel = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const tracingLevelUndefined: NapTracingLevel = 0i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const tracingLevelBasic: NapTracingLevel = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const tracingLevelAdvanced: NapTracingLevel = 2i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const tracingLevelDebug: NapTracingLevel = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for NetworkSoH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NetworkSoH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NetworkSoH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NetworkSoH {}
impl ::core::default::Default for NetworkSoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for PrivateData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PrivateData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PrivateData>()) == 0 }
    }
}
impl ::core::cmp::Eq for PrivateData {}
impl ::core::default::Default for PrivateData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub type RemoteConfigurationType = i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const remoteConfigTypeMachine: RemoteConfigurationType = 1i32;
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for ResultCodes {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ResultCodes {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ResultCodes>()) == 0 }
    }
}
impl ::core::cmp::Eq for ResultCodes {}
impl ::core::default::Default for ResultCodes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for SoH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SoH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SoH>()) == 0 }
    }
}
impl ::core::cmp::Eq for SoH {}
impl ::core::default::Default for SoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for SoHAttribute {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SoHAttribute {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SoHAttribute>()) == 0 }
    }
}
impl ::core::cmp::Eq for SoHAttribute {}
impl ::core::default::Default for SoHAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_NetworkAccessProtection'*"]
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
unsafe impl ::windows::core::Abi for SystemHealthAgentState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SystemHealthAgentState {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SystemHealthAgentState>()) == 0 }
    }
}
impl ::core::cmp::Eq for SystemHealthAgentState {}
impl ::core::default::Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
