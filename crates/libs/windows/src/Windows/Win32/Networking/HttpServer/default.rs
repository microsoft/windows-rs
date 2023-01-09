impl ::core::default::Default for HTTP2_SETTINGS_LIMITS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP2_SETTINGS_LIMITS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Http2MaxSettingsPerFrame == other.Http2MaxSettingsPerFrame && self.Http2MaxSettingsPerMinute == other.Http2MaxSettingsPerMinute
    }
}
impl ::core::cmp::Eq for HTTP2_SETTINGS_LIMITS_PARAM {}
impl ::core::fmt::Debug for HTTP2_SETTINGS_LIMITS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP2_SETTINGS_LIMITS_PARAM").field("Http2MaxSettingsPerFrame", &self.Http2MaxSettingsPerFrame).field("Http2MaxSettingsPerMinute", &self.Http2MaxSettingsPerMinute).finish()
    }
}
impl ::core::default::Default for HTTP2_WINDOW_SIZE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP2_WINDOW_SIZE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Http2ReceiveWindowSize == other.Http2ReceiveWindowSize
    }
}
impl ::core::cmp::Eq for HTTP2_WINDOW_SIZE_PARAM {}
impl ::core::fmt::Debug for HTTP2_WINDOW_SIZE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP2_WINDOW_SIZE_PARAM").field("Http2ReceiveWindowSize", &self.Http2ReceiveWindowSize).finish()
    }
}
impl ::core::default::Default for HTTPAPI_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTPAPI_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.HttpApiMajorVersion == other.HttpApiMajorVersion && self.HttpApiMinorVersion == other.HttpApiMinorVersion
    }
}
impl ::core::cmp::Eq for HTTPAPI_VERSION {}
impl ::core::fmt::Debug for HTTPAPI_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTPAPI_VERSION").field("HttpApiMajorVersion", &self.HttpApiMajorVersion).field("HttpApiMinorVersion", &self.HttpApiMinorVersion).finish()
    }
}
impl ::core::default::Default for HTTP_503_RESPONSE_VERBOSITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_503_RESPONSE_VERBOSITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_503_RESPONSE_VERBOSITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_AUTHENTICATION_HARDENING_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_AUTH_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_BANDWIDTH_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_BANDWIDTH_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxBandwidth == other.MaxBandwidth
    }
}
impl ::core::cmp::Eq for HTTP_BANDWIDTH_LIMIT_INFO {}
impl ::core::fmt::Debug for HTTP_BANDWIDTH_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BANDWIDTH_LIMIT_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequestQueueHandle == other.RequestQueueHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_BINDING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BINDING_INFO").field("Flags", &self.Flags).field("RequestQueueHandle", &self.RequestQueueHandle).finish()
    }
}
impl ::core::default::Default for HTTP_BYTE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_BYTE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for HTTP_BYTE_RANGE {}
impl ::core::fmt::Debug for HTTP_BYTE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_BYTE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for HTTP_CACHE_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_CACHE_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Policy == other.Policy && self.SecondsToLive == other.SecondsToLive
    }
}
impl ::core::cmp::Eq for HTTP_CACHE_POLICY {}
impl ::core::fmt::Debug for HTTP_CACHE_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CACHE_POLICY").field("Policy", &self.Policy).field("SecondsToLive", &self.SecondsToLive).finish()
    }
}
impl ::core::default::Default for HTTP_CACHE_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_CACHE_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_CACHE_POLICY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_CHANNEL_BIND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_CHANNEL_BIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Hardening == other.Hardening && self.Flags == other.Flags && self.ServiceNames == other.ServiceNames && self.NumberOfServiceNames == other.NumberOfServiceNames
    }
}
impl ::core::cmp::Eq for HTTP_CHANNEL_BIND_INFO {}
impl ::core::fmt::Debug for HTTP_CHANNEL_BIND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CHANNEL_BIND_INFO").field("Hardening", &self.Hardening).field("Flags", &self.Flags).field("ServiceNames", &self.ServiceNames).field("NumberOfServiceNames", &self.NumberOfServiceNames).finish()
    }
}
impl ::core::default::Default for HTTP_CONNECTION_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_CONNECTION_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxConnections == other.MaxConnections
    }
}
impl ::core::cmp::Eq for HTTP_CONNECTION_LIMIT_INFO {}
impl ::core::fmt::Debug for HTTP_CONNECTION_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CONNECTION_LIMIT_INFO").field("Flags", &self.Flags).field("MaxConnections", &self.MaxConnections).finish()
    }
}
impl ::core::default::Default for HTTP_COOKED_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_COOKED_URL {
    fn eq(&self, other: &Self) -> bool {
        self.FullUrlLength == other.FullUrlLength && self.HostLength == other.HostLength && self.AbsPathLength == other.AbsPathLength && self.QueryStringLength == other.QueryStringLength && self.pFullUrl == other.pFullUrl && self.pHost == other.pHost && self.pAbsPath == other.pAbsPath && self.pQueryString == other.pQueryString
    }
}
impl ::core::cmp::Eq for HTTP_COOKED_URL {}
impl ::core::fmt::Debug for HTTP_COOKED_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_COOKED_URL").field("FullUrlLength", &self.FullUrlLength).field("HostLength", &self.HostLength).field("AbsPathLength", &self.AbsPathLength).field("QueryStringLength", &self.QueryStringLength).field("pFullUrl", &self.pFullUrl).field("pHost", &self.pHost).field("pAbsPath", &self.pAbsPath).field("pQueryString", &self.pQueryString).finish()
    }
}
impl ::core::default::Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.PropertyInfoLength == other.PropertyInfoLength && self.PropertyInfo == other.PropertyInfo
    }
}
impl ::core::cmp::Eq for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {}
impl ::core::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HTTP_DATA_CHUNK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_DATA_CHUNK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_DATA_CHUNK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_DELEGATE_REQUEST_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.PropertyInfoLength == other.PropertyInfoLength && self.PropertyInfo == other.PropertyInfo
    }
}
impl ::core::cmp::Eq for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {}
impl ::core::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_DELEGATE_REQUEST_PROPERTY_INFO").field("PropertyId", &self.PropertyId).field("PropertyInfoLength", &self.PropertyInfoLength).field("PropertyInfo", &self.PropertyInfo).finish()
    }
}
impl ::core::default::Default for HTTP_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_ENABLED_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_ERROR_HEADERS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_ERROR_HEADERS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.StatusCode == other.StatusCode && self.HeaderCount == other.HeaderCount && self.Headers == other.Headers
    }
}
impl ::core::cmp::Eq for HTTP_ERROR_HEADERS_PARAM {}
impl ::core::fmt::Debug for HTTP_ERROR_HEADERS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_ERROR_HEADERS_PARAM").field("StatusCode", &self.StatusCode).field("HeaderCount", &self.HeaderCount).field("Headers", &self.Headers).finish()
    }
}
impl ::core::default::Default for HTTP_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_FEATURE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_FLOWRATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FLOWRATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxBandwidth == other.MaxBandwidth && self.MaxPeakBandwidth == other.MaxPeakBandwidth && self.BurstSize == other.BurstSize
    }
}
impl ::core::cmp::Eq for HTTP_FLOWRATE_INFO {}
impl ::core::fmt::Debug for HTTP_FLOWRATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FLOWRATE_INFO").field("Flags", &self.Flags).field("MaxBandwidth", &self.MaxBandwidth).field("MaxPeakBandwidth", &self.MaxPeakBandwidth).field("BurstSize", &self.BurstSize).finish()
    }
}
impl ::core::default::Default for HTTP_HEADER_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_HEADER_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_HEADER_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_INITIALIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_INITIALIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_INITIALIZE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HTTP_INITIALIZE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HTTP_INITIALIZE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HTTP_INITIALIZE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HTTP_INITIALIZE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HTTP_INITIALIZE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HTTP_KNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_KNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RawValueLength == other.RawValueLength && self.pRawValue == other.pRawValue
    }
}
impl ::core::cmp::Eq for HTTP_KNOWN_HEADER {}
impl ::core::fmt::Debug for HTTP_KNOWN_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_KNOWN_HEADER").field("RawValueLength", &self.RawValueLength).field("pRawValue", &self.pRawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_LISTEN_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_LISTEN_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EnableSharing == other.EnableSharing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_LISTEN_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_LISTEN_ENDPOINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LISTEN_ENDPOINT_INFO").field("Flags", &self.Flags).field("EnableSharing", &self.EnableSharing).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for HTTP_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for HTTP_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.LoggingFlags == other.LoggingFlags && self.SoftwareName == other.SoftwareName && self.SoftwareNameLength == other.SoftwareNameLength && self.DirectoryNameLength == other.DirectoryNameLength && self.DirectoryName == other.DirectoryName && self.Format == other.Format && self.Fields == other.Fields && self.pExtFields == other.pExtFields && self.NumOfExtFields == other.NumOfExtFields && self.MaxRecordSize == other.MaxRecordSize && self.RolloverType == other.RolloverType && self.RolloverSize == other.RolloverSize && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for HTTP_LOGGING_INFO {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for HTTP_LOGGING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOGGING_INFO")
            .field("Flags", &self.Flags)
            .field("LoggingFlags", &self.LoggingFlags)
            .field("SoftwareName", &self.SoftwareName)
            .field("SoftwareNameLength", &self.SoftwareNameLength)
            .field("DirectoryNameLength", &self.DirectoryNameLength)
            .field("DirectoryName", &self.DirectoryName)
            .field("Format", &self.Format)
            .field("Fields", &self.Fields)
            .field("pExtFields", &self.pExtFields)
            .field("NumOfExtFields", &self.NumOfExtFields)
            .field("MaxRecordSize", &self.MaxRecordSize)
            .field("RolloverType", &self.RolloverType)
            .field("RolloverSize", &self.RolloverSize)
            .field("pSecurityDescriptor", &self.pSecurityDescriptor)
            .finish()
    }
}
impl ::core::default::Default for HTTP_LOGGING_ROLLOVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_LOGGING_ROLLOVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_ROLLOVER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_LOGGING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_LOGGING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_LOG_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_LOG_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::core::cmp::Eq for HTTP_LOG_DATA {}
impl ::core::fmt::Debug for HTTP_LOG_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOG_DATA").field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for HTTP_LOG_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_LOG_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_LOG_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_LOG_FIELDS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_LOG_FIELDS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base
            && self.UserNameLength == other.UserNameLength
            && self.UriStemLength == other.UriStemLength
            && self.ClientIpLength == other.ClientIpLength
            && self.ServerNameLength == other.ServerNameLength
            && self.ServiceNameLength == other.ServiceNameLength
            && self.ServerIpLength == other.ServerIpLength
            && self.MethodLength == other.MethodLength
            && self.UriQueryLength == other.UriQueryLength
            && self.HostLength == other.HostLength
            && self.UserAgentLength == other.UserAgentLength
            && self.CookieLength == other.CookieLength
            && self.ReferrerLength == other.ReferrerLength
            && self.UserName == other.UserName
            && self.UriStem == other.UriStem
            && self.ClientIp == other.ClientIp
            && self.ServerName == other.ServerName
            && self.ServiceName == other.ServiceName
            && self.ServerIp == other.ServerIp
            && self.Method == other.Method
            && self.UriQuery == other.UriQuery
            && self.Host == other.Host
            && self.UserAgent == other.UserAgent
            && self.Cookie == other.Cookie
            && self.Referrer == other.Referrer
            && self.ServerPort == other.ServerPort
            && self.ProtocolStatus == other.ProtocolStatus
            && self.Win32Status == other.Win32Status
            && self.MethodNum == other.MethodNum
            && self.SubStatus == other.SubStatus
    }
}
impl ::core::cmp::Eq for HTTP_LOG_FIELDS_DATA {}
impl ::core::fmt::Debug for HTTP_LOG_FIELDS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_LOG_FIELDS_DATA")
            .field("Base", &self.Base)
            .field("UserNameLength", &self.UserNameLength)
            .field("UriStemLength", &self.UriStemLength)
            .field("ClientIpLength", &self.ClientIpLength)
            .field("ServerNameLength", &self.ServerNameLength)
            .field("ServiceNameLength", &self.ServiceNameLength)
            .field("ServerIpLength", &self.ServerIpLength)
            .field("MethodLength", &self.MethodLength)
            .field("UriQueryLength", &self.UriQueryLength)
            .field("HostLength", &self.HostLength)
            .field("UserAgentLength", &self.UserAgentLength)
            .field("CookieLength", &self.CookieLength)
            .field("ReferrerLength", &self.ReferrerLength)
            .field("UserName", &self.UserName)
            .field("UriStem", &self.UriStem)
            .field("ClientIp", &self.ClientIp)
            .field("ServerName", &self.ServerName)
            .field("ServiceName", &self.ServiceName)
            .field("ServerIp", &self.ServerIp)
            .field("Method", &self.Method)
            .field("UriQuery", &self.UriQuery)
            .field("Host", &self.Host)
            .field("UserAgent", &self.UserAgent)
            .field("Cookie", &self.Cookie)
            .field("Referrer", &self.Referrer)
            .field("ServerPort", &self.ServerPort)
            .field("ProtocolStatus", &self.ProtocolStatus)
            .field("Win32Status", &self.Win32Status)
            .field("MethodNum", &self.MethodNum)
            .field("SubStatus", &self.SubStatus)
            .finish()
    }
}
impl ::core::default::Default for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderId == other.HeaderId && self.Flags == other.Flags && self.KnownHeaderCount == other.KnownHeaderCount && self.KnownHeaders == other.KnownHeaders
    }
}
impl ::core::cmp::Eq for HTTP_MULTIPLE_KNOWN_HEADERS {}
impl ::core::fmt::Debug for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_MULTIPLE_KNOWN_HEADERS").field("HeaderId", &self.HeaderId).field("Flags", &self.Flags).field("KnownHeaderCount", &self.KnownHeaderCount).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
impl ::core::default::Default for HTTP_PERFORMANCE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_PERFORMANCE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for HTTP_PERFORMANCE_PARAM {}
impl ::core::fmt::Debug for HTTP_PERFORMANCE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PERFORMANCE_PARAM").field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for HTTP_PERFORMANCE_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_PERFORMANCE_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PERFORMANCE_PARAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_PROPERTY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_PROPERTY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for HTTP_PROPERTY_FLAGS {}
impl ::core::fmt::Debug for HTTP_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PROPERTY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for HTTP_PROTECTION_LEVEL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_PROTECTION_LEVEL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for HTTP_PROTECTION_LEVEL_INFO {}
impl ::core::fmt::Debug for HTTP_PROTECTION_LEVEL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PROTECTION_LEVEL_INFO").field("Flags", &self.Flags).field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for HTTP_PROTECTION_LEVEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_PROTECTION_LEVEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PROTECTION_LEVEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_QOS_SETTING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QOS_SETTING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.QosType == other.QosType && self.QosSetting == other.QosSetting
    }
}
impl ::core::cmp::Eq for HTTP_QOS_SETTING_INFO {}
impl ::core::fmt::Debug for HTTP_QOS_SETTING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QOS_SETTING_INFO").field("QosType", &self.QosType).field("QosSetting", &self.QosSetting).finish()
    }
}
impl ::core::default::Default for HTTP_QOS_SETTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_QOS_SETTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_QOS_SETTING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn eq(&self, other: &Self) -> bool {
        self.Freshness == other.Freshness
    }
}
impl ::core::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {}
impl ::core::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_QUIC").field("Freshness", &self.Freshness).finish()
    }
}
impl ::core::default::Default for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn eq(&self, other: &Self) -> bool {
        self.Freshness == other.Freshness
    }
}
impl ::core::cmp::Eq for HTTP_QUERY_REQUEST_QUALIFIER_TCP {}
impl ::core::fmt::Debug for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUERY_REQUEST_QUALIFIER_TCP").field("Freshness", &self.Freshness).finish()
    }
}
impl ::core::default::Default for HTTP_QUIC_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QUIC_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionTimings == other.ConnectionTimings && self.StreamTimings == other.StreamTimings
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_API_TIMINGS {}
impl ::core::fmt::Debug for HTTP_QUIC_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_API_TIMINGS").field("ConnectionTimings", &self.ConnectionTimings).field("StreamTimings", &self.StreamTimings).finish()
    }
}
impl ::core::default::Default for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.OpenTime == other.OpenTime && self.CloseTime == other.CloseTime && self.StartTime == other.StartTime && self.ShutdownTime == other.ShutdownTime && self.SecConfigCreateTime == other.SecConfigCreateTime && self.SecConfigDeleteTime == other.SecConfigDeleteTime && self.GetParamCount == other.GetParamCount && self.GetParamSum == other.GetParamSum && self.SetParamCount == other.SetParamCount && self.SetParamSum == other.SetParamSum && self.SetCallbackHandlerCount == other.SetCallbackHandlerCount && self.SetCallbackHandlerSum == other.SetCallbackHandlerSum && self.ControlStreamTimings == other.ControlStreamTimings
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_CONNECTION_API_TIMINGS {}
impl ::core::fmt::Debug for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_CONNECTION_API_TIMINGS")
            .field("OpenTime", &self.OpenTime)
            .field("CloseTime", &self.CloseTime)
            .field("StartTime", &self.StartTime)
            .field("ShutdownTime", &self.ShutdownTime)
            .field("SecConfigCreateTime", &self.SecConfigCreateTime)
            .field("SecConfigDeleteTime", &self.SecConfigDeleteTime)
            .field("GetParamCount", &self.GetParamCount)
            .field("GetParamSum", &self.GetParamSum)
            .field("SetParamCount", &self.SetParamCount)
            .field("SetParamSum", &self.SetParamSum)
            .field("SetCallbackHandlerCount", &self.SetCallbackHandlerCount)
            .field("SetCallbackHandlerSum", &self.SetCallbackHandlerSum)
            .field("ControlStreamTimings", &self.ControlStreamTimings)
            .finish()
    }
}
impl ::core::default::Default for HTTP_QUIC_STREAM_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_QUIC_STREAM_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.OpenCount == other.OpenCount && self.OpenSum == other.OpenSum && self.CloseCount == other.CloseCount && self.CloseSum == other.CloseSum && self.StartCount == other.StartCount && self.StartSum == other.StartSum && self.ShutdownCount == other.ShutdownCount && self.ShutdownSum == other.ShutdownSum && self.SendCount == other.SendCount && self.SendSum == other.SendSum && self.ReceiveSetEnabledCount == other.ReceiveSetEnabledCount && self.ReceiveSetEnabledSum == other.ReceiveSetEnabledSum && self.GetParamCount == other.GetParamCount && self.GetParamSum == other.GetParamSum && self.SetParamCount == other.SetParamCount && self.SetParamSum == other.SetParamSum && self.SetCallbackHandlerCount == other.SetCallbackHandlerCount && self.SetCallbackHandlerSum == other.SetCallbackHandlerSum
    }
}
impl ::core::cmp::Eq for HTTP_QUIC_STREAM_API_TIMINGS {}
impl ::core::fmt::Debug for HTTP_QUIC_STREAM_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_QUIC_STREAM_API_TIMINGS")
            .field("OpenCount", &self.OpenCount)
            .field("OpenSum", &self.OpenSum)
            .field("CloseCount", &self.CloseCount)
            .field("CloseSum", &self.CloseSum)
            .field("StartCount", &self.StartCount)
            .field("StartSum", &self.StartSum)
            .field("ShutdownCount", &self.ShutdownCount)
            .field("ShutdownSum", &self.ShutdownSum)
            .field("SendCount", &self.SendCount)
            .field("SendSum", &self.SendSum)
            .field("ReceiveSetEnabledCount", &self.ReceiveSetEnabledCount)
            .field("ReceiveSetEnabledSum", &self.ReceiveSetEnabledSum)
            .field("GetParamCount", &self.GetParamCount)
            .field("GetParamSum", &self.GetParamSum)
            .field("SetParamCount", &self.SetParamCount)
            .field("SetParamSum", &self.SetParamSum)
            .field("SetCallbackHandlerCount", &self.SetCallbackHandlerCount)
            .field("SetCallbackHandlerSum", &self.SetCallbackHandlerSum)
            .finish()
    }
}
impl ::core::default::Default for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_RECEIVE_HTTP_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_REQUEST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_REQUEST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuthStatus == other.AuthStatus && self.SecStatus == other.SecStatus && self.Flags == other.Flags && self.AuthType == other.AuthType && self.AccessToken == other.AccessToken && self.ContextAttributes == other.ContextAttributes && self.PackedContextLength == other.PackedContextLength && self.PackedContextType == other.PackedContextType && self.PackedContext == other.PackedContext && self.MutualAuthDataLength == other.MutualAuthDataLength && self.pMutualAuthData == other.pMutualAuthData && self.PackageNameLength == other.PackageNameLength && self.pPackageName == other.pPackageName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_REQUEST_AUTH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_REQUEST_AUTH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_AUTH_INFO")
            .field("AuthStatus", &self.AuthStatus)
            .field("SecStatus", &self.SecStatus)
            .field("Flags", &self.Flags)
            .field("AuthType", &self.AuthType)
            .field("AccessToken", &self.AccessToken)
            .field("ContextAttributes", &self.ContextAttributes)
            .field("PackedContextLength", &self.PackedContextLength)
            .field("PackedContextType", &self.PackedContextType)
            .field("PackedContext", &self.PackedContext)
            .field("MutualAuthDataLength", &self.MutualAuthDataLength)
            .field("pMutualAuthData", &self.pMutualAuthData)
            .field("PackageNameLength", &self.PackageNameLength)
            .field("pPackageName", &self.pPackageName)
            .finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_AUTH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceName == other.ServiceName && self.ChannelToken == other.ChannelToken && self.ChannelTokenSize == other.ChannelTokenSize && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_CHANNEL_BIND_STATUS {}
impl ::core::fmt::Debug for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_CHANNEL_BIND_STATUS").field("ServiceName", &self.ServiceName).field("ChannelToken", &self.ChannelToken).field("ChannelTokenSize", &self.ChannelTokenSize).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnknownHeaderCount == other.UnknownHeaderCount && self.pUnknownHeaders == other.pUnknownHeaders && self.TrailerCount == other.TrailerCount && self.pTrailers == other.pTrailers && self.KnownHeaders == other.KnownHeaders
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_HEADERS {}
impl ::core::fmt::Debug for HTTP_REQUEST_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InfoType == other.InfoType && self.InfoLength == other.InfoLength && self.pInfo == other.pInfo
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_INFO {}
impl ::core::fmt::Debug for HTTP_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_INFO").field("InfoType", &self.InfoType).field("InfoLength", &self.InfoLength).field("pInfo", &self.pInfo).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_PROPERTY_SNI {
    fn eq(&self, other: &Self) -> bool {
        self.Hostname == other.Hostname && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_PROPERTY_SNI {}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY_SNI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_PROPERTY_SNI").field("Hostname", &self.Hostname).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_PROPERTY_STREAM_ERROR {}
impl ::core::fmt::Debug for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_PROPERTY_STREAM_ERROR").field("ErrorCode", &self.ErrorCode).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_SIZING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequestIndex == other.RequestIndex && self.RequestSizingCount == other.RequestSizingCount && self.RequestSizing == other.RequestSizing
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_SIZING_INFO {}
impl ::core::fmt::Debug for HTTP_REQUEST_SIZING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_SIZING_INFO").field("Flags", &self.Flags).field("RequestIndex", &self.RequestIndex).field("RequestSizingCount", &self.RequestSizingCount).field("RequestSizing", &self.RequestSizing).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_SIZING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_SIZING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_SIZING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TIMING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RequestTimingCount == other.RequestTimingCount && self.RequestTiming == other.RequestTiming
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TIMING_INFO {}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TIMING_INFO").field("RequestTimingCount", &self.RequestTimingCount).field("RequestTiming", &self.RequestTiming).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_TIMING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_TIMING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TokenBinding == other.TokenBinding && self.TokenBindingSize == other.TokenBindingSize && self.EKM == other.EKM && self.EKMSize == other.EKMSize && self.KeyType == other.KeyType
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TOKEN_BINDING_INFO {}
impl ::core::fmt::Debug for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TOKEN_BINDING_INFO").field("TokenBinding", &self.TokenBinding).field("TokenBindingSize", &self.TokenBindingSize).field("EKM", &self.EKM).field("EKMSize", &self.EKMSize).field("KeyType", &self.KeyType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_REQUEST_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ConnectionId == other.ConnectionId && self.RequestId == other.RequestId && self.UrlContext == other.UrlContext && self.Version == other.Version && self.Verb == other.Verb && self.UnknownVerbLength == other.UnknownVerbLength && self.RawUrlLength == other.RawUrlLength && self.pUnknownVerb == other.pUnknownVerb && self.pRawUrl == other.pRawUrl && self.CookedUrl == other.CookedUrl && self.Address == other.Address && self.Headers == other.Headers && self.BytesReceived == other.BytesReceived && self.EntityChunkCount == other.EntityChunkCount && self.pEntityChunks == other.pEntityChunks && self.RawConnectionId == other.RawConnectionId && self.pSslInfo == other.pSslInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_REQUEST_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_REQUEST_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_V1")
            .field("Flags", &self.Flags)
            .field("ConnectionId", &self.ConnectionId)
            .field("RequestId", &self.RequestId)
            .field("UrlContext", &self.UrlContext)
            .field("Version", &self.Version)
            .field("Verb", &self.Verb)
            .field("UnknownVerbLength", &self.UnknownVerbLength)
            .field("RawUrlLength", &self.RawUrlLength)
            .field("pUnknownVerb", &self.pUnknownVerb)
            .field("pRawUrl", &self.pRawUrl)
            .field("CookedUrl", &self.CookedUrl)
            .field("Address", &self.Address)
            .field("Headers", &self.Headers)
            .field("BytesReceived", &self.BytesReceived)
            .field("EntityChunkCount", &self.EntityChunkCount)
            .field("pEntityChunks", &self.pEntityChunks)
            .field("RawConnectionId", &self.RawConnectionId)
            .field("pSslInfo", &self.pSslInfo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.RequestInfoCount == other.RequestInfoCount && self.pRequestInfo == other.pRequestInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_REQUEST_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_REQUEST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_V2").field("Base", &self.Base).field("RequestInfoCount", &self.RequestInfoCount).field("pRequestInfo", &self.pRequestInfo).finish()
    }
}
impl ::core::default::Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_RESPONSE_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnknownHeaderCount == other.UnknownHeaderCount && self.pUnknownHeaders == other.pUnknownHeaders && self.TrailerCount == other.TrailerCount && self.pTrailers == other.pTrailers && self.KnownHeaders == other.KnownHeaders
    }
}
impl ::core::cmp::Eq for HTTP_RESPONSE_HEADERS {}
impl ::core::fmt::Debug for HTTP_RESPONSE_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_HEADERS").field("UnknownHeaderCount", &self.UnknownHeaderCount).field("pUnknownHeaders", &self.pUnknownHeaders).field("TrailerCount", &self.TrailerCount).field("pTrailers", &self.pTrailers).field("KnownHeaders", &self.KnownHeaders).finish()
    }
}
impl ::core::default::Default for HTTP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Length == other.Length && self.pInfo == other.pInfo
    }
}
impl ::core::cmp::Eq for HTTP_RESPONSE_INFO {}
impl ::core::fmt::Debug for HTTP_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_INFO").field("Type", &self.Type).field("Length", &self.Length).field("pInfo", &self.pInfo).finish()
    }
}
impl ::core::default::Default for HTTP_RESPONSE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_RESPONSE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_RESPONSE_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_RESPONSE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_RESPONSE_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Version == other.Version && self.StatusCode == other.StatusCode && self.ReasonLength == other.ReasonLength && self.pReason == other.pReason && self.Headers == other.Headers && self.EntityChunkCount == other.EntityChunkCount && self.pEntityChunks == other.pEntityChunks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_RESPONSE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_RESPONSE_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_V1").field("Flags", &self.Flags).field("Version", &self.Version).field("StatusCode", &self.StatusCode).field("ReasonLength", &self.ReasonLength).field("pReason", &self.pReason).field("Headers", &self.Headers).field("EntityChunkCount", &self.EntityChunkCount).field("pEntityChunks", &self.pEntityChunks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_RESPONSE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_RESPONSE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.ResponseInfoCount == other.ResponseInfoCount && self.pResponseInfo == other.pResponseInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_RESPONSE_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_RESPONSE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_RESPONSE_V2").field("Base", &self.Base).field("ResponseInfoCount", &self.ResponseInfoCount).field("pResponseInfo", &self.pResponseInfo).finish()
    }
}
impl ::core::default::Default for HTTP_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SCHEME").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.RealmLength == other.RealmLength && self.Realm == other.Realm
    }
}
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {}
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS").field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.DomainNameLength == other.DomainNameLength && self.DomainName == other.DomainName && self.RealmLength == other.RealmLength && self.Realm == other.Realm
    }
}
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {}
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS").field("DomainNameLength", &self.DomainNameLength).field("DomainName", &self.DomainName).field("RealmLength", &self.RealmLength).field("Realm", &self.Realm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SERVER_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SERVER_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.AuthSchemes == other.AuthSchemes && self.ReceiveMutualAuth == other.ReceiveMutualAuth && self.ReceiveContextHandle == other.ReceiveContextHandle && self.DisableNTLMCredentialCaching == other.DisableNTLMCredentialCaching && self.ExFlags == other.ExFlags && self.DigestParams == other.DigestParams && self.BasicParams == other.BasicParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SERVER_AUTHENTICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SERVER_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVER_AUTHENTICATION_INFO").field("Flags", &self.Flags).field("AuthSchemes", &self.AuthSchemes).field("ReceiveMutualAuth", &self.ReceiveMutualAuth).field("ReceiveContextHandle", &self.ReceiveContextHandle).field("DisableNTLMCredentialCaching", &self.DisableNTLMCredentialCaching).field("ExFlags", &self.ExFlags).field("DigestParams", &self.DigestParams).field("BasicParams", &self.BasicParams).finish()
    }
}
impl ::core::default::Default for HTTP_SERVER_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVER_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_BINDING_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_A {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_A {}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_A").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_BINDING_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_BASE {}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_BASE").field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_BINDING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_BINDING_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_BINDING_W {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_BINDING_W {}
impl ::core::fmt::Debug for HTTP_SERVICE_BINDING_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_BINDING_W").field("Base", &self.Base).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_CACHE_KEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_CACHE_SET {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_CACHE_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_ID").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.AddrLength == other.AddrLength && self.pAddress == other.pAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM").field("AddrLength", &self.AddrLength).field("pAddress", &self.pAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.AddrCount == other.AddrCount && self.AddrList == other.AddrList
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY").field("AddrCount", &self.AddrCount).field("AddrList", &self.AddrList).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_QUERY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_SETTING_KEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SETTING_SET {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SETTING_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddress == other.LocalAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_KEY").field("LocalAddress", &self.LocalAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_CCS_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_CCS_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pIpPort == other.pIpPort
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY").field("pIpPort", &self.pIpPort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.IpPort == other.IpPort
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_KEY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_KEY_EX").field("IpPort", &self.IpPort).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.SslHashLength == other.SslHashLength && self.pSslHash == other.pSslHash && self.AppId == other.AppId && self.pSslCertStoreName == other.pSslCertStoreName && self.DefaultCertCheckMode == other.DefaultCertCheckMode && self.DefaultRevocationFreshnessTime == other.DefaultRevocationFreshnessTime && self.DefaultRevocationUrlRetrievalTimeout == other.DefaultRevocationUrlRetrievalTimeout && self.pDefaultSslCtlIdentifier == other.pDefaultSslCtlIdentifier && self.pDefaultSslCtlStoreName == other.pDefaultSslCtlStoreName && self.DefaultFlags == other.DefaultFlags
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_PARAM {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_PARAM")
            .field("SslHashLength", &self.SslHashLength)
            .field("pSslHash", &self.pSslHash)
            .field("AppId", &self.AppId)
            .field("pSslCertStoreName", &self.pSslCertStoreName)
            .field("DefaultCertCheckMode", &self.DefaultCertCheckMode)
            .field("DefaultRevocationFreshnessTime", &self.DefaultRevocationFreshnessTime)
            .field("DefaultRevocationUrlRetrievalTimeout", &self.DefaultRevocationUrlRetrievalTimeout)
            .field("pDefaultSslCtlIdentifier", &self.pDefaultSslCtlIdentifier)
            .field("pDefaultSslCtlStoreName", &self.pDefaultSslCtlStoreName)
            .field("DefaultFlags", &self.DefaultFlags)
            .finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.IpPort == other.IpPort && self.Host == other.Host
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_KEY").field("IpPort", &self.IpPort).field("Host", &self.Host).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken && self.ParamType == other.ParamType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).field("ParamType", &self.ParamType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_SSL_SNI_SET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_SSL_SNI_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_TIMEOUT_KEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_TIMEOUT_SET {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_TIMEOUT_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pUrlPrefix == other.pUrlPrefix
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_KEY {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_KEY").field("pUrlPrefix", &self.pUrlPrefix).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.pStringSecurityDescriptor == other.pStringSecurityDescriptor
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_PARAM {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_PARAM").field("pStringSecurityDescriptor", &self.pStringSecurityDescriptor).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.QueryDesc == other.QueryDesc && self.KeyDesc == other.KeyDesc && self.dwToken == other.dwToken
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_QUERY {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_QUERY").field("QueryDesc", &self.QueryDesc).field("KeyDesc", &self.KeyDesc).field("dwToken", &self.dwToken).finish()
    }
}
impl ::core::default::Default for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn eq(&self, other: &Self) -> bool {
        self.KeyDesc == other.KeyDesc && self.ParamDesc == other.ParamDesc
    }
}
impl ::core::cmp::Eq for HTTP_SERVICE_CONFIG_URLACL_SET {}
impl ::core::fmt::Debug for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SERVICE_CONFIG_URLACL_SET").field("KeyDesc", &self.KeyDesc).field("ParamDesc", &self.ParamDesc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SSL_CLIENT_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SSL_CLIENT_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CertFlags == other.CertFlags && self.CertEncodedSize == other.CertEncodedSize && self.pCertEncoded == other.pCertEncoded && self.Token == other.Token && self.CertDeniedByMapper == other.CertDeniedByMapper
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SSL_CLIENT_CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SSL_CLIENT_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_CLIENT_CERT_INFO").field("CertFlags", &self.CertFlags).field("CertEncodedSize", &self.CertEncodedSize).field("pCertEncoded", &self.pCertEncoded).field("Token", &self.Token).field("CertDeniedByMapper", &self.CertDeniedByMapper).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_SSL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_SSL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ServerCertKeySize == other.ServerCertKeySize && self.ConnectionKeySize == other.ConnectionKeySize && self.ServerCertIssuerSize == other.ServerCertIssuerSize && self.ServerCertSubjectSize == other.ServerCertSubjectSize && self.pServerCertIssuer == other.pServerCertIssuer && self.pServerCertSubject == other.pServerCertSubject && self.pClientCertInfo == other.pClientCertInfo && self.SslClientCertNegotiated == other.SslClientCertNegotiated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_SSL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_SSL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_INFO").field("ServerCertKeySize", &self.ServerCertKeySize).field("ConnectionKeySize", &self.ConnectionKeySize).field("ServerCertIssuerSize", &self.ServerCertIssuerSize).field("ServerCertSubjectSize", &self.ServerCertSubjectSize).field("pServerCertIssuer", &self.pServerCertIssuer).field("pServerCertSubject", &self.pServerCertSubject).field("pClientCertInfo", &self.pClientCertInfo).field("SslClientCertNegotiated", &self.SslClientCertNegotiated).finish()
    }
}
impl ::core::default::Default for HTTP_SSL_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_SSL_PROTOCOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Protocol == other.Protocol && self.CipherType == other.CipherType && self.CipherStrength == other.CipherStrength && self.HashType == other.HashType && self.HashStrength == other.HashStrength && self.KeyExchangeType == other.KeyExchangeType && self.KeyExchangeStrength == other.KeyExchangeStrength
    }
}
impl ::core::cmp::Eq for HTTP_SSL_PROTOCOL_INFO {}
impl ::core::fmt::Debug for HTTP_SSL_PROTOCOL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_SSL_PROTOCOL_INFO").field("Protocol", &self.Protocol).field("CipherType", &self.CipherType).field("CipherStrength", &self.CipherStrength).field("HashType", &self.HashType).field("HashStrength", &self.HashStrength).field("KeyExchangeType", &self.KeyExchangeType).field("KeyExchangeStrength", &self.KeyExchangeStrength).finish()
    }
}
impl ::core::default::Default for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.State == other.State
    }
}
impl ::core::cmp::Eq for HTTP_STATE_INFO {}
impl ::core::fmt::Debug for HTTP_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_STATE_INFO").field("Flags", &self.Flags).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for HTTP_TIMEOUT_LIMIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_TIMEOUT_LIMIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EntityBody == other.EntityBody && self.DrainEntityBody == other.DrainEntityBody && self.RequestQueue == other.RequestQueue && self.IdleConnection == other.IdleConnection && self.HeaderWait == other.HeaderWait && self.MinSendRate == other.MinSendRate
    }
}
impl ::core::cmp::Eq for HTTP_TIMEOUT_LIMIT_INFO {}
impl ::core::fmt::Debug for HTTP_TIMEOUT_LIMIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TIMEOUT_LIMIT_INFO").field("Flags", &self.Flags).field("EntityBody", &self.EntityBody).field("DrainEntityBody", &self.DrainEntityBody).field("RequestQueue", &self.RequestQueue).field("IdleConnection", &self.IdleConnection).field("HeaderWait", &self.HeaderWait).field("MinSendRate", &self.MinSendRate).finish()
    }
}
impl ::core::default::Default for HTTP_TLS_RESTRICTIONS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_TLS_RESTRICTIONS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.RestrictionCount == other.RestrictionCount && self.TlsRestrictions == other.TlsRestrictions
    }
}
impl ::core::cmp::Eq for HTTP_TLS_RESTRICTIONS_PARAM {}
impl ::core::fmt::Debug for HTTP_TLS_RESTRICTIONS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TLS_RESTRICTIONS_PARAM").field("RestrictionCount", &self.RestrictionCount).field("TlsRestrictions", &self.TlsRestrictions).finish()
    }
}
impl ::core::default::Default for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.SessionTicketKeyCount == other.SessionTicketKeyCount && self.SessionTicketKeys == other.SessionTicketKeys
    }
}
impl ::core::cmp::Eq for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {}
impl ::core::fmt::Debug for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TLS_SESSION_TICKET_KEYS_PARAM").field("SessionTicketKeyCount", &self.SessionTicketKeyCount).field("SessionTicketKeys", &self.SessionTicketKeys).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for HTTP_TRANSPORT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for HTTP_TRANSPORT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.pRemoteAddress == other.pRemoteAddress && self.pLocalAddress == other.pLocalAddress
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for HTTP_TRANSPORT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for HTTP_TRANSPORT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRANSPORT_ADDRESS").field("pRemoteAddress", &self.pRemoteAddress).field("pLocalAddress", &self.pLocalAddress).finish()
    }
}
impl ::core::default::Default for HTTP_UNKNOWN_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_UNKNOWN_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NameLength == other.NameLength && self.RawValueLength == other.RawValueLength && self.pName == other.pName && self.pRawValue == other.pRawValue
    }
}
impl ::core::cmp::Eq for HTTP_UNKNOWN_HEADER {}
impl ::core::fmt::Debug for HTTP_UNKNOWN_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_UNKNOWN_HEADER").field("NameLength", &self.NameLength).field("RawValueLength", &self.RawValueLength).field("pName", &self.pName).field("pRawValue", &self.pRawValue).finish()
    }
}
impl ::core::default::Default for HTTP_VERB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_VERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_VERB").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for HTTP_VERSION {}
impl ::core::fmt::Debug for HTTP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for HTTP_WSK_API_TIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_WSK_API_TIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectCount == other.ConnectCount && self.ConnectSum == other.ConnectSum && self.DisconnectCount == other.DisconnectCount && self.DisconnectSum == other.DisconnectSum && self.SendCount == other.SendCount && self.SendSum == other.SendSum && self.ReceiveCount == other.ReceiveCount && self.ReceiveSum == other.ReceiveSum && self.ReleaseCount == other.ReleaseCount && self.ReleaseSum == other.ReleaseSum && self.ControlSocketCount == other.ControlSocketCount && self.ControlSocketSum == other.ControlSocketSum
    }
}
impl ::core::cmp::Eq for HTTP_WSK_API_TIMINGS {}
impl ::core::fmt::Debug for HTTP_WSK_API_TIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_WSK_API_TIMINGS")
            .field("ConnectCount", &self.ConnectCount)
            .field("ConnectSum", &self.ConnectSum)
            .field("DisconnectCount", &self.DisconnectCount)
            .field("DisconnectSum", &self.DisconnectSum)
            .field("SendCount", &self.SendCount)
            .field("SendSum", &self.SendSum)
            .field("ReceiveCount", &self.ReceiveCount)
            .field("ReceiveSum", &self.ReceiveSum)
            .field("ReleaseCount", &self.ReleaseCount)
            .field("ReleaseSum", &self.ReleaseSum)
            .field("ControlSocketCount", &self.ControlSocketCount)
            .field("ControlSocketSum", &self.ControlSocketSum)
            .finish()
    }
}
