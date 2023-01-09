impl ::core::cmp::PartialEq for AsyncIFtpAuthenticationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpAuthenticationProvider {}
impl ::core::fmt::Debug for AsyncIFtpAuthenticationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpAuthenticationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpAuthorizationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpAuthorizationProvider {}
impl ::core::fmt::Debug for AsyncIFtpAuthorizationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpAuthorizationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpHomeDirectoryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpHomeDirectoryProvider {}
impl ::core::fmt::Debug for AsyncIFtpHomeDirectoryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpHomeDirectoryProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpLogProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpLogProvider {}
impl ::core::fmt::Debug for AsyncIFtpLogProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpLogProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpPostprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpPostprocessProvider {}
impl ::core::fmt::Debug for AsyncIFtpPostprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpPostprocessProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpPreprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpPreprocessProvider {}
impl ::core::fmt::Debug for AsyncIFtpPreprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpPreprocessProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpRoleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpRoleProvider {}
impl ::core::fmt::Debug for AsyncIFtpRoleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpRoleProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AsyncIMSAdminBaseSinkW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMSAdminBaseSinkW {}
impl ::core::fmt::Debug for AsyncIMSAdminBaseSinkW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMSAdminBaseSinkW").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CERT_CONTEXT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CERT_CONTEXT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.CertContext == other.CertContext && self.cbAllocated == other.cbAllocated && self.dwCertificateFlags == other.dwCertificateFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CERT_CONTEXT_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CERT_CONTEXT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CONTEXT_EX").field("CertContext", &self.CertContext).field("cbAllocated", &self.cbAllocated).field("dwCertificateFlags", &self.dwCertificateFlags).finish()
    }
}
impl ::core::default::Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIGURATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.bstrKey == other.bstrKey && self.bstrValue == other.bstrValue
    }
}
impl ::core::cmp::Eq for CONFIGURATION_ENTRY {}
impl ::core::fmt::Debug for CONFIGURATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIGURATION_ENTRY").field("bstrKey", &self.bstrKey).field("bstrValue", &self.bstrValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTENSION_CONTROL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVersion == other.dwVersion && self.ConnID == other.ConnID && self.dwHttpStatusCode == other.dwHttpStatusCode && self.lpszLogData == other.lpszLogData && self.lpszMethod == other.lpszMethod && self.lpszQueryString == other.lpszQueryString && self.lpszPathInfo == other.lpszPathInfo && self.lpszPathTranslated == other.lpszPathTranslated && self.cbTotalBytes == other.cbTotalBytes && self.cbAvailable == other.cbAvailable && self.lpbData == other.lpbData && self.lpszContentType == other.lpszContentType && self.GetServerVariable == other.GetServerVariable && self.WriteClient == other.WriteClient && self.ReadClient == other.ReadClient && self.ServerSupportFunction == other.ServerSupportFunction
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTENSION_CONTROL_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTENSION_CONTROL_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENSION_CONTROL_BLOCK")
            .field("cbSize", &self.cbSize)
            .field("dwVersion", &self.dwVersion)
            .field("ConnID", &self.ConnID)
            .field("dwHttpStatusCode", &self.dwHttpStatusCode)
            .field("lpszLogData", &self.lpszLogData)
            .field("lpszMethod", &self.lpszMethod)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("lpszPathInfo", &self.lpszPathInfo)
            .field("lpszPathTranslated", &self.lpszPathTranslated)
            .field("cbTotalBytes", &self.cbTotalBytes)
            .field("cbAvailable", &self.cbAvailable)
            .field("lpbData", &self.lpbData)
            .field("lpszContentType", &self.lpszContentType)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("WriteClient", &self.WriteClient)
            .field("ReadClient", &self.ReadClient)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
impl ::core::default::Default for FTP_ACCESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FTP_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_ACCESS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FTP_PROCESS_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FTP_PROCESS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_PROCESS_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_CUSTOM_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus && self.uHttpSubError == other.uHttpSubError && self.fAsync == other.fAsync
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_CUSTOM_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_CUSTOM_ERROR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_CUSTOM_ERROR_INFO").field("pszStatus", &self.pszStatus).field("uHttpSubError", &self.uHttpSubError).field("fAsync", &self.fAsync).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_UNICODE_URL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl && self.pszMethod == other.pszMethod && self.pszChildHeaders == other.pszChildHeaders && self.pUserInfo == other.pUserInfo && self.pEntity == other.pEntity && self.dwExecUrlFlags == other.dwExecUrlFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_UNICODE_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_UNICODE_URL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_UNICODE_URL_INFO").field("pszUrl", &self.pszUrl).field("pszMethod", &self.pszMethod).field("pszChildHeaders", &self.pszChildHeaders).field("pUserInfo", &self.pUserInfo).field("pEntity", &self.pEntity).field("dwExecUrlFlags", &self.dwExecUrlFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken && self.pszCustomUserName == other.pszCustomUserName && self.pszCustomAuthType == other.pszCustomAuthType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_UNICODE_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_UNICODE_URL_USER_INFO").field("hImpersonationToken", &self.hImpersonationToken).field("pszCustomUserName", &self.pszCustomUserName).field("pszCustomAuthType", &self.pszCustomAuthType).finish()
    }
}
impl ::core::default::Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_ENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbAvailable == other.cbAvailable && self.lpbData == other.lpbData
    }
}
impl ::core::cmp::Eq for HSE_EXEC_URL_ENTITY_INFO {}
impl ::core::fmt::Debug for HSE_EXEC_URL_ENTITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_ENTITY_INFO").field("cbAvailable", &self.cbAvailable).field("lpbData", &self.lpbData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_URL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl && self.pszMethod == other.pszMethod && self.pszChildHeaders == other.pszChildHeaders && self.pUserInfo == other.pUserInfo && self.pEntity == other.pEntity && self.dwExecUrlFlags == other.dwExecUrlFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_URL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_INFO").field("pszUrl", &self.pszUrl).field("pszMethod", &self.pszMethod).field("pszChildHeaders", &self.pszChildHeaders).field("pUserInfo", &self.pUserInfo).field("pEntity", &self.pEntity).field("dwExecUrlFlags", &self.dwExecUrlFlags).finish()
    }
}
impl ::core::default::Default for HSE_EXEC_URL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uHttpStatusCode == other.uHttpStatusCode && self.uHttpSubStatus == other.uHttpSubStatus && self.dwWin32Error == other.dwWin32Error
    }
}
impl ::core::cmp::Eq for HSE_EXEC_URL_STATUS {}
impl ::core::fmt::Debug for HSE_EXEC_URL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_STATUS").field("uHttpStatusCode", &self.uHttpStatusCode).field("uHttpSubStatus", &self.uHttpSubStatus).field("dwWin32Error", &self.dwWin32Error).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken && self.pszCustomUserName == other.pszCustomUserName && self.pszCustomAuthType == other.pszCustomAuthType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_URL_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_USER_INFO").field("hImpersonationToken", &self.hImpersonationToken).field("pszCustomUserName", &self.pszCustomUserName).field("pszCustomAuthType", &self.pszCustomAuthType).finish()
    }
}
impl ::core::default::Default for HSE_RESPONSE_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSE_RESPONSE_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pszStatus == other.pszStatus && self.pszHeaders == other.pszHeaders && self.nElementCount == other.nElementCount && self.lpElementArray == other.lpElementArray
    }
}
impl ::core::cmp::Eq for HSE_RESPONSE_VECTOR {}
impl ::core::fmt::Debug for HSE_RESPONSE_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_RESPONSE_VECTOR").field("dwFlags", &self.dwFlags).field("pszStatus", &self.pszStatus).field("pszHeaders", &self.pszHeaders).field("nElementCount", &self.nElementCount).field("lpElementArray", &self.lpElementArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_SEND_HEADER_EX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_SEND_HEADER_EX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus && self.pszHeader == other.pszHeader && self.cchStatus == other.cchStatus && self.cchHeader == other.cchHeader && self.fKeepConn == other.fKeepConn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_SEND_HEADER_EX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_SEND_HEADER_EX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_SEND_HEADER_EX_INFO").field("pszStatus", &self.pszStatus).field("pszHeader", &self.pszHeader).field("cchStatus", &self.cchStatus).field("cchHeader", &self.cchHeader).field("fKeepConn", &self.fKeepConn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_TRACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fTraceRequest == other.fTraceRequest && self.TraceContextId == other.TraceContextId && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_TRACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_TRACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_TRACE_INFO").field("fTraceRequest", &self.fTraceRequest).field("TraceContextId", &self.TraceContextId).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::default::Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSE_UNICODE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL
    }
}
impl ::core::cmp::Eq for HSE_UNICODE_URL_MAPEX_INFO {}
impl ::core::fmt::Debug for HSE_UNICODE_URL_MAPEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_UNICODE_URL_MAPEX_INFO").field("lpszPath", &self.lpszPath).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_URL_MAPEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_URL_MAPEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_URL_MAPEX_INFO").field("lpszPath", &self.lpszPath).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::default::Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSE_VECTOR_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.pvContext == other.pvContext && self.cbOffset == other.cbOffset && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for HSE_VECTOR_ELEMENT {}
impl ::core::fmt::Debug for HSE_VECTOR_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_VECTOR_ELEMENT").field("ElementType", &self.ElementType).field("pvContext", &self.pvContext).field("cbOffset", &self.cbOffset).field("cbSize", &self.cbSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExtensionVersion == other.dwExtensionVersion && self.lpszExtensionDesc == other.lpszExtensionDesc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_VERSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_VERSION_INFO").field("dwExtensionVersion", &self.dwExtensionVersion).field("lpszExtensionDesc", &self.lpszExtensionDesc).finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_ACCESS_DENIED {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.dwReason == other.dwReason
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_ACCESS_DENIED {}
impl ::core::fmt::Debug for HTTP_FILTER_ACCESS_DENIED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_ACCESS_DENIED").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("dwReason", &self.dwReason).finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTHENT {
    fn eq(&self, other: &Self) -> bool {
        self.pszUser == other.pszUser && self.cbUserBuff == other.cbUserBuff && self.pszPassword == other.pszPassword && self.cbPasswordBuff == other.cbPasswordBuff
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_AUTHENT {}
impl ::core::fmt::Debug for HTTP_FILTER_AUTHENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_AUTHENT").field("pszUser", &self.pszUser).field("cbUserBuff", &self.cbUserBuff).field("pszPassword", &self.pszPassword).field("cbPasswordBuff", &self.cbPasswordBuff).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader && self.SetHeader == other.SetHeader && self.AddHeader == other.AddHeader && self.GetUserToken == other.GetUserToken && self.HttpStatus == other.HttpStatus && self.fResetAuth == other.fResetAuth && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_AUTH_COMPLETE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_AUTH_COMPLETE_INFO").field("GetHeader", &self.GetHeader).field("SetHeader", &self.SetHeader).field("AddHeader", &self.AddHeader).field("GetUserToken", &self.GetUserToken).field("HttpStatus", &self.HttpStatus).field("fResetAuth", &self.fResetAuth).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Revision == other.Revision && self.ServerContext == other.ServerContext && self.ulReserved == other.ulReserved && self.fIsSecurePort == other.fIsSecurePort && self.pFilterContext == other.pFilterContext && self.GetServerVariable == other.GetServerVariable && self.AddResponseHeaders == other.AddResponseHeaders && self.WriteClient == other.WriteClient && self.AllocMem == other.AllocMem && self.ServerSupportFunction == other.ServerSupportFunction
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_CONTEXT")
            .field("cbSize", &self.cbSize)
            .field("Revision", &self.Revision)
            .field("ServerContext", &self.ServerContext)
            .field("ulReserved", &self.ulReserved)
            .field("fIsSecurePort", &self.fIsSecurePort)
            .field("pFilterContext", &self.pFilterContext)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("AddResponseHeaders", &self.AddResponseHeaders)
            .field("WriteClient", &self.WriteClient)
            .field("AllocMem", &self.AllocMem)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_LOG {
    fn eq(&self, other: &Self) -> bool {
        self.pszClientHostName == other.pszClientHostName && self.pszClientUserName == other.pszClientUserName && self.pszServerName == other.pszServerName && self.pszOperation == other.pszOperation && self.pszTarget == other.pszTarget && self.pszParameters == other.pszParameters && self.dwHttpStatus == other.dwHttpStatus && self.dwWin32Status == other.dwWin32Status && self.dwBytesSent == other.dwBytesSent && self.dwBytesRecvd == other.dwBytesRecvd && self.msTimeForProcessing == other.msTimeForProcessing
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_LOG {}
impl ::core::fmt::Debug for HTTP_FILTER_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_LOG")
            .field("pszClientHostName", &self.pszClientHostName)
            .field("pszClientUserName", &self.pszClientUserName)
            .field("pszServerName", &self.pszServerName)
            .field("pszOperation", &self.pszOperation)
            .field("pszTarget", &self.pszTarget)
            .field("pszParameters", &self.pszParameters)
            .field("dwHttpStatus", &self.dwHttpStatus)
            .field("dwWin32Status", &self.dwWin32Status)
            .field("dwBytesSent", &self.dwBytesSent)
            .field("dwBytesRecvd", &self.dwBytesRecvd)
            .field("msTimeForProcessing", &self.msTimeForProcessing)
            .finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_PREPROC_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader && self.SetHeader == other.SetHeader && self.AddHeader == other.AddHeader && self.HttpStatus == other.HttpStatus && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_PREPROC_HEADERS {}
impl ::core::fmt::Debug for HTTP_FILTER_PREPROC_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_PREPROC_HEADERS").field("GetHeader", &self.GetHeader).field("SetHeader", &self.SetHeader).field("AddHeader", &self.AddHeader).field("HttpStatus", &self.HttpStatus).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pvInData == other.pvInData && self.cbInData == other.cbInData && self.cbInBuffer == other.cbInBuffer && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_RAW_DATA {}
impl ::core::fmt::Debug for HTTP_FILTER_RAW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_RAW_DATA").field("pvInData", &self.pvInData).field("cbInData", &self.cbInData).field("cbInBuffer", &self.cbInBuffer).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.cbPathBuff == other.cbPathBuff
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_URL_MAP {}
impl ::core::fmt::Debug for HTTP_FILTER_URL_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_URL_MAP").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("cbPathBuff", &self.cbPathBuff).finish()
    }
}
impl ::core::default::Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP_EX {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.cbPathBuff == other.cbPathBuff && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL && self.pszScriptMapEntry == other.pszScriptMapEntry
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_URL_MAP_EX {}
impl ::core::fmt::Debug for HTTP_FILTER_URL_MAP_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_URL_MAP_EX").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("cbPathBuff", &self.cbPathBuff).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).field("pszScriptMapEntry", &self.pszScriptMapEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwServerFilterVersion == other.dwServerFilterVersion && self.dwFilterVersion == other.dwFilterVersion && self.lpszFilterDesc == other.lpszFilterDesc && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_VERSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_VERSION").field("dwServerFilterVersion", &self.dwServerFilterVersion).field("dwFilterVersion", &self.dwFilterVersion).field("lpszFilterDesc", &self.lpszFilterDesc).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_TRACE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_TRACE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid && self.dwAreas == other.dwAreas && self.dwVerbosity == other.dwVerbosity && self.fProviderEnabled == other.fProviderEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_TRACE_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_TRACE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_CONFIGURATION").field("pProviderGuid", &self.pProviderGuid).field("dwAreas", &self.dwAreas).field("dwVerbosity", &self.dwVerbosity).field("fProviderEnabled", &self.fProviderEnabled).finish()
    }
}
impl ::core::default::Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid && self.dwArea == other.dwArea && self.pAreaGuid == other.pAreaGuid && self.dwEvent == other.dwEvent && self.pszEventName == other.pszEventName && self.dwEventVersion == other.dwEventVersion && self.dwVerbosity == other.dwVerbosity && self.pActivityGuid == other.pActivityGuid && self.pRelatedActivityGuid == other.pRelatedActivityGuid && self.dwTimeStamp == other.dwTimeStamp && self.dwFlags == other.dwFlags && self.cEventItems == other.cEventItems && self.pEventItems == other.pEventItems
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT {}
impl ::core::fmt::Debug for HTTP_TRACE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_EVENT")
            .field("pProviderGuid", &self.pProviderGuid)
            .field("dwArea", &self.dwArea)
            .field("pAreaGuid", &self.pAreaGuid)
            .field("dwEvent", &self.dwEvent)
            .field("pszEventName", &self.pszEventName)
            .field("dwEventVersion", &self.dwEventVersion)
            .field("dwVerbosity", &self.dwVerbosity)
            .field("pActivityGuid", &self.pActivityGuid)
            .field("pRelatedActivityGuid", &self.pRelatedActivityGuid)
            .field("dwTimeStamp", &self.dwTimeStamp)
            .field("dwFlags", &self.dwFlags)
            .field("cEventItems", &self.cEventItems)
            .field("pEventItems", &self.pEventItems)
            .finish()
    }
}
impl ::core::default::Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.dwDataType == other.dwDataType && self.pbData == other.pbData && self.cbData == other.cbData && self.pszDataDescription == other.pszDataDescription
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT_ITEM {}
impl ::core::fmt::Debug for HTTP_TRACE_EVENT_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_EVENT_ITEM").field("pszName", &self.pszName).field("dwDataType", &self.dwDataType).field("pbData", &self.pbData).field("cbData", &self.cbData).field("pszDataDescription", &self.pszDataDescription).finish()
    }
}
impl ::core::default::Default for HTTP_TRACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_TRACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_TRACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IADMEXT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADMEXT {}
impl ::core::fmt::Debug for IADMEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADMEXT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpAuthenticationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpAuthenticationProvider {}
impl ::core::fmt::Debug for IFtpAuthenticationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpAuthenticationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpAuthorizationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpAuthorizationProvider {}
impl ::core::fmt::Debug for IFtpAuthorizationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpAuthorizationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpHomeDirectoryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpHomeDirectoryProvider {}
impl ::core::fmt::Debug for IFtpHomeDirectoryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpHomeDirectoryProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpLogProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpLogProvider {}
impl ::core::fmt::Debug for IFtpLogProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpLogProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpPostprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpPostprocessProvider {}
impl ::core::fmt::Debug for IFtpPostprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpPostprocessProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpPreprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpPreprocessProvider {}
impl ::core::fmt::Debug for IFtpPreprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpPreprocessProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpProviderConstruct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpProviderConstruct {}
impl ::core::fmt::Debug for IFtpProviderConstruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpProviderConstruct").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFtpRoleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpRoleProvider {}
impl ::core::fmt::Debug for IFtpRoleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpRoleProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMSAdminBase2W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBase2W {}
impl ::core::fmt::Debug for IMSAdminBase2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBase2W").field(&self.0).finish()
    }
}
impl IMSAdminBase2W {
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteChildKeys)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumKeys)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), ::core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyKey)(::windows::core::Vtable::as_raw(self), hmdsourcehandle, pszmdsourcepath.into().abi(), hmddesthandle, pszmddestpath.into().abi(), bmdoverwriteflag.into(), bmdcopyflag.into()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RenameKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pszmdnewname.into().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAllData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdusertype, dwmddatatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyData)(::windows::core::Vtable::as_raw(self), hmdsourcehandle, pszmdsourcepath.into().abi(), hmddesthandle, pszmddestpath.into().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.into()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDataPaths)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len() as _, ::core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdaccessrequested, dwmdtimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CloseKey)(::windows::core::Vtable::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangePermissions)(::windows::core::Vtable::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveData)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHandleInfo)(::windows::core::Vtable::as_raw(self), hmdhandle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemChangeNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDataSetNumber)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLastChangeTime)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pftmdlastchangetime, blocaltime.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetLastChangeTime)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pftmdlastchangetime, blocaltime.into()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.KeyExchangePhase1)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.KeyExchangePhase2)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Backup)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Restore)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumBackups)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteBackup)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnmarshalInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetServerGuid)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMSAdminBase3W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBase3W {}
impl ::core::fmt::Debug for IMSAdminBase3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBase3W").field(&self.0).finish()
    }
}
impl IMSAdminBase3W {
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteChildKeys)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumKeys)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), ::core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyKey)(::windows::core::Vtable::as_raw(self), hmdsourcehandle, pszmdsourcepath.into().abi(), hmddesthandle, pszmddestpath.into().abi(), bmdoverwriteflag.into(), bmdcopyflag.into()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RenameKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pszmdnewname.into().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteAllData)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdusertype, dwmddatatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyData)(::windows::core::Vtable::as_raw(self), hmdsourcehandle, pszmdsourcepath.into().abi(), hmddesthandle, pszmddestpath.into().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.into()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataPaths)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len() as _, ::core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenKey)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), dwmdaccessrequested, dwmdtimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CloseKey)(::windows::core::Vtable::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ChangePermissions)(::windows::core::Vtable::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveData)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetHandleInfo)(::windows::core::Vtable::as_raw(self), hmdhandle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemChangeNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataSetNumber)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLastChangeTime)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pftmdlastchangetime, blocaltime.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastChangeTime)(::windows::core::Vtable::as_raw(self), hmdhandle, pszmdpath.into().abi(), pftmdlastchangetime, blocaltime.into()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyExchangePhase1)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.KeyExchangePhase2)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Backup)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Restore)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumBackups)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteBackup)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UnmarshalInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetServerGuid)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BackupWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BackupWithPasswd)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags, pszpasswd.into().abi()).ok()
    }
    pub unsafe fn RestoreWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreWithPasswd)(::windows::core::Vtable::as_raw(self), pszmdbackuplocation.into().abi(), dwmdversion, dwmdflags, pszpasswd.into().abi()).ok()
    }
    pub unsafe fn Export<P0, P1, P2>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Export)(::windows::core::Vtable::as_raw(self), pszpasswd.into().abi(), pszfilename.into().abi(), pszsourcepath.into().abi(), dwmdflags).ok()
    }
    pub unsafe fn Import<P0, P1, P2, P3>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, pszdestpath: P3, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Import)(::windows::core::Vtable::as_raw(self), pszpasswd.into().abi(), pszfilename.into().abi(), pszsourcepath.into().abi(), pszdestpath.into().abi(), dwmdflags).ok()
    }
    pub unsafe fn RestoreHistory<P0>(&self, pszmdhistorylocation: P0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreHistory)(::windows::core::Vtable::as_raw(self), pszmdhistorylocation.into().abi(), dwmdmajorversion, dwmdminorversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumHistory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszmdhistorylocation.as_ptr()), pdwmdmajorversion, pdwmdminorversion, pftmdhistorytime, dwmdenumindex).ok()
    }
}
impl ::core::cmp::PartialEq for IMSAdminBaseSinkW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBaseSinkW {}
impl ::core::fmt::Debug for IMSAdminBaseSinkW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBaseSinkW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMSAdminBaseW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBaseW {}
impl ::core::fmt::Debug for IMSAdminBaseW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBaseW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMSImpExpHelpW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSImpExpHelpW {}
impl ::core::fmt::Debug for IMSImpExpHelpW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSImpExpHelpW").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOGGING_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.dwElapsedMilliseconds == other.dwElapsedMilliseconds
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.pszInformation == other.pszInformation
    }
}
impl ::core::cmp::Eq for LOGGING_PARAMETERS {}
impl ::core::fmt::Debug for LOGGING_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGGING_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("dwElapsedMilliseconds", &self.dwElapsedMilliseconds)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("pszInformation", &self.pszInformation)
            .finish()
    }
}
impl ::core::default::Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MD_CHANGE_OBJECT_W {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDPath == other.pszMDPath && self.dwMDChangeType == other.dwMDChangeType && self.dwMDNumDataIDs == other.dwMDNumDataIDs && self.pdwMDDataIDs == other.pdwMDDataIDs
    }
}
impl ::core::cmp::Eq for MD_CHANGE_OBJECT_W {}
impl ::core::fmt::Debug for MD_CHANGE_OBJECT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MD_CHANGE_OBJECT_W").field("pszMDPath", &self.pszMDPath).field("dwMDChangeType", &self.dwMDChangeType).field("dwMDNumDataIDs", &self.dwMDNumDataIDs).field("pdwMDDataIDs", &self.pdwMDDataIDs).finish()
    }
}
impl ::core::default::Default for METADATATYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for METADATATYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("METADATATYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for METADATA_GETALL_INTERNAL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for METADATA_GETALL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for METADATA_GETALL_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier && self.dwMDAttributes == other.dwMDAttributes && self.dwMDUserType == other.dwMDUserType && self.dwMDDataType == other.dwMDDataType && self.dwMDDataLen == other.dwMDDataLen && self.dwMDDataOffset == other.dwMDDataOffset && self.dwMDDataTag == other.dwMDDataTag
    }
}
impl ::core::cmp::Eq for METADATA_GETALL_RECORD {}
impl ::core::fmt::Debug for METADATA_GETALL_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_GETALL_RECORD").field("dwMDIdentifier", &self.dwMDIdentifier).field("dwMDAttributes", &self.dwMDAttributes).field("dwMDUserType", &self.dwMDUserType).field("dwMDDataType", &self.dwMDDataType).field("dwMDDataLen", &self.dwMDDataLen).field("dwMDDataOffset", &self.dwMDDataOffset).field("dwMDDataTag", &self.dwMDDataTag).finish()
    }
}
impl ::core::default::Default for METADATA_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for METADATA_HANDLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDPermissions == other.dwMDPermissions && self.dwMDSystemChangeNumber == other.dwMDSystemChangeNumber
    }
}
impl ::core::cmp::Eq for METADATA_HANDLE_INFO {}
impl ::core::fmt::Debug for METADATA_HANDLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_HANDLE_INFO").field("dwMDPermissions", &self.dwMDPermissions).field("dwMDSystemChangeNumber", &self.dwMDSystemChangeNumber).finish()
    }
}
impl ::core::default::Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for METADATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier && self.dwMDAttributes == other.dwMDAttributes && self.dwMDUserType == other.dwMDUserType && self.dwMDDataType == other.dwMDDataType && self.dwMDDataLen == other.dwMDDataLen && self.pbMDData == other.pbMDData && self.dwMDDataTag == other.dwMDDataTag
    }
}
impl ::core::cmp::Eq for METADATA_RECORD {}
impl ::core::fmt::Debug for METADATA_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_RECORD").field("dwMDIdentifier", &self.dwMDIdentifier).field("dwMDAttributes", &self.dwMDAttributes).field("dwMDUserType", &self.dwMDUserType).field("dwMDDataType", &self.dwMDDataType).field("dwMDDataLen", &self.dwMDDataLen).field("pbMDData", &self.pbMDData).field("dwMDDataTag", &self.dwMDDataTag).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POST_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POST_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.SessionStartTime == other.SessionStartTime
            && self.BytesSentPerSession == other.BytesSentPerSession
            && self.BytesReceivedPerSession == other.BytesReceivedPerSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POST_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POST_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POST_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRE_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRE_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId && self.pszSiteName == other.pszSiteName && self.pszUserName == other.pszUserName && self.pszHostName == other.pszHostName && self.pszRemoteIpAddress == other.pszRemoteIpAddress && self.dwRemoteIpPort == other.dwRemoteIpPort && self.pszLocalIpAddress == other.pszLocalIpAddress && self.dwLocalIpPort == other.dwLocalIpPort && self.pszCommand == other.pszCommand && self.pszCommandParameters == other.pszCommandParameters && self.SessionStartTime == other.SessionStartTime && self.BytesSentPerSession == other.BytesSentPerSession && self.BytesReceivedPerSession == other.BytesReceivedPerSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRE_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRE_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRE_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
impl ::core::default::Default for SF_PROPERTY_IIS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SF_PROPERTY_IIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_PROPERTY_IIS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SF_REQ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SF_REQ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_REQ_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SF_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SF_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_STATUS_TYPE").field(&self.0).finish()
    }
}
