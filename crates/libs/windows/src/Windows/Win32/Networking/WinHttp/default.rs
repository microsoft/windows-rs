impl ::core::default::Default for HTTP_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
impl ::core::cmp::Eq for HTTP_VERSION_INFO {}
impl ::core::fmt::Debug for HTTP_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_VERSION_INFO").field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::core::default::Default for URL_COMPONENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for URL_COMPONENTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTS {}
impl ::core::fmt::Debug for URL_COMPONENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTS")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
impl ::core::default::Default for WINHTTP_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwResult == other.dwResult && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for WINHTTP_ASYNC_RESULT {}
impl ::core::fmt::Debug for WINHTTP_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_ASYNC_RESULT").field("dwResult", &self.dwResult).field("dwError", &self.dwError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_AUTOPROXY_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_AUTOPROXY_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwAutoDetectFlags == other.dwAutoDetectFlags && self.lpszAutoConfigUrl == other.lpszAutoConfigUrl && self.lpvReserved == other.lpvReserved && self.dwReserved == other.dwReserved && self.fAutoLogonIfChallenged == other.fAutoLogonIfChallenged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_AUTOPROXY_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_AUTOPROXY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_AUTOPROXY_OPTIONS").field("dwFlags", &self.dwFlags).field("dwAutoDetectFlags", &self.dwAutoDetectFlags).field("lpszAutoConfigUrl", &self.lpszAutoConfigUrl).field("lpvReserved", &self.lpvReserved).field("dwReserved", &self.dwReserved).field("fAutoLogonIfChallenged", &self.fAutoLogonIfChallenged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpiry == other.ftExpiry && self.ftStart == other.ftStart && self.lpszSubjectInfo == other.lpszSubjectInfo && self.lpszIssuerInfo == other.lpszIssuerInfo && self.lpszProtocolName == other.lpszProtocolName && self.lpszSignatureAlgName == other.lpszSignatureAlgName && self.lpszEncryptionAlgName == other.lpszEncryptionAlgName && self.dwKeySize == other.dwKeySize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_CERTIFICATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CERTIFICATE_INFO").field("ftExpiry", &self.ftExpiry).field("ftStart", &self.ftStart).field("lpszSubjectInfo", &self.lpszSubjectInfo).field("lpszIssuerInfo", &self.lpszIssuerInfo).field("lpszProtocolName", &self.lpszProtocolName).field("lpszSignatureAlgName", &self.lpszSignatureAlgName).field("lpszEncryptionAlgName", &self.lpszEncryptionAlgName).field("dwKeySize", &self.dwKeySize).finish()
    }
}
impl ::core::default::Default for WINHTTP_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_CONNECTION_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.cConnections == other.cConnections && self.guidGroup == other.guidGroup
    }
}
impl ::core::cmp::Eq for WINHTTP_CONNECTION_GROUP {}
impl ::core::fmt::Debug for WINHTTP_CONNECTION_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CONNECTION_GROUP").field("cConnections", &self.cConnections).field("guidGroup", &self.guidGroup).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WINHTTP_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WINHTTP_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_CREDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_CREDS {
    fn eq(&self, other: &Self) -> bool {
        self.lpszUserName == other.lpszUserName && self.lpszPassword == other.lpszPassword && self.lpszRealm == other.lpszRealm && self.dwAuthScheme == other.dwAuthScheme && self.lpszHostName == other.lpszHostName && self.dwPort == other.dwPort
    }
}
impl ::core::cmp::Eq for WINHTTP_CREDS {}
impl ::core::fmt::Debug for WINHTTP_CREDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CREDS").field("lpszUserName", &self.lpszUserName).field("lpszPassword", &self.lpszPassword).field("lpszRealm", &self.lpszRealm).field("dwAuthScheme", &self.dwAuthScheme).field("lpszHostName", &self.lpszHostName).field("dwPort", &self.dwPort).finish()
    }
}
impl ::core::default::Default for WINHTTP_CREDS_AUTHSCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_CREDS_AUTHSCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_CREDS_AUTHSCHEME").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_CREDS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_CREDS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.lpszUserName == other.lpszUserName && self.lpszPassword == other.lpszPassword && self.lpszRealm == other.lpszRealm && self.dwAuthScheme == other.dwAuthScheme && self.lpszHostName == other.lpszHostName && self.dwPort == other.dwPort && self.lpszUrl == other.lpszUrl
    }
}
impl ::core::cmp::Eq for WINHTTP_CREDS_EX {}
impl ::core::fmt::Debug for WINHTTP_CREDS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CREDS_EX").field("lpszUserName", &self.lpszUserName).field("lpszPassword", &self.lpszPassword).field("lpszRealm", &self.lpszRealm).field("dwAuthScheme", &self.dwAuthScheme).field("lpszHostName", &self.lpszHostName).field("dwPort", &self.dwPort).field("lpszUrl", &self.lpszUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.fAutoDetect == other.fAutoDetect && self.lpszAutoConfigUrl == other.lpszAutoConfigUrl && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_CURRENT_USER_IE_PROXY_CONFIG").field("fAutoDetect", &self.fAutoDetect).field("lpszAutoConfigUrl", &self.lpszAutoConfigUrl).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::core::default::Default for WINHTTP_EXTENDED_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxRetries == other.dwMaxRetries && self.dwAllowedRetryConditions == other.dwAllowedRetryConditions
    }
}
impl ::core::cmp::Eq for WINHTTP_FAILED_CONNECTION_RETRIES {}
impl ::core::fmt::Debug for WINHTTP_FAILED_CONNECTION_RETRIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_FAILED_CONNECTION_RETRIES").field("dwMaxRetries", &self.dwMaxRetries).field("dwAllowedRetryConditions", &self.dwAllowedRetryConditions).finish()
    }
}
impl ::core::default::Default for WINHTTP_HEADER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_HOST_CONNECTION_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_HOST_CONNECTION_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.pwszHost == other.pwszHost && self.cConnectionGroups == other.cConnectionGroups && self.pConnectionGroups == other.pConnectionGroups
    }
}
impl ::core::cmp::Eq for WINHTTP_HOST_CONNECTION_GROUP {}
impl ::core::fmt::Debug for WINHTTP_HOST_CONNECTION_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_HOST_CONNECTION_GROUP").field("pwszHost", &self.pwszHost).field("cConnectionGroups", &self.cConnectionGroups).field("pConnectionGroups", &self.pConnectionGroups).finish()
    }
}
impl ::core::default::Default for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.ulStreamWindow == other.ulStreamWindow && self.ulStreamWindowUpdateDelta == other.ulStreamWindowUpdateDelta
    }
}
impl ::core::cmp::Eq for WINHTTP_HTTP2_RECEIVE_WINDOW {}
impl ::core::fmt::Debug for WINHTTP_HTTP2_RECEIVE_WINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_HTTP2_RECEIVE_WINDOW").field("ulStreamWindow", &self.ulStreamWindow).field("ulStreamWindowUpdateDelta", &self.ulStreamWindowUpdateDelta).finish()
    }
}
impl ::core::default::Default for WINHTTP_INTERNET_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_INTERNET_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_INTERNET_SCHEME").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_MATCH_CONNECTION_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_MATCH_CONNECTION_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_OPEN_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_OPEN_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_OPEN_REQUEST_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINHTTP_OPEN_REQUEST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINHTTP_OPEN_REQUEST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WINHTTP_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAccessType == other.dwAccessType && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
impl ::core::cmp::Eq for WINHTTP_PROXY_INFO {}
impl ::core::fmt::Debug for WINHTTP_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_INFO").field("dwAccessType", &self.dwAccessType).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::core::default::Default for WINHTTP_PROXY_NETWORKING_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_PROXY_NETWORKING_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pbBuffer == other.pbBuffer
    }
}
impl ::core::cmp::Eq for WINHTTP_PROXY_NETWORKING_KEY {}
impl ::core::fmt::Debug for WINHTTP_PROXY_NETWORKING_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_NETWORKING_KEY").field("pbBuffer", &self.pbBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.fProxy == other.fProxy && self.fBypass == other.fBypass && self.ProxyScheme == other.ProxyScheme && self.pwszProxy == other.pwszProxy && self.ProxyPort == other.ProxyPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT_ENTRY").field("fProxy", &self.fProxy).field("fBypass", &self.fBypass).field("ProxyScheme", &self.ProxyScheme).field("pwszProxy", &self.pwszProxy).field("ProxyPort", &self.ProxyPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_RESULT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_RESULT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries && self.hProxyDetectionHandle == other.hProxyDetectionHandle && self.dwProxyInterfaceAffinity == other.dwProxyInterfaceAffinity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_RESULT_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_RESULT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_RESULT_EX").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).field("hProxyDetectionHandle", &self.hProxyDetectionHandle).field("dwProxyInterfaceAffinity", &self.dwProxyInterfaceAffinity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINHTTP_PROXY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINHTTP_PROXY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.dwFlags == other.dwFlags
            && self.dwCurrentSettingsVersion == other.dwCurrentSettingsVersion
            && self.pwszConnectionName == other.pwszConnectionName
            && self.pwszProxy == other.pwszProxy
            && self.pwszProxyBypass == other.pwszProxyBypass
            && self.pwszAutoconfigUrl == other.pwszAutoconfigUrl
            && self.pwszAutoconfigSecondaryUrl == other.pwszAutoconfigSecondaryUrl
            && self.dwAutoDiscoveryFlags == other.dwAutoDiscoveryFlags
            && self.pwszLastKnownGoodAutoConfigUrl == other.pwszLastKnownGoodAutoConfigUrl
            && self.dwAutoconfigReloadDelayMins == other.dwAutoconfigReloadDelayMins
            && self.ftLastKnownDetectTime == other.ftLastKnownDetectTime
            && self.dwDetectedInterfaceIpCount == other.dwDetectedInterfaceIpCount
            && self.pdwDetectedInterfaceIp == other.pdwDetectedInterfaceIp
            && self.cNetworkKeys == other.cNetworkKeys
            && self.pNetworkKeys == other.pNetworkKeys
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINHTTP_PROXY_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINHTTP_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_PROXY_SETTINGS")
            .field("dwStructSize", &self.dwStructSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwCurrentSettingsVersion", &self.dwCurrentSettingsVersion)
            .field("pwszConnectionName", &self.pwszConnectionName)
            .field("pwszProxy", &self.pwszProxy)
            .field("pwszProxyBypass", &self.pwszProxyBypass)
            .field("pwszAutoconfigUrl", &self.pwszAutoconfigUrl)
            .field("pwszAutoconfigSecondaryUrl", &self.pwszAutoconfigSecondaryUrl)
            .field("dwAutoDiscoveryFlags", &self.dwAutoDiscoveryFlags)
            .field("pwszLastKnownGoodAutoConfigUrl", &self.pwszLastKnownGoodAutoConfigUrl)
            .field("dwAutoconfigReloadDelayMins", &self.dwAutoconfigReloadDelayMins)
            .field("ftLastKnownDetectTime", &self.ftLastKnownDetectTime)
            .field("dwDetectedInterfaceIpCount", &self.dwDetectedInterfaceIpCount)
            .field("pdwDetectedInterfaceIp", &self.pdwDetectedInterfaceIp)
            .field("cNetworkKeys", &self.cNetworkKeys)
            .field("pNetworkKeys", &self.pNetworkKeys)
            .finish()
    }
}
impl ::core::default::Default for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.cHosts == other.cHosts && self.pHostConnectionGroups == other.pHostConnectionGroups
    }
}
impl ::core::cmp::Eq for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {}
impl ::core::fmt::Debug for WINHTTP_QUERY_CONNECTION_GROUP_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_QUERY_CONNECTION_GROUP_RESULT").field("cHosts", &self.cHosts).field("pHostConnectionGroups", &self.pHostConnectionGroups).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_REQUEST_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_REQUEST_STAT_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_REQUEST_STAT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_REQUEST_STAT_ENTRY").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_REQUEST_TIME_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_REQUEST_TIME_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_REQUEST_TIME_ENTRY").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for WINHTTP_RESOLVER_CACHE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINHTTP_SECURE_DNS_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_SECURE_DNS_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_SECURE_DNS_SETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.AsyncResult == other.AsyncResult && self.Operation == other.Operation
    }
}
impl ::core::cmp::Eq for WINHTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_WEB_SOCKET_ASYNC_RESULT").field("AsyncResult", &self.AsyncResult).field("Operation", &self.Operation).finish()
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINHTTP_WEB_SOCKET_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINHTTP_WEB_SOCKET_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINHTTP_WEB_SOCKET_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwBytesTransferred == other.dwBytesTransferred && self.eBufferType == other.eBufferType
    }
}
impl ::core::cmp::Eq for WINHTTP_WEB_SOCKET_STATUS {}
impl ::core::fmt::Debug for WINHTTP_WEB_SOCKET_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINHTTP_WEB_SOCKET_STATUS").field("dwBytesTransferred", &self.dwBytesTransferred).field("eBufferType", &self.eBufferType).finish()
    }
}
impl ::core::default::Default for WIN_HTTP_CREATE_URL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN_HTTP_CREATE_URL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN_HTTP_CREATE_URL_FLAGS").field(&self.0).finish()
    }
}
