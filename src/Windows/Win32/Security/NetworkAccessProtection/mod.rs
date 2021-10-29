#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CorrelationId {
    pub connId: ::windows::runtime::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CorrelationId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CorrelationId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CorrelationId").field("connId", &self.connId).field("timeStamp", &self.timeStamp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CorrelationId {
    fn eq(&self, other: &Self) -> bool {
        self.connId == other.connId && self.timeStamp == other.timeStamp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CorrelationId {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CountedString {
    pub length: u16,
    pub string: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CountedString {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CountedString {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CountedString {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CountedString").field("length", &self.length).field("string", &self.string).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CountedString {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.string == other.string
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CountedString {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CountedString {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedIsolationState(pub i32);
pub const extendedIsolationStateNoData: ExtendedIsolationState = ExtendedIsolationState(0i32);
pub const extendedIsolationStateTransition: ExtendedIsolationState = ExtendedIsolationState(1i32);
pub const extendedIsolationStateInfected: ExtendedIsolationState = ExtendedIsolationState(2i32);
pub const extendedIsolationStateUnknown: ExtendedIsolationState = ExtendedIsolationState(3i32);
impl ::std::convert::From<i32> for ExtendedIsolationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedIsolationState {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FailureCategory(pub i32);
pub const failureCategoryNone: FailureCategory = FailureCategory(0i32);
pub const failureCategoryOther: FailureCategory = FailureCategory(1i32);
pub const failureCategoryClientComponent: FailureCategory = FailureCategory(2i32);
pub const failureCategoryClientCommunication: FailureCategory = FailureCategory(3i32);
pub const failureCategoryServerComponent: FailureCategory = FailureCategory(4i32);
pub const failureCategoryServerCommunication: FailureCategory = FailureCategory(5i32);
impl ::std::convert::From<i32> for FailureCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FailureCategory {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
#[cfg(feature = "Win32_Foundation")]
impl FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FailureCategoryMapping {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FailureCategoryMapping").field("mappingCompliance", &self.mappingCompliance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FailureCategoryMapping {
    fn eq(&self, other: &Self) -> bool {
        self.mappingCompliance == other.mappingCompliance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FailureCategoryMapping {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl FixupInfo {}
impl ::std::default::Default for FixupInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FixupInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FixupInfo").field("state", &self.state).field("percentage", &self.percentage).field("resultCodes", &self.resultCodes).field("fixupMsgId", &self.fixupMsgId).finish()
    }
}
impl ::std::cmp::PartialEq for FixupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage && self.resultCodes == other.resultCodes && self.fixupMsgId == other.fixupMsgId
    }
}
impl ::std::cmp::Eq for FixupInfo {}
unsafe impl ::windows::runtime::Abi for FixupInfo {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FixupState(pub i32);
pub const fixupStateSuccess: FixupState = FixupState(0i32);
pub const fixupStateInProgress: FixupState = FixupState(1i32);
pub const fixupStateCouldNotUpdate: FixupState = FixupState(2i32);
impl ::std::convert::From<i32> for FixupState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FixupState {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl Ipv4Address {}
impl ::std::default::Default for Ipv4Address {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Ipv4Address {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Ipv4Address").field("addr", &self.addr).finish()
    }
}
impl ::std::cmp::PartialEq for Ipv4Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::std::cmp::Eq for Ipv4Address {}
unsafe impl ::windows::runtime::Abi for Ipv4Address {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl Ipv6Address {}
impl ::std::default::Default for Ipv6Address {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Ipv6Address {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Ipv6Address").field("addr", &self.addr).finish()
    }
}
impl ::std::cmp::PartialEq for Ipv6Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::std::cmp::Eq for Ipv6Address {}
unsafe impl ::windows::runtime::Abi for Ipv6Address {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IsolationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IsolationInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IsolationInfo").field("isolationState", &self.isolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IsolationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IsolationInfo {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IsolationInfoEx {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IsolationInfoEx {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IsolationInfoEx").field("isolationState", &self.isolationState).field("extendedIsolationState", &self.extendedIsolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IsolationInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.isolationState == other.isolationState && self.extendedIsolationState == other.extendedIsolationState && self.probEndTime == other.probEndTime && self.failureUrl == other.failureUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IsolationInfoEx {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IsolationState(pub i32);
pub const isolationStateNotRestricted: IsolationState = IsolationState(1i32);
pub const isolationStateInProbation: IsolationState = IsolationState(2i32);
pub const isolationStateRestrictedAccess: IsolationState = IsolationState(3i32);
impl ::std::convert::From<i32> for IsolationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IsolationState {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: ::windows::runtime::GUID,
    pub configClsid: ::windows::runtime::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NapComponentRegistrationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NapComponentRegistrationInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NapComponentRegistrationInfo")
            .field("id", &self.id)
            .field("friendlyName", &self.friendlyName)
            .field("description", &self.description)
            .field("version", &self.version)
            .field("vendorName", &self.vendorName)
            .field("infoClsid", &self.infoClsid)
            .field("configClsid", &self.configClsid)
            .field("registrationDate", &self.registrationDate)
            .field("componentType", &self.componentType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NapComponentRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.friendlyName == other.friendlyName && self.description == other.description && self.version == other.version && self.vendorName == other.vendorName && self.infoClsid == other.infoClsid && self.configClsid == other.configClsid && self.registrationDate == other.registrationDate && self.componentType == other.componentType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NapComponentRegistrationInfo {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NapNotifyType(pub i32);
pub const napNotifyTypeUnknown: NapNotifyType = NapNotifyType(0i32);
pub const napNotifyTypeServiceState: NapNotifyType = NapNotifyType(1i32);
pub const napNotifyTypeQuarState: NapNotifyType = NapNotifyType(2i32);
impl ::std::convert::From<i32> for NapNotifyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NapNotifyType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NapTracingLevel(pub i32);
pub const tracingLevelUndefined: NapTracingLevel = NapTracingLevel(0i32);
pub const tracingLevelBasic: NapTracingLevel = NapTracingLevel(1i32);
pub const tracingLevelAdvanced: NapTracingLevel = NapTracingLevel(2i32);
pub const tracingLevelDebug: NapTracingLevel = NapTracingLevel(3i32);
impl ::std::convert::From<i32> for NapTracingLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NapTracingLevel {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl NetworkSoH {}
impl ::std::default::Default for NetworkSoH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NetworkSoH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NetworkSoH").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::std::cmp::PartialEq for NetworkSoH {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::std::cmp::Eq for NetworkSoH {}
unsafe impl ::windows::runtime::Abi for NetworkSoH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl PrivateData {}
impl ::std::default::Default for PrivateData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PrivateData {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PrivateData").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::std::cmp::PartialEq for PrivateData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::std::cmp::Eq for PrivateData {}
unsafe impl ::windows::runtime::Abi for PrivateData {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RemoteConfigurationType(pub i32);
pub const remoteConfigTypeMachine: RemoteConfigurationType = RemoteConfigurationType(1i32);
pub const remoteConfigTypeConfigBlob: RemoteConfigurationType = RemoteConfigurationType(2i32);
impl ::std::convert::From<i32> for RemoteConfigurationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteConfigurationType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut ::windows::runtime::HRESULT,
}
impl ResultCodes {}
impl ::std::default::Default for ResultCodes {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ResultCodes {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ResultCodes").field("count", &self.count).field("results", &self.results).finish()
    }
}
impl ::std::cmp::PartialEq for ResultCodes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.results == other.results
    }
}
impl ::std::cmp::Eq for ResultCodes {}
unsafe impl ::windows::runtime::Abi for ResultCodes {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl SoH {}
impl ::std::default::Default for SoH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SoH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SoH").field("count", &self.count).field("attributes", &self.attributes).finish()
    }
}
impl ::std::cmp::PartialEq for SoH {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.attributes == other.attributes
    }
}
impl ::std::cmp::Eq for SoH {}
unsafe impl ::windows::runtime::Abi for SoH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl SoHAttribute {}
impl ::std::default::Default for SoHAttribute {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SoHAttribute {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SoHAttribute").field("r#type", &self.r#type).field("size", &self.size).field("value", &self.value).finish()
    }
}
impl ::std::cmp::PartialEq for SoHAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.value == other.value
    }
}
impl ::std::cmp::Eq for SoHAttribute {}
unsafe impl ::windows::runtime::Abi for SoHAttribute {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl SystemHealthAgentState {}
impl ::std::default::Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SystemHealthAgentState {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SystemHealthAgentState").field("id", &self.id).field("shaResultCodes", &self.shaResultCodes).field("failureCategory", &self.failureCategory).field("fixupInfo", &self.fixupInfo).finish()
    }
}
impl ::std::cmp::PartialEq for SystemHealthAgentState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.shaResultCodes == other.shaResultCodes && self.failureCategory == other.failureCategory && self.fixupInfo == other.fixupInfo
    }
}
impl ::std::cmp::Eq for SystemHealthAgentState {}
unsafe impl ::windows::runtime::Abi for SystemHealthAgentState {
    type Abi = Self;
    type DefaultType = Self;
}
