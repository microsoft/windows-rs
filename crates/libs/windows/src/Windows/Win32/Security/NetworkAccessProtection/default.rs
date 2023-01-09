#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CorrelationId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for CorrelationId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CorrelationId").field("connId", &self.connId).field("timeStamp", &self.timeStamp).finish()
    }
}
impl ::core::default::Default for CountedString {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CountedString {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.string == other.string
    }
}
impl ::core::cmp::Eq for CountedString {}
impl ::core::fmt::Debug for CountedString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CountedString").field("length", &self.length).field("string", &self.string).finish()
    }
}
impl ::core::default::Default for ExtendedIsolationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ExtendedIsolationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedIsolationState").field(&self.0).finish()
    }
}
impl ::core::default::Default for FailureCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FailureCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FailureCategory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FailureCategoryMapping {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for FailureCategoryMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FailureCategoryMapping").field("mappingCompliance", &self.mappingCompliance).finish()
    }
}
impl ::core::default::Default for FixupInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FixupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage && self.resultCodes == other.resultCodes && self.fixupMsgId == other.fixupMsgId
    }
}
impl ::core::cmp::Eq for FixupInfo {}
impl ::core::fmt::Debug for FixupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FixupInfo").field("state", &self.state).field("percentage", &self.percentage).field("resultCodes", &self.resultCodes).field("fixupMsgId", &self.fixupMsgId).finish()
    }
}
impl ::core::default::Default for FixupState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FixupState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FixupState").field(&self.0).finish()
    }
}
impl ::core::default::Default for Ipv4Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Ipv4Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::core::cmp::Eq for Ipv4Address {}
impl ::core::fmt::Debug for Ipv4Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Ipv4Address").field("addr", &self.addr).finish()
    }
}
impl ::core::default::Default for Ipv6Address {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Ipv6Address {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl ::core::cmp::Eq for Ipv6Address {}
impl ::core::fmt::Debug for Ipv6Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Ipv6Address").field("addr", &self.addr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolationInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for IsolationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolationInfo").field("isolationState", &self.isolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolationInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for IsolationInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolationInfoEx").field("isolationState", &self.isolationState).field("extendedIsolationState", &self.extendedIsolationState).field("probEndTime", &self.probEndTime).field("failureUrl", &self.failureUrl).finish()
    }
}
impl ::core::default::Default for IsolationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IsolationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolationState").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NapComponentRegistrationInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for NapComponentRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NapComponentRegistrationInfo").field("id", &self.id).field("friendlyName", &self.friendlyName).field("description", &self.description).field("version", &self.version).field("vendorName", &self.vendorName).field("infoClsid", &self.infoClsid).field("configClsid", &self.configClsid).field("registrationDate", &self.registrationDate).field("componentType", &self.componentType).finish()
    }
}
impl ::core::default::Default for NapNotifyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NapNotifyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NapNotifyType").field(&self.0).finish()
    }
}
impl ::core::default::Default for NapTracingLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NapTracingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NapTracingLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for NetworkSoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NetworkSoH {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for NetworkSoH {}
impl ::core::fmt::Debug for NetworkSoH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NetworkSoH").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for PrivateData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PrivateData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for PrivateData {}
impl ::core::fmt::Debug for PrivateData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrivateData").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for RemoteConfigurationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RemoteConfigurationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteConfigurationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ResultCodes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ResultCodes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.results == other.results
    }
}
impl ::core::cmp::Eq for ResultCodes {}
impl ::core::fmt::Debug for ResultCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResultCodes").field("count", &self.count).field("results", &self.results).finish()
    }
}
impl ::core::default::Default for SoH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SoH {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.attributes == other.attributes
    }
}
impl ::core::cmp::Eq for SoH {}
impl ::core::fmt::Debug for SoH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SoH").field("count", &self.count).field("attributes", &self.attributes).finish()
    }
}
impl ::core::default::Default for SoHAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SoHAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.value == other.value
    }
}
impl ::core::cmp::Eq for SoHAttribute {}
impl ::core::fmt::Debug for SoHAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SoHAttribute").field("type", &self.r#type).field("size", &self.size).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for SystemHealthAgentState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SystemHealthAgentState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.shaResultCodes == other.shaResultCodes && self.failureCategory == other.failureCategory && self.fixupInfo == other.fixupInfo
    }
}
impl ::core::cmp::Eq for SystemHealthAgentState {}
impl ::core::fmt::Debug for SystemHealthAgentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SystemHealthAgentState").field("id", &self.id).field("shaResultCodes", &self.shaResultCodes).field("failureCategory", &self.failureCategory).field("fixupInfo", &self.fixupInfo).finish()
    }
}
