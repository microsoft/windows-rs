impl ::core::default::Default for APP_CACHE_DOWNLOAD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUrl == other.pwszUrl && self.dwEntryType == other.dwEntryType
    }
}
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_ENTRY {}
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_DOWNLOAD_ENTRY").field("pwszUrl", &self.pwszUrl).field("dwEntryType", &self.dwEntryType).finish()
    }
}
impl ::core::default::Default for APP_CACHE_DOWNLOAD_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwEntryCount == other.dwEntryCount && self.pEntries == other.pEntries
    }
}
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_LIST {}
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_DOWNLOAD_LIST").field("dwEntryCount", &self.dwEntryCount).field("pEntries", &self.pEntries).finish()
    }
}
impl ::core::default::Default for APP_CACHE_FINALIZE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APP_CACHE_FINALIZE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APP_CACHE_FINALIZE_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszManifestUrl == other.pwszManifestUrl && self.ftLastAccessTime == other.ftLastAccessTime && self.ullSize == other.ullSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_GROUP_INFO").field("pwszManifestUrl", &self.pwszManifestUrl).field("ftLastAccessTime", &self.ftLastAccessTime).field("ullSize", &self.ullSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppCacheGroupCount == other.dwAppCacheGroupCount && self.pAppCacheGroups == other.pAppCacheGroups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_CACHE_GROUP_LIST").field("dwAppCacheGroupCount", &self.dwAppCacheGroupCount).field("pAppCacheGroups", &self.pAppCacheGroups).finish()
    }
}
impl ::core::default::Default for APP_CACHE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APP_CACHE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APP_CACHE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUTO_PROXY_SCRIPT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUTO_PROXY_SCRIPT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScriptBuffer == other.lpszScriptBuffer && self.dwScriptBufferSize == other.dwScriptBufferSize
    }
}
impl ::core::cmp::Eq for AUTO_PROXY_SCRIPT_BUFFER {}
impl ::core::fmt::Debug for AUTO_PROXY_SCRIPT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTO_PROXY_SCRIPT_BUFFER").field("dwStructSize", &self.dwStructSize).field("lpszScriptBuffer", &self.lpszScriptBuffer).field("dwScriptBufferSize", &self.dwScriptBufferSize).finish()
    }
}
impl ::core::default::Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AutoProxyHelperFunctions {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl
    }
}
impl ::core::cmp::Eq for AutoProxyHelperFunctions {}
impl ::core::fmt::Debug for AutoProxyHelperFunctions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutoProxyHelperFunctions").field("lpVtbl", &self.lpVtbl).finish()
    }
}
impl ::core::default::Default for AutoProxyHelperVtbl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AutoProxyHelperVtbl {
    fn eq(&self, other: &Self) -> bool {
        self.IsResolvable == other.IsResolvable && self.GetIPAddress == other.GetIPAddress && self.ResolveHostName == other.ResolveHostName && self.IsInNet == other.IsInNet && self.IsResolvableEx == other.IsResolvableEx && self.GetIPAddressEx == other.GetIPAddressEx && self.ResolveHostNameEx == other.ResolveHostNameEx && self.IsInNetEx == other.IsInNetEx && self.SortIpList == other.SortIpList
    }
}
impl ::core::cmp::Eq for AutoProxyHelperVtbl {}
impl ::core::fmt::Debug for AutoProxyHelperVtbl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AutoProxyHelperVtbl").field("IsResolvable", &self.IsResolvable).field("GetIPAddress", &self.GetIPAddress).field("ResolveHostName", &self.ResolveHostName).field("IsInNet", &self.IsInNet).field("IsResolvableEx", &self.IsResolvableEx).field("GetIPAddressEx", &self.GetIPAddressEx).field("ResolveHostNameEx", &self.ResolveHostNameEx).field("IsInNetEx", &self.IsInNetEx).field("SortIpList", &self.SortIpList).finish()
    }
}
impl ::core::default::Default for CACHE_CONFIG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CACHE_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CACHE_CONFIG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COOKIE_DLG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COOKIE_DLG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszServer == other.pszServer && self.pic == other.pic && self.dwStopWarning == other.dwStopWarning && self.cx == other.cx && self.cy == other.cy && self.pszHeader == other.pszHeader && self.dwOperation == other.dwOperation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COOKIE_DLG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COOKIE_DLG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COOKIE_DLG_INFO").field("pszServer", &self.pszServer).field("pic", &self.pic).field("dwStopWarning", &self.dwStopWarning).field("cx", &self.cx).field("cy", &self.cy).field("pszHeader", &self.pszHeader).field("dwOperation", &self.dwOperation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CookieDecision {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CookieDecision {
    fn eq(&self, other: &Self) -> bool {
        self.dwCookieState == other.dwCookieState && self.fAllowSession == other.fAllowSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CookieDecision {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CookieDecision {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CookieDecision").field("dwCookieState", &self.dwCookieState).field("fAllowSession", &self.fAllowSession).finish()
    }
}
impl ::core::default::Default for FORTCMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FORTCMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORTCMD").field(&self.0).finish()
    }
}
impl ::core::default::Default for FORTSTAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FORTSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORTSTAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for FTP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FTP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ShortAbstract == other.ShortAbstract && self.AbstractFile == other.AbstractFile
    }
}
impl ::core::cmp::Eq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ABSTRACT_ATTRIBUTE_TYPE").field("ShortAbstract", &self.ShortAbstract).field("AbstractFile", &self.AbstractFile).finish()
    }
}
impl ::core::default::Default for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment && self.EmailAddress == other.EmailAddress
    }
}
impl ::core::cmp::Eq for GOPHER_ADMIN_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ADMIN_ATTRIBUTE_TYPE").field("Comment", &self.Comment).field("EmailAddress", &self.EmailAddress).finish()
    }
}
impl ::core::default::Default for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.QuestionType == other.QuestionType && self.QuestionText == other.QuestionText
    }
}
impl ::core::cmp::Eq for GOPHER_ASK_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ASK_ATTRIBUTE_TYPE").field("QuestionType", &self.QuestionType).field("QuestionText", &self.QuestionText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_FIND_DATAA").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_FIND_DATAW").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
impl ::core::default::Default for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DegreesNorth == other.DegreesNorth && self.MinutesNorth == other.MinutesNorth && self.SecondsNorth == other.SecondsNorth && self.DegreesEast == other.DegreesEast && self.MinutesEast == other.MinutesEast && self.SecondsEast == other.SecondsEast
    }
}
impl ::core::cmp::Eq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE").field("DegreesNorth", &self.DegreesNorth).field("MinutesNorth", &self.MinutesNorth).field("SecondsNorth", &self.SecondsNorth).field("DegreesEast", &self.DegreesEast).field("MinutesEast", &self.MinutesEast).field("SecondsEast", &self.SecondsEast).finish()
    }
}
impl ::core::default::Default for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
    }
}
impl ::core::cmp::Eq for GOPHER_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_LOCATION_ATTRIBUTE_TYPE").field("Location", &self.Location).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DateAndTime == other.DateAndTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_MOD_DATE_ATTRIBUTE_TYPE").field("DateAndTime", &self.DateAndTime).finish()
    }
}
impl ::core::default::Default for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Organization == other.Organization
    }
}
impl ::core::cmp::Eq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_ORGANIZATION_ATTRIBUTE_TYPE").field("Organization", &self.Organization).finish()
    }
}
impl ::core::default::Default for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Provider == other.Provider
    }
}
impl ::core::cmp::Eq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_PROVIDER_ATTRIBUTE_TYPE").field("Provider", &self.Provider).finish()
    }
}
impl ::core::default::Default for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SCORE_ATTRIBUTE_TYPE").field("Score", &self.Score).finish()
    }
}
impl ::core::default::Default for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.LowerBound == other.LowerBound && self.UpperBound == other.UpperBound
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE").field("LowerBound", &self.LowerBound).field("UpperBound", &self.UpperBound).finish()
    }
}
impl ::core::default::Default for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Site == other.Site
    }
}
impl ::core::cmp::Eq for GOPHER_SITE_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_SITE_ATTRIBUTE_TYPE").field("Site", &self.Site).finish()
    }
}
impl ::core::default::Default for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Zone == other.Zone
    }
}
impl ::core::cmp::Eq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_TIMEZONE_ATTRIBUTE_TYPE").field("Zone", &self.Zone).finish()
    }
}
impl ::core::default::Default for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Ttl == other.Ttl
    }
}
impl ::core::cmp::Eq for GOPHER_TTL_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_TTL_ATTRIBUTE_TYPE").field("Ttl", &self.Ttl).finish()
    }
}
impl ::core::default::Default for GOPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GOPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GOPHER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
impl ::core::cmp::Eq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_UNKNOWN_ATTRIBUTE_TYPE").field("Text", &self.Text).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TreeWalk == other.TreeWalk
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_VERONICA_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VERONICA_ATTRIBUTE_TYPE").field("TreeWalk", &self.TreeWalk).finish()
    }
}
impl ::core::default::Default for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
    }
}
impl ::core::cmp::Eq for GOPHER_VERSION_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VERSION_ATTRIBUTE_TYPE").field("Version", &self.Version).finish()
    }
}
impl ::core::default::Default for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ContentType == other.ContentType && self.Language == other.Language && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for GOPHER_VIEW_ATTRIBUTE_TYPE {}
impl ::core::fmt::Debug for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOPHER_VIEW_ATTRIBUTE_TYPE").field("ContentType", &self.ContentType).field("Language", &self.Language).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for HTTP_ADDREQ_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_ADDREQ_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_ADDREQ_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HTTP_ADDREQ_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HTTP_ADDREQ_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HTTP_POLICY_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_POLICY_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_POLICY_EXTENSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_POLICY_EXTENSION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_POLICY_EXTENSION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_POLICY_EXTENSION_VERSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_PUSH_NOTIFICATION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_PUSH_NOTIFICATION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatusValid == other.ChannelStatusValid && self.ChannelStatus == other.ChannelStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_PUSH_NOTIFICATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_PUSH_NOTIFICATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PUSH_NOTIFICATION_STATUS").field("ChannelStatusValid", &self.ChannelStatusValid).field("ChannelStatus", &self.ChannelStatus).finish()
    }
}
impl ::core::default::Default for HTTP_PUSH_TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_PUSH_TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventId == other.BrokerEventId
    }
}
impl ::core::cmp::Eq for HTTP_PUSH_TRANSPORT_SETTING {}
impl ::core::fmt::Debug for HTTP_PUSH_TRANSPORT_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_PUSH_TRANSPORT_SETTING").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventId", &self.BrokerEventId).finish()
    }
}
impl ::core::default::Default for HTTP_PUSH_WAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_PUSH_WAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_PUSH_WAIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TIMES {
    fn eq(&self, other: &Self) -> bool {
        self.cTimes == other.cTimes && self.rgTimes == other.rgTimes
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TIMES {}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_REQUEST_TIMES").field("cTimes", &self.cTimes).field("rgTimes", &self.rgTimes).finish()
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.AsyncResult == other.AsyncResult && self.Operation == other.Operation && self.BufferType == other.BufferType && self.dwBytesTransferred == other.dwBytesTransferred
    }
}
impl ::core::cmp::Eq for HTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_WEB_SOCKET_ASYNC_RESULT").field("AsyncResult", &self.AsyncResult).field("Operation", &self.Operation).field("BufferType", &self.BufferType).field("dwBytesTransferred", &self.dwBytesTransferred).finish()
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTP_WEB_SOCKET_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_WEB_SOCKET_OPERATION").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDialBranding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialBranding {}
impl ::core::fmt::Debug for IDialBranding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialBranding").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDialEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialEngine {}
impl ::core::fmt::Debug for IDialEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDialEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialEventSink {}
impl ::core::fmt::Debug for IDialEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialEventSink").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERNET_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERNET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwResult == other.dwResult && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for INTERNET_ASYNC_RESULT {}
impl ::core::fmt::Debug for INTERNET_ASYNC_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_ASYNC_RESULT").field("dwResult", &self.dwResult).field("dwError", &self.dwError).finish()
    }
}
impl ::core::default::Default for INTERNET_AUTH_NOTIFY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INTERNET_AUTODIAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_AUTODIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_AUTODIAL").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_BUFFERSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
impl ::core::cmp::Eq for INTERNET_BUFFERSA {}
impl ::core::fmt::Debug for INTERNET_BUFFERSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_BUFFERSA").field("dwStructSize", &self.dwStructSize).field("Next", &self.Next).field("lpcszHeader", &self.lpcszHeader).field("dwHeadersLength", &self.dwHeadersLength).field("dwHeadersTotal", &self.dwHeadersTotal).field("lpvBuffer", &self.lpvBuffer).field("dwBufferLength", &self.dwBufferLength).field("dwBufferTotal", &self.dwBufferTotal).field("dwOffsetLow", &self.dwOffsetLow).field("dwOffsetHigh", &self.dwOffsetHigh).finish()
    }
}
impl ::core::default::Default for INTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_BUFFERSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
impl ::core::cmp::Eq for INTERNET_BUFFERSW {}
impl ::core::fmt::Debug for INTERNET_BUFFERSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_BUFFERSW").field("dwStructSize", &self.dwStructSize).field("Next", &self.Next).field("lpcszHeader", &self.lpcszHeader).field("dwHeadersLength", &self.dwHeadersLength).field("dwHeadersTotal", &self.dwHeadersTotal).field("lpvBuffer", &self.lpvBuffer).field("dwBufferLength", &self.dwBufferLength).field("dwBufferTotal", &self.dwBufferTotal).field("dwOffsetLow", &self.dwOffsetLow).field("dwOffsetHigh", &self.dwOffsetHigh).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYA").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {}
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYW").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOA {}
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONTAINER_INFOA").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOW {}
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_CONTAINER_INFOW").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_GROUP_INFOA").field("dwGroupSize", &self.dwGroupSize).field("dwGroupFlags", &self.dwGroupFlags).field("dwGroupType", &self.dwGroupType).field("dwDiskUsage", &self.dwDiskUsage).field("dwDiskQuota", &self.dwDiskQuota).field("dwOwnerStorage", &self.dwOwnerStorage).field("szGroupName", &self.szGroupName).finish()
    }
}
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOW {}
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_GROUP_INFOW").field("dwGroupSize", &self.dwGroupSize).field("dwGroupFlags", &self.dwGroupFlags).field("dwGroupType", &self.dwGroupType).field("dwDiskUsage", &self.dwDiskUsage).field("dwDiskQuota", &self.dwDiskQuota).field("dwOwnerStorage", &self.dwOwnerStorage).field("szGroupName", &self.szGroupName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpires == other.ftExpires && self.ftLastModified == other.ftLastModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_TIMESTAMPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_TIMESTAMPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CACHE_TIMESTAMPS").field("ftExpires", &self.ftExpires).field("ftLastModified", &self.ftLastModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CALLBACK_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CALLBACK_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszName == other.pcwszName && self.pcwszValue == other.pcwszValue && self.pcwszDomain == other.pcwszDomain && self.pcwszPath == other.pcwszPath && self.ftExpires == other.ftExpires && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CALLBACK_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CALLBACK_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CALLBACK_COOKIE").field("pcwszName", &self.pcwszName).field("pcwszValue", &self.pcwszValue).field("pcwszDomain", &self.pcwszDomain).field("pcwszPath", &self.pcwszPath).field("ftExpires", &self.ftExpires).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpiry == other.ftExpiry && self.ftStart == other.ftStart && self.lpszSubjectInfo == other.lpszSubjectInfo && self.lpszIssuerInfo == other.lpszIssuerInfo && self.lpszProtocolName == other.lpszProtocolName && self.lpszSignatureAlgName == other.lpszSignatureAlgName && self.lpszEncryptionAlgName == other.lpszEncryptionAlgName && self.dwKeySize == other.dwKeySize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CERTIFICATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CERTIFICATE_INFO").field("ftExpiry", &self.ftExpiry).field("ftStart", &self.ftStart).field("lpszSubjectInfo", &self.lpszSubjectInfo).field("lpszIssuerInfo", &self.lpszIssuerInfo).field("lpszProtocolName", &self.lpszProtocolName).field("lpszSignatureAlgName", &self.lpszSignatureAlgName).field("lpszEncryptionAlgName", &self.lpszEncryptionAlgName).field("dwKeySize", &self.dwKeySize).finish()
    }
}
impl ::core::default::Default for INTERNET_CONNECTED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_CONNECTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwConnectedState == other.dwConnectedState && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for INTERNET_CONNECTED_INFO {}
impl ::core::fmt::Debug for INTERNET_CONNECTED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_CONNECTED_INFO").field("dwConnectedState", &self.dwConnectedState).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for INTERNET_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_CONNECTION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INTERNET_CONNECTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_CONNECTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_CONNECTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_CONNECTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INTERNET_CONNECTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszName == other.pszName && self.pszData == other.pszData && self.pszDomain == other.pszDomain && self.pszPath == other.pszPath && self.pftExpires == other.pftExpires && self.dwFlags == other.dwFlags && self.pszUrl == other.pszUrl && self.pszP3PPolicy == other.pszP3PPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_COOKIE").field("cbSize", &self.cbSize).field("pszName", &self.pszName).field("pszData", &self.pszData).field("pszDomain", &self.pszDomain).field("pszPath", &self.pszPath).field("pftExpires", &self.pftExpires).field("dwFlags", &self.dwFlags).field("pszUrl", &self.pszUrl).field("pszP3PPolicy", &self.pszP3PPolicy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE2 {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue && self.pwszDomain == other.pwszDomain && self.pwszPath == other.pwszPath && self.dwFlags == other.dwFlags && self.ftExpires == other.ftExpires && self.fExpiresSet == other.fExpiresSet
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_COOKIE2").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).field("pwszDomain", &self.pwszDomain).field("pwszPath", &self.pwszPath).field("dwFlags", &self.dwFlags).field("ftExpires", &self.ftExpires).field("fExpiresSet", &self.fExpiresSet).finish()
    }
}
impl ::core::default::Default for INTERNET_COOKIE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_COOKIE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_COOKIE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Socket == other.Socket && self.SourcePort == other.SourcePort && self.DestPort == other.DestPort && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for INTERNET_DIAGNOSTIC_SOCKET_INFO {}
impl ::core::fmt::Debug for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_DIAGNOSTIC_SOCKET_INFO").field("Socket", &self.Socket).field("SourcePort", &self.SourcePort).field("DestPort", &self.DestPort).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszFileName == other.pcwszFileName && self.phFile == other.phFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_DOWNLOAD_MODE_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_DOWNLOAD_MODE_HANDLE").field("pcwszFileName", &self.pcwszFileName).field("phFile", &self.phFile).finish()
    }
}
impl ::core::default::Default for INTERNET_END_BROWSER_SESSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_END_BROWSER_SESSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpBuffer == other.lpBuffer && self.dwBufferLength == other.dwBufferLength
    }
}
impl ::core::cmp::Eq for INTERNET_END_BROWSER_SESSION_DATA {}
impl ::core::fmt::Debug for INTERNET_END_BROWSER_SESSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_END_BROWSER_SESSION_DATA").field("lpBuffer", &self.lpBuffer).field("dwBufferLength", &self.dwBufferLength).finish()
    }
}
impl ::core::default::Default for INTERNET_PER_CONN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_PER_CONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_PER_CONN").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PER_CONN_OPTION_LISTA").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PER_CONN_OPTION_LISTW").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
impl ::core::default::Default for INTERNET_PREFETCH_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_PREFETCH_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for INTERNET_PREFETCH_STATUS {}
impl ::core::fmt::Debug for INTERNET_PREFETCH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PREFETCH_STATUS").field("dwStatus", &self.dwStatus).field("dwSize", &self.dwSize).finish()
    }
}
impl ::core::default::Default for INTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAccessType == other.dwAccessType && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
impl ::core::cmp::Eq for INTERNET_PROXY_INFO {}
impl ::core::fmt::Debug for INTERNET_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_PROXY_INFO").field("dwAccessType", &self.dwAccessType).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::core::default::Default for INTERNET_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_SCHEME").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::default::Default for INTERNET_SECURITY_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_CONNECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fSecure == other.fSecure && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_CONNECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SECURITY_CONNECTION_INFO").field("dwSize", &self.dwSize).field("fSecure", &self.fSecure).field("connectionInfo", &self.connectionInfo).field("cipherInfo", &self.cipherInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for INTERNET_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pCertificate == other.pCertificate && self.pcCertChain == other.pcCertChain && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo && self.pcUnverifiedCertChain == other.pcUnverifiedCertChain && self.channelBindingToken == other.channelBindingToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SECURITY_INFO").field("dwSize", &self.dwSize).field("pCertificate", &self.pCertificate).field("pcCertChain", &self.pcCertChain).field("connectionInfo", &self.connectionInfo).field("cipherInfo", &self.cipherInfo).field("pcUnverifiedCertChain", &self.pcUnverifiedCertChain).field("channelBindingToken", &self.channelBindingToken).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_SERVER_CONNECTION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_SERVER_CONNECTION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.lpcwszHostName == other.lpcwszHostName && self.fProxy == other.fProxy && self.dwCounter == other.dwCounter && self.dwConnectionLimit == other.dwConnectionLimit && self.dwAvailableCreates == other.dwAvailableCreates && self.dwAvailableKeepAlives == other.dwAvailableKeepAlives && self.dwActiveConnections == other.dwActiveConnections && self.dwWaiters == other.dwWaiters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_SERVER_CONNECTION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_SERVER_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_SERVER_CONNECTION_STATE").field("lpcwszHostName", &self.lpcwszHostName).field("fProxy", &self.fProxy).field("dwCounter", &self.dwCounter).field("dwConnectionLimit", &self.dwConnectionLimit).field("dwAvailableCreates", &self.dwAvailableCreates).field("dwAvailableKeepAlives", &self.dwAvailableKeepAlives).field("dwActiveConnections", &self.dwActiveConnections).field("dwWaiters", &self.dwWaiters).finish()
    }
}
impl ::core::default::Default for INTERNET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNET_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERNET_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERNET_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
impl ::core::cmp::Eq for INTERNET_VERSION_INFO {}
impl ::core::fmt::Debug for INTERNET_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERNET_VERSION_INFO").field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::core::cmp::PartialEq for IProofOfPossessionCookieInfoManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProofOfPossessionCookieInfoManager {}
impl ::core::fmt::Debug for IProofOfPossessionCookieInfoManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProofOfPossessionCookieInfoManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProofOfPossessionCookieInfoManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProofOfPossessionCookieInfoManager2 {}
impl ::core::fmt::Debug for IProofOfPossessionCookieInfoManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProofOfPossessionCookieInfoManager2").field(&self.0).finish()
    }
}
impl ::core::default::Default for IncomingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IncomingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSession == other.cSession && self.cPersistent == other.cPersistent && self.cAccepted == other.cAccepted && self.cLeashed == other.cLeashed && self.cDowngraded == other.cDowngraded && self.cBlocked == other.cBlocked && self.pszLocation == other.pszLocation
    }
}
impl ::core::cmp::Eq for IncomingCookieState {}
impl ::core::fmt::Debug for IncomingCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IncomingCookieState").field("cSession", &self.cSession).field("cPersistent", &self.cPersistent).field("cAccepted", &self.cAccepted).field("cLeashed", &self.cLeashed).field("cDowngraded", &self.cDowngraded).field("cBlocked", &self.cBlocked).field("pszLocation", &self.pszLocation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for InternetCookieHistory {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for InternetCookieHistory {
    fn eq(&self, other: &Self) -> bool {
        self.fAccepted == other.fAccepted && self.fLeashed == other.fLeashed && self.fDowngraded == other.fDowngraded && self.fRejected == other.fRejected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for InternetCookieHistory {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for InternetCookieHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InternetCookieHistory").field("fAccepted", &self.fAccepted).field("fLeashed", &self.fLeashed).field("fDowngraded", &self.fDowngraded).field("fRejected", &self.fRejected).finish()
    }
}
impl ::core::default::Default for InternetCookieState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InternetCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InternetCookieState").field(&self.0).finish()
    }
}
impl ::core::default::Default for OutgoingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OutgoingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSent == other.cSent && self.cSuppressed == other.cSuppressed && self.pszLocation == other.pszLocation
    }
}
impl ::core::cmp::Eq for OutgoingCookieState {}
impl ::core::fmt::Debug for OutgoingCookieState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OutgoingCookieState").field("cSent", &self.cSent).field("cSuppressed", &self.cSuppressed).field("pszLocation", &self.pszLocation).finish()
    }
}
impl ::core::default::Default for PROXY_AUTO_DETECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROXY_AUTO_DETECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_AUTO_DETECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ProofOfPossessionCookieInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ProofOfPossessionCookieInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data && self.flags == other.flags && self.p3pHeader == other.p3pHeader
    }
}
impl ::core::cmp::Eq for ProofOfPossessionCookieInfo {}
impl ::core::fmt::Debug for ProofOfPossessionCookieInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProofOfPossessionCookieInfo").field("name", &self.name).field("data", &self.data).field("flags", &self.flags).field("p3pHeader", &self.p3pHeader).finish()
    }
}
impl ::core::default::Default for REQUEST_TIMES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REQUEST_TIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REQUEST_TIMES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URLCACHE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URLCACHE_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSourceUrlName == other.pwszSourceUrlName && self.pwszLocalFileName == other.pwszLocalFileName && self.dwCacheEntryType == other.dwCacheEntryType && self.dwUseCount == other.dwUseCount && self.dwHitRate == other.dwHitRate && self.dwSizeLow == other.dwSizeLow && self.dwSizeHigh == other.dwSizeHigh && self.ftLastModifiedTime == other.ftLastModifiedTime && self.ftExpireTime == other.ftExpireTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastSyncTime == other.ftLastSyncTime && self.pbHeaderInfo == other.pbHeaderInfo && self.cbHeaderInfoSize == other.cbHeaderInfoSize && self.pbExtraData == other.pbExtraData && self.cbExtraDataSize == other.cbExtraDataSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URLCACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URLCACHE_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URLCACHE_ENTRY_INFO")
            .field("pwszSourceUrlName", &self.pwszSourceUrlName)
            .field("pwszLocalFileName", &self.pwszLocalFileName)
            .field("dwCacheEntryType", &self.dwCacheEntryType)
            .field("dwUseCount", &self.dwUseCount)
            .field("dwHitRate", &self.dwHitRate)
            .field("dwSizeLow", &self.dwSizeLow)
            .field("dwSizeHigh", &self.dwSizeHigh)
            .field("ftLastModifiedTime", &self.ftLastModifiedTime)
            .field("ftExpireTime", &self.ftExpireTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastSyncTime", &self.ftLastSyncTime)
            .field("pbHeaderInfo", &self.pbHeaderInfo)
            .field("cbHeaderInfoSize", &self.cbHeaderInfoSize)
            .field("pbExtraData", &self.pbExtraData)
            .field("cbExtraDataSize", &self.cbExtraDataSize)
            .finish()
    }
}
impl ::core::default::Default for URL_CACHE_LIMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URL_CACHE_LIMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_CACHE_LIMIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for URL_COMPONENTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for URL_COMPONENTSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTSA {}
impl ::core::fmt::Debug for URL_COMPONENTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTSA")
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
impl ::core::default::Default for URL_COMPONENTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for URL_COMPONENTSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScheme == other.lpszScheme && self.dwSchemeLength == other.dwSchemeLength && self.nScheme == other.nScheme && self.lpszHostName == other.lpszHostName && self.dwHostNameLength == other.dwHostNameLength && self.nPort == other.nPort && self.lpszUserName == other.lpszUserName && self.dwUserNameLength == other.dwUserNameLength && self.lpszPassword == other.lpszPassword && self.dwPasswordLength == other.dwPasswordLength && self.lpszUrlPath == other.lpszUrlPath && self.dwUrlPathLength == other.dwUrlPathLength && self.lpszExtraInfo == other.lpszExtraInfo && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
impl ::core::cmp::Eq for URL_COMPONENTSW {}
impl ::core::fmt::Debug for URL_COMPONENTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URL_COMPONENTSW")
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fProxy == other.fProxy && self.fBypass == other.fBypass && self.ProxyScheme == other.ProxyScheme && self.pwszProxy == other.pwszProxy && self.ProxyPort == other.ProxyPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WININET_PROXY_INFO").field("fProxy", &self.fProxy).field("fBypass", &self.fBypass).field("ProxyScheme", &self.ProxyScheme).field("pwszProxy", &self.pwszProxy).field("ProxyPort", &self.ProxyPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProxyInfoCount == other.dwProxyInfoCount && self.pProxyInfo == other.pProxyInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WININET_PROXY_INFO_LIST").field("dwProxyInfoCount", &self.dwProxyInfoCount).field("pProxyInfo", &self.pProxyInfo).finish()
    }
}
impl ::core::default::Default for WININET_SYNC_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WININET_SYNC_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WININET_SYNC_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPAD_CACHE_DELETE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPAD_CACHE_DELETE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPAD_CACHE_DELETE").field(&self.0).finish()
    }
}
