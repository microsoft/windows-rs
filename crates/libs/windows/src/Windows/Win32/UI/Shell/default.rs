impl ::core::default::Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AASHELLMENUFILENAME {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotal == other.cbTotal && self.rgbReserved == other.rgbReserved && self.szFileName == other.szFileName
    }
}
impl ::core::cmp::Eq for AASHELLMENUFILENAME {}
impl ::core::fmt::Debug for AASHELLMENUFILENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AASHELLMENUFILENAME").field("cbTotal", &self.cbTotal).field("rgbReserved", &self.rgbReserved).field("szFileName", &self.szFileName).finish()
    }
}
impl ::core::default::Default for AASHELLMENUITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AASHELLMENUITEM {
    fn eq(&self, other: &Self) -> bool {
        self.lpReserved1 == other.lpReserved1 && self.iReserved == other.iReserved && self.uiReserved == other.uiReserved && self.lpName == other.lpName && self.psz == other.psz
    }
}
impl ::core::cmp::Eq for AASHELLMENUITEM {}
impl ::core::fmt::Debug for AASHELLMENUITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AASHELLMENUITEM").field("lpReserved1", &self.lpReserved1).field("iReserved", &self.iReserved).field("uiReserved", &self.uiReserved).field("lpName", &self.lpName).field("psz", &self.psz).finish()
    }
}
impl ::core::default::Default for ACENUMOPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACENUMOPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACENUMOPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for ACTIVATEOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVATEOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATEOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADJACENT_DISPLAY_EDGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADJACENT_DISPLAY_EDGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADJACENT_DISPLAY_EDGES").field(&self.0).finish()
    }
}
impl ::core::default::Default for AHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AHE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AHTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AHTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AHTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPACTIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPACTIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPACTIONFLAGS").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APPBARDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APPBARDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for APPCATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APPCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Locale == other.Locale && self.pszDescription == other.pszDescription && self.AppCategoryId == other.AppCategoryId
    }
}
impl ::core::cmp::Eq for APPCATEGORYINFO {}
impl ::core::fmt::Debug for APPCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPCATEGORYINFO").field("Locale", &self.Locale).field("pszDescription", &self.pszDescription).field("AppCategoryId", &self.AppCategoryId).finish()
    }
}
impl ::core::default::Default for APPCATEGORYINFOLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APPCATEGORYINFOLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cCategory == other.cCategory && self.pCategoryInfo == other.pCategoryInfo
    }
}
impl ::core::cmp::Eq for APPCATEGORYINFOLIST {}
impl ::core::fmt::Debug for APPCATEGORYINFOLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPCATEGORYINFOLIST").field("cCategory", &self.cCategory).field("pCategoryInfo", &self.pCategoryInfo).finish()
    }
}
impl ::core::default::Default for APPDOCLISTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPDOCLISTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPDOCLISTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPINFODATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APPINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwMask == other.dwMask
            && self.pszDisplayName == other.pszDisplayName
            && self.pszVersion == other.pszVersion
            && self.pszPublisher == other.pszPublisher
            && self.pszProductID == other.pszProductID
            && self.pszRegisteredOwner == other.pszRegisteredOwner
            && self.pszRegisteredCompany == other.pszRegisteredCompany
            && self.pszLanguage == other.pszLanguage
            && self.pszSupportUrl == other.pszSupportUrl
            && self.pszSupportTelephone == other.pszSupportTelephone
            && self.pszHelpLink == other.pszHelpLink
            && self.pszInstallLocation == other.pszInstallLocation
            && self.pszInstallSource == other.pszInstallSource
            && self.pszInstallDate == other.pszInstallDate
            && self.pszContact == other.pszContact
            && self.pszComments == other.pszComments
            && self.pszImage == other.pszImage
            && self.pszReadmeUrl == other.pszReadmeUrl
            && self.pszUpdateInfoUrl == other.pszUpdateInfoUrl
    }
}
impl ::core::cmp::Eq for APPINFODATA {}
impl ::core::fmt::Debug for APPINFODATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPINFODATA")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("pszDisplayName", &self.pszDisplayName)
            .field("pszVersion", &self.pszVersion)
            .field("pszPublisher", &self.pszPublisher)
            .field("pszProductID", &self.pszProductID)
            .field("pszRegisteredOwner", &self.pszRegisteredOwner)
            .field("pszRegisteredCompany", &self.pszRegisteredCompany)
            .field("pszLanguage", &self.pszLanguage)
            .field("pszSupportUrl", &self.pszSupportUrl)
            .field("pszSupportTelephone", &self.pszSupportTelephone)
            .field("pszHelpLink", &self.pszHelpLink)
            .field("pszInstallLocation", &self.pszInstallLocation)
            .field("pszInstallSource", &self.pszInstallSource)
            .field("pszInstallDate", &self.pszInstallDate)
            .field("pszContact", &self.pszContact)
            .field("pszComments", &self.pszComments)
            .field("pszImage", &self.pszImage)
            .field("pszReadmeUrl", &self.pszReadmeUrl)
            .field("pszUpdateInfoUrl", &self.pszUpdateInfoUrl)
            .finish()
    }
}
impl ::core::default::Default for APPINFODATAFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPINFODATAFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPINFODATAFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPLICATION_VIEW_MIN_WIDTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATION_VIEW_MIN_WIDTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_VIEW_MIN_WIDTH").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPLICATION_VIEW_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATION_VIEW_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_VIEW_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPLICATION_VIEW_SIZE_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATION_VIEW_SIZE_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_VIEW_SIZE_PREFERENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for APPLICATION_VIEW_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATION_VIEW_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_VIEW_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCDATA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCDATA").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCF").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ASSOCF {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ASSOCF {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ASSOCF {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ASSOCF {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ASSOCF {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ASSOCIATIONLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCIATIONLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCIATIONLEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCIATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCIATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCIATIONTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCKEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCKEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCSTR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOCSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOCSTR").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOC_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASSOC_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASSOC_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for ATTACHMENT_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ATTACHMENT_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTACHMENT_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for ATTACHMENT_PROMPT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ATTACHMENT_PROMPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTACHMENT_PROMPT").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUTOCOMPLETELISTOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTOCOMPLETELISTOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTOCOMPLETELISTOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUTOCOMPLETEOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTOCOMPLETEOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTOCOMPLETEOPTIONS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTO_SCROLL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for BANDINFOSFB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::PartialEq for BANDINFOSFB {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.dwStateMask == other.dwStateMask && self.dwState == other.dwState && self.crBkgnd == other.crBkgnd && self.crBtnLt == other.crBtnLt && self.crBtnDk == other.crBtnDk && self.wViewMode == other.wViewMode && self.wAlign == other.wAlign && self.psf == other.psf && self.pidl == other.pidl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::Eq for BANDINFOSFB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::fmt::Debug for BANDINFOSFB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BANDINFOSFB").field("dwMask", &self.dwMask).field("dwStateMask", &self.dwStateMask).field("dwState", &self.dwState).field("crBkgnd", &self.crBkgnd).field("crBtnLt", &self.crBtnLt).field("crBtnDk", &self.crBtnDk).field("wViewMode", &self.wViewMode).field("wAlign", &self.wAlign).field("psf", &self.psf).field("pidl", &self.pidl).finish()
    }
}
impl ::core::default::Default for BANDSITECID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BANDSITECID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BANDSITECID").field(&self.0).finish()
    }
}
impl ::core::default::Default for BANDSITEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BANDSITEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.dwState == other.dwState && self.dwStyle == other.dwStyle
    }
}
impl ::core::cmp::Eq for BANDSITEINFO {}
impl ::core::fmt::Debug for BANDSITEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BANDSITEINFO").field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("dwStyle", &self.dwStyle).finish()
    }
}
impl ::core::default::Default for BANNER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BANNER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.providerIdentity == other.providerIdentity && self.contentId == other.contentId
    }
}
impl ::core::cmp::Eq for BANNER_NOTIFICATION {}
impl ::core::fmt::Debug for BANNER_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BANNER_NOTIFICATION").field("event", &self.event).field("providerIdentity", &self.providerIdentity).field("contentId", &self.contentId).finish()
    }
}
impl ::core::default::Default for BANNER_NOTIFICATION_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BANNER_NOTIFICATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BANNER_NOTIFICATION_EVENT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for BASEBROWSERDATALH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::PartialEq for BASEBROWSERDATALH {
    fn eq(&self, other: &Self) -> bool {
        self._hwnd == other._hwnd
            && self._ptl == other._ptl
            && self._phlf == other._phlf
            && self._pautoWB2 == other._pautoWB2
            && self._pautoEDS == other._pautoEDS
            && self._pautoSS == other._pautoSS
            && self._eSecureLockIcon == other._eSecureLockIcon
            && self._bitfield == other._bitfield
            && self._uActivateState == other._uActivateState
            && self._pidlViewState == other._pidlViewState
            && self._pctView == other._pctView
            && self._pidlCur == other._pidlCur
            && self._psv == other._psv
            && self._psf == other._psf
            && self._hwndView == other._hwndView
            && self._pszTitleCur == other._pszTitleCur
            && self._pidlPending == other._pidlPending
            && self._psvPending == other._psvPending
            && self._psfPending == other._psfPending
            && self._hwndViewPending == other._hwndViewPending
            && self._pszTitlePending == other._pszTitlePending
            && self._fIsViewMSHTML == other._fIsViewMSHTML
            && self._fPrivacyImpacted == other._fPrivacyImpacted
            && self._clsidView == other._clsidView
            && self._clsidViewPending == other._clsidViewPending
            && self._hwndFrame == other._hwndFrame
            && self._lPhishingFilterStatus == other._lPhishingFilterStatus
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::Eq for BASEBROWSERDATALH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::fmt::Debug for BASEBROWSERDATALH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BASEBROWSERDATALH")
            .field("_hwnd", &self._hwnd)
            .field("_ptl", &self._ptl)
            .field("_phlf", &self._phlf)
            .field("_pautoWB2", &self._pautoWB2)
            .field("_pautoEDS", &self._pautoEDS)
            .field("_pautoSS", &self._pautoSS)
            .field("_eSecureLockIcon", &self._eSecureLockIcon)
            .field("_bitfield", &self._bitfield)
            .field("_uActivateState", &self._uActivateState)
            .field("_pidlViewState", &self._pidlViewState)
            .field("_pctView", &self._pctView)
            .field("_pidlCur", &self._pidlCur)
            .field("_psv", &self._psv)
            .field("_psf", &self._psf)
            .field("_hwndView", &self._hwndView)
            .field("_pszTitleCur", &self._pszTitleCur)
            .field("_pidlPending", &self._pidlPending)
            .field("_psvPending", &self._psvPending)
            .field("_psfPending", &self._psfPending)
            .field("_hwndViewPending", &self._hwndViewPending)
            .field("_pszTitlePending", &self._pszTitlePending)
            .field("_fIsViewMSHTML", &self._fIsViewMSHTML)
            .field("_fPrivacyImpacted", &self._fPrivacyImpacted)
            .field("_clsidView", &self._clsidView)
            .field("_clsidViewPending", &self._clsidViewPending)
            .field("_hwndFrame", &self._hwndFrame)
            .field("_lPhishingFilterStatus", &self._lPhishingFilterStatus)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for BASEBROWSERDATAXP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::PartialEq for BASEBROWSERDATAXP {
    fn eq(&self, other: &Self) -> bool {
        self._hwnd == other._hwnd
            && self._ptl == other._ptl
            && self._phlf == other._phlf
            && self._pautoWB2 == other._pautoWB2
            && self._pautoEDS == other._pautoEDS
            && self._pautoSS == other._pautoSS
            && self._eSecureLockIcon == other._eSecureLockIcon
            && self._bitfield == other._bitfield
            && self._uActivateState == other._uActivateState
            && self._pidlViewState == other._pidlViewState
            && self._pctView == other._pctView
            && self._pidlCur == other._pidlCur
            && self._psv == other._psv
            && self._psf == other._psf
            && self._hwndView == other._hwndView
            && self._pszTitleCur == other._pszTitleCur
            && self._pidlPending == other._pidlPending
            && self._psvPending == other._psvPending
            && self._psfPending == other._psfPending
            && self._hwndViewPending == other._hwndViewPending
            && self._pszTitlePending == other._pszTitlePending
            && self._fIsViewMSHTML == other._fIsViewMSHTML
            && self._fPrivacyImpacted == other._fPrivacyImpacted
            && self._clsidView == other._clsidView
            && self._clsidViewPending == other._clsidViewPending
            && self._hwndFrame == other._hwndFrame
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::Eq for BASEBROWSERDATAXP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::fmt::Debug for BASEBROWSERDATAXP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BASEBROWSERDATAXP")
            .field("_hwnd", &self._hwnd)
            .field("_ptl", &self._ptl)
            .field("_phlf", &self._phlf)
            .field("_pautoWB2", &self._pautoWB2)
            .field("_pautoEDS", &self._pautoEDS)
            .field("_pautoSS", &self._pautoSS)
            .field("_eSecureLockIcon", &self._eSecureLockIcon)
            .field("_bitfield", &self._bitfield)
            .field("_uActivateState", &self._uActivateState)
            .field("_pidlViewState", &self._pidlViewState)
            .field("_pctView", &self._pctView)
            .field("_pidlCur", &self._pidlCur)
            .field("_psv", &self._psv)
            .field("_psf", &self._psf)
            .field("_hwndView", &self._hwndView)
            .field("_pszTitleCur", &self._pszTitleCur)
            .field("_pidlPending", &self._pidlPending)
            .field("_psvPending", &self._psvPending)
            .field("_psfPending", &self._psfPending)
            .field("_hwndViewPending", &self._hwndViewPending)
            .field("_pszTitlePending", &self._pszTitlePending)
            .field("_fIsViewMSHTML", &self._fIsViewMSHTML)
            .field("_fPrivacyImpacted", &self._fPrivacyImpacted)
            .field("_clsidView", &self._clsidView)
            .field("_clsidViewPending", &self._clsidViewPending)
            .field("_hwndFrame", &self._hwndFrame)
            .finish()
    }
}
impl ::core::default::Default for BNSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BNSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BNSTATE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for BROWSEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for BROWSEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BrowserNavConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BrowserNavConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrowserNavConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for CABINETSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CATEGORYINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CATEGORYINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CATEGORYINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CATEGORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CATEGORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cif == other.cif && self.wszName == other.wszName
    }
}
impl ::core::cmp::Eq for CATEGORY_INFO {}
impl ::core::fmt::Debug for CATEGORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORY_INFO").field("cif", &self.cif).field("wszName", &self.wszName).finish()
    }
}
impl ::core::default::Default for CATSORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CATSORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CATSORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CDBURNINGEXTENSIONRET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CDBURNINGEXTENSIONRET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CDBURNINGEXTENSIONRET").field(&self.0).finish()
    }
}
impl ::core::default::Default for CDCONTROLSTATEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CDCONTROLSTATEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CDCONTROLSTATEF").field(&self.0).finish()
    }
}
impl ::core::default::Default for CIDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for CIE4ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for CIE4ConnectionPoint {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for CIE4ConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIE4ConnectionPoint").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl CIE4ConnectionPoint {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetConnectionInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::core::Result<super::super::System::Com::IConnectionPointContainer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetConnectionPointContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), punksink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumConnections(&self) -> ::windows::core::Result<super::super::System::Com::IEnumConnections> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumConnections)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMINVOKECOMMANDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMINVOKECOMMANDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.hwnd == other.hwnd && self.lpVerb == other.lpVerb && self.lpParameters == other.lpParameters && self.lpDirectory == other.lpDirectory && self.nShow == other.nShow && self.dwHotKey == other.dwHotKey && self.hIcon == other.hIcon
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMINVOKECOMMANDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMINVOKECOMMANDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMINVOKECOMMANDINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("hwnd", &self.hwnd).field("lpVerb", &self.lpVerb).field("lpParameters", &self.lpParameters).field("lpDirectory", &self.lpDirectory).field("nShow", &self.nShow).field("dwHotKey", &self.dwHotKey).field("hIcon", &self.hIcon).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMINVOKECOMMANDINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMINVOKECOMMANDINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.hwnd == other.hwnd && self.lpVerb == other.lpVerb && self.lpParameters == other.lpParameters && self.lpDirectory == other.lpDirectory && self.nShow == other.nShow && self.dwHotKey == other.dwHotKey && self.hIcon == other.hIcon && self.lpTitle == other.lpTitle && self.lpVerbW == other.lpVerbW && self.lpParametersW == other.lpParametersW && self.lpDirectoryW == other.lpDirectoryW && self.lpTitleW == other.lpTitleW && self.ptInvoke == other.ptInvoke
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMINVOKECOMMANDINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMINVOKECOMMANDINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMINVOKECOMMANDINFOEX")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("hwnd", &self.hwnd)
            .field("lpVerb", &self.lpVerb)
            .field("lpParameters", &self.lpParameters)
            .field("lpDirectory", &self.lpDirectory)
            .field("nShow", &self.nShow)
            .field("dwHotKey", &self.dwHotKey)
            .field("hIcon", &self.hIcon)
            .field("lpTitle", &self.lpTitle)
            .field("lpVerbW", &self.lpVerbW)
            .field("lpParametersW", &self.lpParametersW)
            .field("lpDirectoryW", &self.lpDirectoryW)
            .field("lpTitleW", &self.lpTitleW)
            .field("ptInvoke", &self.ptInvoke)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMINVOKECOMMANDINFOEX_REMOTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMINVOKECOMMANDINFOEX_REMOTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.hwnd == other.hwnd && self.lpVerbString == other.lpVerbString && self.lpParameters == other.lpParameters && self.lpDirectory == other.lpDirectory && self.nShow == other.nShow && self.dwHotKey == other.dwHotKey && self.lpTitle == other.lpTitle && self.lpVerbWString == other.lpVerbWString && self.lpParametersW == other.lpParametersW && self.lpDirectoryW == other.lpDirectoryW && self.lpTitleW == other.lpTitleW && self.ptInvoke == other.ptInvoke && self.lpVerbInt == other.lpVerbInt && self.lpVerbWInt == other.lpVerbWInt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMINVOKECOMMANDINFOEX_REMOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMINVOKECOMMANDINFOEX_REMOTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMINVOKECOMMANDINFOEX_REMOTE")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("hwnd", &self.hwnd)
            .field("lpVerbString", &self.lpVerbString)
            .field("lpParameters", &self.lpParameters)
            .field("lpDirectory", &self.lpDirectory)
            .field("nShow", &self.nShow)
            .field("dwHotKey", &self.dwHotKey)
            .field("lpTitle", &self.lpTitle)
            .field("lpVerbWString", &self.lpVerbWString)
            .field("lpParametersW", &self.lpParametersW)
            .field("lpDirectoryW", &self.lpDirectoryW)
            .field("lpTitleW", &self.lpTitleW)
            .field("ptInvoke", &self.ptInvoke)
            .field("lpVerbInt", &self.lpVerbInt)
            .field("lpVerbWInt", &self.lpVerbWInt)
            .finish()
    }
}
impl ::core::default::Default for CM_COLUMNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CM_COLUMNINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwState == other.dwState && self.uWidth == other.uWidth && self.uDefaultWidth == other.uDefaultWidth && self.uIdealWidth == other.uIdealWidth && self.wszName == other.wszName
    }
}
impl ::core::cmp::Eq for CM_COLUMNINFO {}
impl ::core::fmt::Debug for CM_COLUMNINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_COLUMNINFO").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("uWidth", &self.uWidth).field("uDefaultWidth", &self.uDefaultWidth).field("uIdealWidth", &self.uIdealWidth).field("wszName", &self.wszName).finish()
    }
}
impl ::core::default::Default for CM_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CM_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_ENUM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for CM_SET_WIDTH_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CM_SET_WIDTH_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_SET_WIDTH_VALUE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONFIRM_CONFLICT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIRM_CONFLICT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pShellItem == other.pShellItem && self.pszOriginalName == other.pszOriginalName && self.pszAlternateName == other.pszAlternateName && self.pszLocationShort == other.pszLocationShort && self.pszLocationFull == other.pszLocationFull && self.nType == other.nType
    }
}
impl ::core::cmp::Eq for CONFIRM_CONFLICT_ITEM {}
impl ::core::fmt::Debug for CONFIRM_CONFLICT_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIRM_CONFLICT_ITEM").field("pShellItem", &self.pShellItem).field("pszOriginalName", &self.pszOriginalName).field("pszAlternateName", &self.pszAlternateName).field("pszLocationShort", &self.pszLocationShort).field("pszLocationFull", &self.pszLocationFull).field("nType", &self.nType).finish()
    }
}
impl ::core::default::Default for CONFIRM_CONFLICT_RESULT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIRM_CONFLICT_RESULT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszNewName == other.pszNewName && self.iItemIndex == other.iItemIndex
    }
}
impl ::core::cmp::Eq for CONFIRM_CONFLICT_RESULT_INFO {}
impl ::core::fmt::Debug for CONFIRM_CONFLICT_RESULT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIRM_CONFLICT_RESULT_INFO").field("pszNewName", &self.pszNewName).field("iItemIndex", &self.iItemIndex).finish()
    }
}
impl ::core::default::Default for CPLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CPVIEW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CPVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CPVIEW").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulAuthenticationPackage == other.ulAuthenticationPackage && self.clsidCredentialProvider == other.clsidCredentialProvider && self.cbSerialization == other.cbSerialization && self.rgbSerialization == other.rgbSerialization
    }
}
impl ::core::cmp::Eq for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION").field("ulAuthenticationPackage", &self.ulAuthenticationPackage).field("clsidCredentialProvider", &self.clsidCredentialProvider).field("cbSerialization", &self.cbSerialization).field("rgbSerialization", &self.rgbSerialization).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFieldID == other.dwFieldID && self.cpft == other.cpft && self.pszLabel == other.pszLabel && self.guidFieldType == other.guidFieldType
    }
}
impl ::core::cmp::Eq for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR").field("dwFieldID", &self.dwFieldID).field("cpft", &self.cpft).field("pszLabel", &self.pszLabel).field("guidFieldType", &self.guidFieldType).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_FIELD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_FIELD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_FIELD_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_FIELD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_STATUS_ICON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_STATUS_ICON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_STATUS_ICON").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_PROVIDER_USAGE_SCENARIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDENTIAL_PROVIDER_USAGE_SCENARIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDENTIAL_PROVIDER_USAGE_SCENARIO").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for CSFV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CommandStateChangeConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CommandStateChangeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandStateChangeConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATABLOCK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DATAOBJ_GET_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DATAOBJ_GET_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATAOBJ_GET_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEFAULTSAVEFOLDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEFAULTSAVEFOLDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFAULTSAVEFOLDERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEFAULT_FOLDER_MENU_RESTRICTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEFAULT_FOLDER_MENU_RESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFAULT_FOLDER_MENU_RESTRICTIONS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for DEFCONTEXTMENU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::PartialEq for DEFCONTEXTMENU {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pcmcb == other.pcmcb && self.pidlFolder == other.pidlFolder && self.psf == other.psf && self.cidl == other.cidl && self.apidl == other.apidl && self.punkAssociationInfo == other.punkAssociationInfo && self.cKeys == other.cKeys && self.aKeys == other.aKeys
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::Eq for DEFCONTEXTMENU {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
impl ::core::fmt::Debug for DEFCONTEXTMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEFCONTEXTMENU").field("hwnd", &self.hwnd).field("pcmcb", &self.pcmcb).field("pidlFolder", &self.pidlFolder).field("psf", &self.psf).field("cidl", &self.cidl).field("apidl", &self.apidl).field("punkAssociationInfo", &self.punkAssociationInfo).field("cKeys", &self.cKeys).field("aKeys", &self.aKeys).finish()
    }
}
impl ::core::default::Default for DEF_SHARE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEF_SHARE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEF_SHARE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DELEGATEITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DESKBANDCID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESKBANDCID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESKBANDCID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DESKBANDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DESKBANDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.ptMinSize == other.ptMinSize && self.ptMaxSize == other.ptMaxSize && self.ptIntegral == other.ptIntegral && self.ptActual == other.ptActual && self.wszTitle == other.wszTitle && self.dwModeFlags == other.dwModeFlags && self.crBkgnd == other.crBkgnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DESKBANDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DESKBANDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DESKBANDINFO").field("dwMask", &self.dwMask).field("ptMinSize", &self.ptMinSize).field("ptMaxSize", &self.ptMaxSize).field("ptIntegral", &self.ptIntegral).field("ptActual", &self.ptActual).field("wszTitle", &self.wszTitle).field("dwModeFlags", &self.dwModeFlags).field("crBkgnd", &self.crBkgnd).finish()
    }
}
impl ::core::default::Default for DESKTOP_SLIDESHOW_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESKTOP_SLIDESHOW_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESKTOP_SLIDESHOW_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DESKTOP_SLIDESHOW_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESKTOP_SLIDESHOW_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESKTOP_SLIDESHOW_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DESKTOP_SLIDESHOW_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESKTOP_SLIDESHOW_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESKTOP_SLIDESHOW_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DESKTOP_WALLPAPER_POSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESKTOP_WALLPAPER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESKTOP_WALLPAPER_POSITION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::default::Default for DETAILSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DFConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DFConstraint {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DFConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFConstraint").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFMICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFMICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.lParam == other.lParam && self.idCmdFirst == other.idCmdFirst && self.idDefMax == other.idDefMax && self.pici == other.pici && self.punkSite == other.punkSite
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFMICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DFMICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFMICS").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("lParam", &self.lParam).field("idCmdFirst", &self.idCmdFirst).field("idDefMax", &self.idDefMax).field("pici", &self.pici).field("punkSite", &self.punkSite).finish()
    }
}
impl ::core::default::Default for DFM_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFM_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFM_CMD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DFM_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFM_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFM_MESSAGE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAY_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAY_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAY_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DLLVERSIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DLLVERSIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformID == other.dwPlatformID
    }
}
impl ::core::cmp::Eq for DLLVERSIONINFO {}
impl ::core::fmt::Debug for DLLVERSIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLLVERSIONINFO").field("cbSize", &self.cbSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("dwBuildNumber", &self.dwBuildNumber).field("dwPlatformID", &self.dwPlatformID).finish()
    }
}
impl ::core::default::Default for DLLVERSIONINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DLLVERSIONINFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.info1 == other.info1 && self.dwFlags == other.dwFlags && self.ullVersion == other.ullVersion
    }
}
impl ::core::cmp::Eq for DLLVERSIONINFO2 {}
impl ::core::fmt::Debug for DLLVERSIONINFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLLVERSIONINFO2").field("info1", &self.info1).field("dwFlags", &self.dwFlags).field("ullVersion", &self.ullVersion).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DROPDESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DROPIMAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DROPIMAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPIMAGETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSH_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DShellFolderViewEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DShellFolderViewEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DShellFolderViewEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DShellFolderViewEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DShellNameSpaceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DShellNameSpaceEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DShellNameSpaceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DShellNameSpaceEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DShellWindowsEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DShellWindowsEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DShellWindowsEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DShellWindowsEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DWebBrowserEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DWebBrowserEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DWebBrowserEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWebBrowserEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DWebBrowserEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DWebBrowserEvents2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DWebBrowserEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWebBrowserEvents2").field(&self.0).finish()
    }
}
impl ::core::default::Default for EC_HOST_UI_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EC_HOST_UI_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_HOST_UI_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDGE_GESTURE_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDGE_GESTURE_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDGE_GESTURE_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXPLORER_BROWSER_FILL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPLORER_BROWSER_FILL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPLORER_BROWSER_FILL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXPLORER_BROWSER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPLORER_BROWSER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPLORER_BROWSER_OPTIONS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXP_DARWIN_LINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EXP_PROPERTYSTORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EXP_SPECIAL_FOLDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXP_SZ_LINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EXTRASEARCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTRASEARCH {
    fn eq(&self, other: &Self) -> bool {
        self.guidSearch == other.guidSearch && self.wszFriendlyName == other.wszFriendlyName && self.wszUrl == other.wszUrl
    }
}
impl ::core::cmp::Eq for EXTRASEARCH {}
impl ::core::fmt::Debug for EXTRASEARCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTRASEARCH").field("guidSearch", &self.guidSearch).field("wszFriendlyName", &self.wszFriendlyName).field("wszUrl", &self.wszUrl).finish()
    }
}
impl ::core::default::Default for FDAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for FDE_OVERWRITE_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDE_OVERWRITE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDE_OVERWRITE_RESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FDE_SHAREVIOLATION_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDE_SHAREVIOLATION_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDE_SHAREVIOLATION_RESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FFFP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FFFP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FFFP_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILEDESCRIPTORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILEDESCRIPTORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILEGROUPDESCRIPTORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILEGROUPDESCRIPTORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILEOPENDIALOGOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILEOPENDIALOGOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILEOPENDIALOGOPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILEOPENDIALOGOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILEOPENDIALOGOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILEOPENDIALOGOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILEOPENDIALOGOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILEOPENDIALOGOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILETYPEATTRIBUTEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILETYPEATTRIBUTEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILETYPEATTRIBUTEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_OPERATION_FLAGS2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_OPERATION_FLAGS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_OPERATION_FLAGS2").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_USAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_USAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLYOUT_PLACEMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLYOUT_PLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLYOUT_PLACEMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for FOLDERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLDERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLDERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FOLDERLOGICALVIEWMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLDERLOGICALVIEWMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLDERLOGICALVIEWMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FOLDERSETDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FOLDERSETDATA {
    fn eq(&self, other: &Self) -> bool {
        self._fs == other._fs && self._vidRestore == other._vidRestore && self._dwViewPriority == other._dwViewPriority
    }
}
impl ::core::cmp::Eq for FOLDERSETDATA {}
impl ::core::fmt::Debug for FOLDERSETDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOLDERSETDATA").field("_fs", &self._fs).field("_vidRestore", &self._vidRestore).field("_dwViewPriority", &self._dwViewPriority).finish()
    }
}
impl ::core::default::Default for FOLDERSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FOLDERSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ViewMode == other.ViewMode && self.fFlags == other.fFlags
    }
}
impl ::core::cmp::Eq for FOLDERSETTINGS {}
impl ::core::fmt::Debug for FOLDERSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOLDERSETTINGS").field("ViewMode", &self.ViewMode).field("fFlags", &self.fFlags).finish()
    }
}
impl ::core::default::Default for FOLDERVIEWMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLDERVIEWMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLDERVIEWMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FOLDERVIEWOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLDERVIEWOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLDERVIEWOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FOLDER_ENUM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLDER_ENUM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLDER_ENUM_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FVTEXTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FVTEXTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FVTEXTTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Folder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Folder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Folder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Folder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Folder2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Folder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Folder2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl Folder2 {
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParentFolder(&self) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows::core::Result<FolderItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseName(&self, bname: &::windows::core::BSTR) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParseName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NewFolder(&self, bname: &::windows::core::BSTR, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NewFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bname), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveHere(&self, vitem: super::super::System::Com::VARIANT, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyHere(&self, vitem: super::super::System::Com::VARIANT, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDetailsOf(&self, vitem: super::super::System::Com::VARIANT, icolumn: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDetailsOf)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), icolumn, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Folder3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Folder3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Folder3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Folder3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl Folder3 {
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParentFolder(&self) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParentFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Items(&self) -> ::windows::core::Result<FolderItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Items)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseName(&self, bname: &::windows::core::BSTR) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParseName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NewFolder(&self, bname: &::windows::core::BSTR, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NewFolder)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bname), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveHere(&self, vitem: super::super::System::Com::VARIANT, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MoveHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyHere(&self, vitem: super::super::System::Com::VARIANT, voptions: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), ::core::mem::transmute(voptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDetailsOf(&self, vitem: super::super::System::Com::VARIANT, icolumn: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDetailsOf)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vitem), icolumn, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Self_(&self) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Self_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OfflineStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OfflineStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Synchronize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Synchronize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HaveToShowWebViewBarricade(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HaveToShowWebViewBarricade)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DismissedWebViewBarricade(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DismissedWebViewBarricade)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl FolderItem2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bs: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetLink(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolder(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLink(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsLink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFolder(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFileSystem(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsFileSystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBrowsable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsBrowsable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModifyDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModifyDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModifyDate(&self, dt: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetModifyDate)(::windows::core::Vtable::as_raw(self), dt).ok()
    }
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Size)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Verbs(&self) -> ::windows::core::Result<FolderItemVerbs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Verbs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InvokeVerb(&self, vverb: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvokeVerb)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vverb)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItemVerb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItemVerb {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItemVerb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItemVerb").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItemVerbs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItemVerbs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItemVerbs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItemVerbs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItems {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItems").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItems2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItems2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItems2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItems2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl FolderItems2 {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: super::super::System::Com::VARIANT) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(index), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for FolderItems3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for FolderItems3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for FolderItems3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderItems3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl FolderItems3 {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: super::super::System::Com::VARIANT) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(index), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InvokeVerbEx(&self, vverb: super::super::System::Com::VARIANT, vargs: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvokeVerbEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vverb), ::core::mem::transmute(vargs)).ok()
    }
}
impl ::core::default::Default for GPFIDL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GPFIDL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPFIDL_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iContextType == other.iContextType && self.iCtrlId == other.iCtrlId && self.hItemHandle == other.hItemHandle && self.dwContextId == other.dwContextId && self.MousePos == other.MousePos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HELPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HELPINFO").field("cbSize", &self.cbSize).field("iContextType", &self.iContextType).field("iCtrlId", &self.iCtrlId).field("hItemHandle", &self.hItemHandle).field("dwContextId", &self.dwContextId).field("MousePos", &self.MousePos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPWININFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPWININFOA {
    fn eq(&self, other: &Self) -> bool {
        self.wStructSize == other.wStructSize && self.x == other.x && self.y == other.y && self.dx == other.dx && self.dy == other.dy && self.wMax == other.wMax && self.rgchMember == other.rgchMember
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPWININFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HELPWININFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HELPWININFOA").field("wStructSize", &self.wStructSize).field("x", &self.x).field("y", &self.y).field("dx", &self.dx).field("dy", &self.dy).field("wMax", &self.wMax).field("rgchMember", &self.rgchMember).finish()
    }
}
impl ::core::default::Default for HELPWININFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HELPWININFOW {
    fn eq(&self, other: &Self) -> bool {
        self.wStructSize == other.wStructSize && self.x == other.x && self.y == other.y && self.dx == other.dx && self.dy == other.dy && self.wMax == other.wMax && self.rgchMember == other.rgchMember
    }
}
impl ::core::cmp::Eq for HELPWININFOW {}
impl ::core::fmt::Debug for HELPWININFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HELPWININFOW").field("wStructSize", &self.wStructSize).field("x", &self.x).field("y", &self.y).field("dx", &self.dx).field("dy", &self.dy).field("wMax", &self.wMax).field("rgchMember", &self.rgchMember).finish()
    }
}
impl ::core::default::Default for HELP_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HELP_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HELP_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLBWIF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLBWIF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLBWIF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HLBWIF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HLBWIF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HLBWIF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HLBWIF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HLBWIF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HLBWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HLBWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.grfHLBWIF == other.grfHLBWIF && self.rcFramePos == other.rcFramePos && self.rcDocPos == other.rcDocPos && self.hltbinfo == other.hltbinfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HLBWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HLBWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLBWINFO").field("cbSize", &self.cbSize).field("grfHLBWIF", &self.grfHLBWIF).field("rcFramePos", &self.rcFramePos).field("rcDocPos", &self.rcDocPos).field("hltbinfo", &self.hltbinfo).finish()
    }
}
impl ::core::default::Default for HLFNAMEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLFNAMEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLFNAMEF").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HLFNAMEF {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HLFNAMEF {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HLFNAMEF {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HLFNAMEF {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HLFNAMEF {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HLID_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLID_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLINKGETREF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLINKGETREF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLINKGETREF").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLINKMISC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLINKMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLINKMISC").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLINKSETF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLINKSETF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLINKSETF").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLINKWHICHMK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLINKWHICHMK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLINKWHICHMK").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HLITEM {
    fn eq(&self, other: &Self) -> bool {
        self.uHLID == other.uHLID && self.pwzFriendlyName == other.pwzFriendlyName
    }
}
impl ::core::cmp::Eq for HLITEM {}
impl ::core::fmt::Debug for HLITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLITEM").field("uHLID", &self.uHLID).field("pwzFriendlyName", &self.pwzFriendlyName).finish()
    }
}
impl ::core::default::Default for HLNF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLNF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLNF").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HLNF {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HLNF {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HLNF {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HLNF {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HLNF {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HLQF_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLQF_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLQF_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLSHORTCUTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLSHORTCUTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLSHORTCUTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLSR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLSR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLSR").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HLTBINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HLTBINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uDockType == other.uDockType && self.rcTbPos == other.rcTbPos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HLTBINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HLTBINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLTBINFO").field("uDockType", &self.uDockType).field("rcTbPos", &self.rcTbPos).finish()
    }
}
impl ::core::default::Default for HLTB_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLTB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLTB_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for HLTRANSLATEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HLTRANSLATEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HLTRANSLATEF").field(&self.0).finish()
    }
}
impl ::core::default::Default for HOMEGROUPSHARINGCHOICES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOMEGROUPSHARINGCHOICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOMEGROUPSHARINGCHOICES").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IACList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IACList {}
impl ::core::fmt::Debug for IACList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IACList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IACList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IACList2 {}
impl ::core::fmt::Debug for IACList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IACList2").field(&self.0).finish()
    }
}
impl IACList2 {
    pub unsafe fn Expand<P0>(&self, pszexpand: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Expand)(::windows::core::Vtable::as_raw(self), pszexpand.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IAccessibilityDockingService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibilityDockingService {}
impl ::core::fmt::Debug for IAccessibilityDockingService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibilityDockingService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibilityDockingServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibilityDockingServiceCallback {}
impl ::core::fmt::Debug for IAccessibilityDockingServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibilityDockingServiceCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibleObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibleObject {}
impl ::core::fmt::Debug for IAccessibleObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActionProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActionProgress {}
impl ::core::fmt::Debug for IActionProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActionProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActionProgressDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActionProgressDialog {}
impl ::core::fmt::Debug for IActionProgressDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActionProgressDialog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAppActivationUIInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppActivationUIInfo {}
impl ::core::fmt::Debug for IAppActivationUIInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppActivationUIInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAppPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppPublisher {}
impl ::core::fmt::Debug for IAppPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppPublisher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAppVisibility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppVisibility {}
impl ::core::fmt::Debug for IAppVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppVisibility").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAppVisibilityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppVisibilityEvents {}
impl ::core::fmt::Debug for IAppVisibilityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppVisibilityEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationActivationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationActivationManager {}
impl ::core::fmt::Debug for IApplicationActivationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationActivationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationAssociationRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationAssociationRegistration {}
impl ::core::fmt::Debug for IApplicationAssociationRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationAssociationRegistration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationAssociationRegistrationUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationAssociationRegistrationUI {}
impl ::core::fmt::Debug for IApplicationAssociationRegistrationUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationAssociationRegistrationUI").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationDesignModeSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDesignModeSettings {}
impl ::core::fmt::Debug for IApplicationDesignModeSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDesignModeSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationDesignModeSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDesignModeSettings2 {}
impl ::core::fmt::Debug for IApplicationDesignModeSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDesignModeSettings2").field(&self.0).finish()
    }
}
impl IApplicationDesignModeSettings2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNativeDisplaySize(&self, nativedisplaysizepixels: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNativeDisplaySize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(nativedisplaysizepixels)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetScaleFactor(&self, scalefactor: Common::DEVICE_SCALE_FACTOR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetScaleFactor)(::windows::core::Vtable::as_raw(self), scalefactor).ok()
    }
    pub unsafe fn SetApplicationViewState(&self, viewstate: APPLICATION_VIEW_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetApplicationViewState)(::windows::core::Vtable::as_raw(self), viewstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComputeApplicationSize(&self) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeApplicationSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn IsApplicationViewStateSupported(&self, viewstate: APPLICATION_VIEW_STATE, nativedisplaysizepixels: super::super::Foundation::SIZE, scalefactor: Common::DEVICE_SCALE_FACTOR) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsApplicationViewStateSupported)(::windows::core::Vtable::as_raw(self), viewstate, ::core::mem::transmute(nativedisplaysizepixels), scalefactor, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TriggerEdgeGesture(&self, edgegesturekind: EDGE_GESTURE_KIND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TriggerEdgeGesture)(::windows::core::Vtable::as_raw(self), edgegesturekind).ok()
    }
}
impl ::core::cmp::PartialEq for IApplicationDestinations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDestinations {}
impl ::core::fmt::Debug for IApplicationDestinations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDestinations").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IApplicationDocumentLists {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDocumentLists {}
impl ::core::fmt::Debug for IApplicationDocumentLists {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDocumentLists").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAssocHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssocHandler {}
impl ::core::fmt::Debug for IAssocHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssocHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAssocHandlerInvoker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssocHandlerInvoker {}
impl ::core::fmt::Debug for IAssocHandlerInvoker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssocHandlerInvoker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAttachmentExecute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAttachmentExecute {}
impl ::core::fmt::Debug for IAttachmentExecute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAttachmentExecute").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAutoComplete {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAutoComplete {}
impl ::core::fmt::Debug for IAutoComplete {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutoComplete").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAutoComplete2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAutoComplete2 {}
impl ::core::fmt::Debug for IAutoComplete2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutoComplete2").field(&self.0).finish()
    }
}
impl IAutoComplete2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<P0, P1, P2, P3>(&self, hwndedit: P0, punkacl: P1, pwszregkeypath: P2, pwszquickcomplete: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Init)(::windows::core::Vtable::as_raw(self), hwndedit.into(), punkacl.into().abi(), pwszregkeypath.into().abi(), pwszquickcomplete.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Enable)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IAutoCompleteDropDown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAutoCompleteDropDown {}
impl ::core::fmt::Debug for IAutoCompleteDropDown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutoCompleteDropDown").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBandHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBandHost {}
impl ::core::fmt::Debug for IBandHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBandHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBandSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBandSite {}
impl ::core::fmt::Debug for IBandSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBandSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBannerNotificationHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBannerNotificationHandler {}
impl ::core::fmt::Debug for IBannerNotificationHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBannerNotificationHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBanneredBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBanneredBar {}
impl ::core::fmt::Debug for IBanneredBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBanneredBar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBrowserFrameOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBrowserFrameOptions {}
impl ::core::fmt::Debug for IBrowserFrameOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBrowserFrameOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBrowserService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBrowserService {}
impl ::core::fmt::Debug for IBrowserService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBrowserService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBrowserService2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBrowserService2 {}
impl ::core::fmt::Debug for IBrowserService2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBrowserService2").field(&self.0).finish()
    }
}
impl IBrowserService2 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetParentSite(&self) -> ::windows::core::Result<super::super::System::Ole::IOleInPlaceSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentSite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetTitle<P0, P1>(&self, psv: P0, pszname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pszname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTitle<P0>(&self, psv: P0, pszname: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), ::core::mem::transmute(pszname.as_ptr()), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetOleObject(&self) -> ::windows::core::Result<super::super::System::Ole::IOleObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOleObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTravelLog(&self) -> ::windows::core::Result<ITravelLog> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTravelLog)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowControlWindow<P0>(&self, id: u32, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowControlWindow)(::windows::core::Vtable::as_raw(self), id, fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsControlWindowShown(&self, id: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsControlWindowShown)(::windows::core::Vtable::as_raw(self), id, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEGetDisplayName(&self, pidl: *const Common::ITEMIDLIST, pwszname: ::windows::core::PWSTR, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IEGetDisplayName)(::windows::core::Vtable::as_raw(self), pidl, ::core::mem::transmute(pwszname), uflags).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEParseDisplayName<P0>(&self, uicp: u32, pwszpath: P0) -> ::windows::core::Result<*mut Common::ITEMIDLIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IEParseDisplayName)(::windows::core::Vtable::as_raw(self), uicp, pwszpath.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayParseError<P0>(&self, hres: ::windows::core::HRESULT, pwszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DisplayParseError)(::windows::core::Vtable::as_raw(self), hres, pwszpath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn NavigateToPidl(&self, pidl: *const Common::ITEMIDLIST, grfhlnf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NavigateToPidl)(::windows::core::Vtable::as_raw(self), pidl, grfhlnf).ok()
    }
    pub unsafe fn SetNavigateState(&self, bnstate: BNSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNavigateState)(::windows::core::Vtable::as_raw(self), bnstate).ok()
    }
    pub unsafe fn GetNavigateState(&self) -> ::windows::core::Result<BNSTATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNavigateState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn NotifyRedirect<P0>(&self, psv: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NotifyRedirect)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pidl, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UpdateWindowList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UpdateWindowList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UpdateBackForwardState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UpdateBackForwardState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, dwflags: u32, dwflagmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFlags)(::windows::core::Vtable::as_raw(self), dwflags, dwflagmask).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanNavigateNow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CanNavigateNow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetPidl(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPidl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetReferrer(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetReferrer)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
    pub unsafe fn GetBrowserIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetBrowserIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBrowserByIndex(&self, dwid: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBrowserByIndex)(::windows::core::Vtable::as_raw(self), dwid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetHistoryObject(&self, ppole: *mut ::core::option::Option<super::super::System::Ole::IOleObject>, pstm: *mut ::core::option::Option<super::super::System::Com::IStream>, ppbc: *mut ::core::option::Option<super::super::System::Com::IBindCtx>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetHistoryObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppole), ::core::mem::transmute(pstm), ::core::mem::transmute(ppbc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHistoryObject<P0, P1>(&self, pole: P0, fislocalanchor: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHistoryObject)(::windows::core::Vtable::as_raw(self), pole.into().abi(), fislocalanchor.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn CacheOLEServer<P0>(&self, pole: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CacheOLEServer)(::windows::core::Vtable::as_raw(self), pole.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSetCodePage(&self, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSetCodePage)(::windows::core::Vtable::as_raw(self), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnHttpEquiv<P0, P1>(&self, psv: P0, fdone: P1, pvarargin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnHttpEquiv)(::windows::core::Vtable::as_raw(self), psv.into().abi(), fdone.into(), pvarargin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HPALETTE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPalette)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWindow<P0>(&self, fforceregister: P0, swc: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterWindow)(::windows::core::Vtable::as_raw(self), fforceregister.into(), swc).ok()
    }
}
impl ::core::cmp::PartialEq for IBrowserService3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBrowserService3 {}
impl ::core::fmt::Debug for IBrowserService3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBrowserService3").field(&self.0).finish()
    }
}
impl IBrowserService3 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetParentSite(&self) -> ::windows::core::Result<super::super::System::Ole::IOleInPlaceSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParentSite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetTitle<P0, P1>(&self, psv: P0, pszname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pszname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTitle<P0>(&self, psv: P0, pszname: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), ::core::mem::transmute(pszname.as_ptr()), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetOleObject(&self) -> ::windows::core::Result<super::super::System::Ole::IOleObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOleObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTravelLog(&self) -> ::windows::core::Result<ITravelLog> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetTravelLog)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowControlWindow<P0>(&self, id: u32, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ShowControlWindow)(::windows::core::Vtable::as_raw(self), id, fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsControlWindowShown(&self, id: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsControlWindowShown)(::windows::core::Vtable::as_raw(self), id, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEGetDisplayName(&self, pidl: *const Common::ITEMIDLIST, pwszname: ::windows::core::PWSTR, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IEGetDisplayName)(::windows::core::Vtable::as_raw(self), pidl, ::core::mem::transmute(pwszname), uflags).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEParseDisplayName<P0>(&self, uicp: u32, pwszpath: P0) -> ::windows::core::Result<*mut Common::ITEMIDLIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IEParseDisplayName)(::windows::core::Vtable::as_raw(self), uicp, pwszpath.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayParseError<P0>(&self, hres: ::windows::core::HRESULT, pwszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DisplayParseError)(::windows::core::Vtable::as_raw(self), hres, pwszpath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn NavigateToPidl(&self, pidl: *const Common::ITEMIDLIST, grfhlnf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NavigateToPidl)(::windows::core::Vtable::as_raw(self), pidl, grfhlnf).ok()
    }
    pub unsafe fn SetNavigateState(&self, bnstate: BNSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNavigateState)(::windows::core::Vtable::as_raw(self), bnstate).ok()
    }
    pub unsafe fn GetNavigateState(&self) -> ::windows::core::Result<BNSTATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNavigateState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn NotifyRedirect<P0>(&self, psv: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NotifyRedirect)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pidl, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UpdateWindowList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateWindowList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UpdateBackForwardState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateBackForwardState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, dwflags: u32, dwflagmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFlags)(::windows::core::Vtable::as_raw(self), dwflags, dwflagmask).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanNavigateNow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CanNavigateNow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetPidl(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPidl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetReferrer(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetReferrer)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
    pub unsafe fn GetBrowserIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBrowserIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBrowserByIndex(&self, dwid: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBrowserByIndex)(::windows::core::Vtable::as_raw(self), dwid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetHistoryObject(&self, ppole: *mut ::core::option::Option<super::super::System::Ole::IOleObject>, pstm: *mut ::core::option::Option<super::super::System::Com::IStream>, ppbc: *mut ::core::option::Option<super::super::System::Com::IBindCtx>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetHistoryObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppole), ::core::mem::transmute(pstm), ::core::mem::transmute(ppbc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHistoryObject<P0, P1>(&self, pole: P0, fislocalanchor: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHistoryObject)(::windows::core::Vtable::as_raw(self), pole.into().abi(), fislocalanchor.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn CacheOLEServer<P0>(&self, pole: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheOLEServer)(::windows::core::Vtable::as_raw(self), pole.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSetCodePage(&self, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSetCodePage)(::windows::core::Vtable::as_raw(self), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnHttpEquiv<P0, P1>(&self, psv: P0, fdone: P1, pvarargin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OnHttpEquiv)(::windows::core::Vtable::as_raw(self), psv.into().abi(), fdone.into(), pvarargin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HPALETTE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPalette)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWindow<P0>(&self, fforceregister: P0, swc: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterWindow)(::windows::core::Vtable::as_raw(self), fforceregister.into(), swc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WndProcBS<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.WndProcBS)(::windows::core::Vtable::as_raw(self), hwnd.into(), umsg, wparam.into(), lparam.into())
    }
    pub unsafe fn SetAsDefFolderSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAsDefFolderSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSize<P0>(&self, wparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnSize)(::windows::core::Vtable::as_raw(self), wparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnCreate(&self, pcs: *const super::WindowsAndMessaging::CREATESTRUCTW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnCreate)(::windows::core::Vtable::as_raw(self), pcs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCommand<P0, P1>(&self, wparam: P0, lparam: P1) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnCommand)(::windows::core::Vtable::as_raw(self), wparam.into(), lparam.into())
    }
    pub unsafe fn OnDestroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnDestroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn OnNotify(&self, pnm: *const super::Controls::NMHDR) -> super::super::Foundation::LRESULT {
        (::windows::core::Vtable::vtable(self).base__.OnNotify)(::windows::core::Vtable::as_raw(self), pnm)
    }
    pub unsafe fn OnSetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnSetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFrameWindowActivateBS<P0>(&self, factive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnFrameWindowActivateBS)(::windows::core::Vtable::as_raw(self), factive.into()).ok()
    }
    pub unsafe fn ReleaseShellView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseShellView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ActivatePendingView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ActivatePendingView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateViewWindow<P0, P1>(&self, psvnew: P0, psvold: P1, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateViewWindow)(::windows::core::Vtable::as_raw(self), psvnew.into().abi(), psvold.into().abi(), prcview, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBrowserPropSheetExt(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBrowserPropSheetExt)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn GetBaseBrowserData(&self) -> ::windows::core::Result<*mut BASEBROWSERDATALH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBaseBrowserData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn PutBaseBrowserData(&self) -> *mut BASEBROWSERDATALH {
        (::windows::core::Vtable::vtable(self).base__.PutBaseBrowserData)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn InitializeTravelLog<P0>(&self, ptl: P0, dw: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITravelLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeTravelLog)(::windows::core::Vtable::as_raw(self), ptl.into().abi(), dw).ok()
    }
    pub unsafe fn SetTopBrowser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTopBrowser)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Offline(&self, icmd: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Offline)(::windows::core::Vtable::as_raw(self), icmd).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowViewResize<P0>(&self, f: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AllowViewResize)(::windows::core::Vtable::as_raw(self), f.into()).ok()
    }
    pub unsafe fn SetActivateState(&self, u: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetActivateState)(::windows::core::Vtable::as_raw(self), u).ok()
    }
    pub unsafe fn UpdateSecureLockIcon(&self, esecurelock: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UpdateSecureLockIcon)(::windows::core::Vtable::as_raw(self), esecurelock).ok()
    }
    pub unsafe fn InitializeDownloadManager(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDownloadManager)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InitializeTransitionSite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeTransitionSite)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _Initialize<P0, P1>(&self, hwnd: P0, pauto: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__._Initialize)(::windows::core::Vtable::as_raw(self), hwnd.into(), pauto.into().abi()).ok()
    }
    pub unsafe fn _CancelPendingNavigationAsync(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._CancelPendingNavigationAsync)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _CancelPendingView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._CancelPendingView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _MaySaveChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._MaySaveChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _PauseOrResumeView<P0>(&self, fpaused: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__._PauseOrResumeView)(::windows::core::Vtable::as_raw(self), fpaused.into()).ok()
    }
    pub unsafe fn _DisableModeless(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._DisableModeless)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn _NavigateToPidl2(&self, pidl: *const Common::ITEMIDLIST, grfhlnf: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._NavigateToPidl2)(::windows::core::Vtable::as_raw(self), pidl, grfhlnf, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn _TryShell2Rename<P0>(&self, psv: P0, pidlnew: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__._TryShell2Rename)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pidlnew).ok()
    }
    pub unsafe fn _SwitchActivationNow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._SwitchActivationNow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn _ExecChildren<P0, P1>(&self, punkbar: P0, fbroadcast: P1, pguidcmdgroup: ::core::option::Option<*const ::windows::core::GUID>, ncmdid: u32, ncmdexecopt: u32, pvarargin: ::core::option::Option<*const super::super::System::Com::VARIANT>, pvarargout: ::core::option::Option<*mut super::super::System::Com::VARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__._ExecChildren)(::windows::core::Vtable::as_raw(self), punkbar.into().abi(), fbroadcast.into(), ::core::mem::transmute(pguidcmdgroup.unwrap_or(::std::ptr::null())), ncmdid, ncmdexecopt, ::core::mem::transmute(pvarargin.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvarargout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _SendChildren<P0, P1, P2, P3>(&self, hwndbar: P0, fbroadcast: P1, umsg: u32, wparam: P2, lparam: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__._SendChildren)(::windows::core::Vtable::as_raw(self), hwndbar.into(), fbroadcast.into(), umsg, wparam.into(), lparam.into()).ok()
    }
    pub unsafe fn GetFolderSetData(&self, pfsd: *mut FOLDERSETDATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFolderSetData)(::windows::core::Vtable::as_raw(self), pfsd).ok()
    }
    pub unsafe fn _OnFocusChange(&self, itb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._OnFocusChange)(::windows::core::Vtable::as_raw(self), itb).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn v_ShowHideChildWindows<P0>(&self, fchildonly: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.v_ShowHideChildWindows)(::windows::core::Vtable::as_raw(self), fchildonly.into()).ok()
    }
    pub unsafe fn _get_itbLastFocus(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__._get_itbLastFocus)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn _put_itbLastFocus(&self, itblastfocus: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._put_itbLastFocus)(::windows::core::Vtable::as_raw(self), itblastfocus).ok()
    }
    pub unsafe fn _UIActivateView(&self, ustate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._UIActivateView)(::windows::core::Vtable::as_raw(self), ustate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _GetViewBorderRect(&self, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._GetViewBorderRect)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
    pub unsafe fn _UpdateViewRectSize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._UpdateViewRectSize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _ResizeNextBorder(&self, itb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._ResizeNextBorder)(::windows::core::Vtable::as_raw(self), itb).ok()
    }
    pub unsafe fn _ResizeView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__._ResizeView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn _GetEffectiveClientArea<P0>(&self, lprectborder: *mut super::super::Foundation::RECT, hmon: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HMONITOR>,
    {
        (::windows::core::Vtable::vtable(self).base__._GetEffectiveClientArea)(::windows::core::Vtable::as_raw(self), lprectborder, hmon.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn v_GetViewStream<P0>(&self, pidl: *mut Common::ITEMIDLIST, grfmode: u32, pwszname: P0) -> ::core::option::Option<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.v_GetViewStream)(::windows::core::Vtable::as_raw(self), pidl, grfmode, pwszname.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ForwardViewMsg<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.ForwardViewMsg)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into())
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetAcceleratorMenu<P0>(&self, hacc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HACCEL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAcceleratorMenu)(::windows::core::Vtable::as_raw(self), hacc.into()).ok()
    }
    pub unsafe fn _GetToolbarCount(&self) -> i32 {
        (::windows::core::Vtable::vtable(self).base__._GetToolbarCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
    pub unsafe fn _GetToolbarItem(&self, itb: i32) -> *mut TOOLBARITEM {
        (::windows::core::Vtable::vtable(self).base__._GetToolbarItem)(::windows::core::Vtable::as_raw(self), itb)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn _SaveToolbars<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__._SaveToolbars)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn _LoadToolbars<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__._LoadToolbars)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _CloseAndReleaseToolbars<P0>(&self, fclose: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__._CloseAndReleaseToolbars)(::windows::core::Vtable::as_raw(self), fclose.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn v_MayGetNextToolbarFocus(&self, lpmsg: *const super::WindowsAndMessaging::MSG, itbnext: u32, citb: i32, pptbi: *mut *mut TOOLBARITEM, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.v_MayGetNextToolbarFocus)(::windows::core::Vtable::as_raw(self), lpmsg, itbnext, citb, pptbi, phwnd).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _ResizeNextBorderHelper<P0>(&self, itb: u32, busehmonitor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__._ResizeNextBorderHelper)(::windows::core::Vtable::as_raw(self), itb, busehmonitor.into()).ok()
    }
    pub unsafe fn _FindTBar<P0>(&self, punksrc: P0) -> u32
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__._FindTBar)(::windows::core::Vtable::as_raw(self), punksrc.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn _SetFocus<P0>(&self, ptbi: *const TOOLBARITEM, hwnd: P0, lpmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__._SetFocus)(::windows::core::Vtable::as_raw(self), ptbi, hwnd.into(), lpmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn v_MayTranslateAccelerator(&self, pmsg: *mut super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.v_MayTranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _GetBorderDWHelper<P0, P1>(&self, punksrc: P0, lprectborder: *mut super::super::Foundation::RECT, busehmonitor: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__._GetBorderDWHelper)(::windows::core::Vtable::as_raw(self), punksrc.into().abi(), lprectborder, busehmonitor.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn v_CheckZoneCrossing(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.v_CheckZoneCrossing)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
}
impl ::core::cmp::PartialEq for IBrowserService4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBrowserService4 {}
impl ::core::fmt::Debug for IBrowserService4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBrowserService4").field(&self.0).finish()
    }
}
impl IBrowserService4 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetParentSite(&self) -> ::windows::core::Result<super::super::System::Ole::IOleInPlaceSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParentSite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetTitle<P0, P1>(&self, psv: P0, pszname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pszname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTitle<P0>(&self, psv: P0, pszname: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTitle)(::windows::core::Vtable::as_raw(self), psv.into().abi(), ::core::mem::transmute(pszname.as_ptr()), pszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetOleObject(&self) -> ::windows::core::Result<super::super::System::Ole::IOleObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOleObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTravelLog(&self) -> ::windows::core::Result<ITravelLog> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTravelLog)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowControlWindow<P0>(&self, id: u32, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowControlWindow)(::windows::core::Vtable::as_raw(self), id, fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsControlWindowShown(&self, id: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsControlWindowShown)(::windows::core::Vtable::as_raw(self), id, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEGetDisplayName(&self, pidl: *const Common::ITEMIDLIST, pwszname: ::windows::core::PWSTR, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IEGetDisplayName)(::windows::core::Vtable::as_raw(self), pidl, ::core::mem::transmute(pwszname), uflags).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEParseDisplayName<P0>(&self, uicp: u32, pwszpath: P0) -> ::windows::core::Result<*mut Common::ITEMIDLIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IEParseDisplayName)(::windows::core::Vtable::as_raw(self), uicp, pwszpath.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayParseError<P0>(&self, hres: ::windows::core::HRESULT, pwszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DisplayParseError)(::windows::core::Vtable::as_raw(self), hres, pwszpath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn NavigateToPidl(&self, pidl: *const Common::ITEMIDLIST, grfhlnf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NavigateToPidl)(::windows::core::Vtable::as_raw(self), pidl, grfhlnf).ok()
    }
    pub unsafe fn SetNavigateState(&self, bnstate: BNSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetNavigateState)(::windows::core::Vtable::as_raw(self), bnstate).ok()
    }
    pub unsafe fn GetNavigateState(&self) -> ::windows::core::Result<BNSTATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetNavigateState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn NotifyRedirect<P0>(&self, psv: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NotifyRedirect)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pidl, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UpdateWindowList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UpdateWindowList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UpdateBackForwardState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UpdateBackForwardState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, dwflags: u32, dwflagmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFlags)(::windows::core::Vtable::as_raw(self), dwflags, dwflagmask).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanNavigateNow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CanNavigateNow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetPidl(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPidl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetReferrer(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetReferrer)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
    pub unsafe fn GetBrowserIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBrowserIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBrowserByIndex(&self, dwid: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBrowserByIndex)(::windows::core::Vtable::as_raw(self), dwid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetHistoryObject(&self, ppole: *mut ::core::option::Option<super::super::System::Ole::IOleObject>, pstm: *mut ::core::option::Option<super::super::System::Com::IStream>, ppbc: *mut ::core::option::Option<super::super::System::Com::IBindCtx>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetHistoryObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppole), ::core::mem::transmute(pstm), ::core::mem::transmute(ppbc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHistoryObject<P0, P1>(&self, pole: P0, fislocalanchor: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetHistoryObject)(::windows::core::Vtable::as_raw(self), pole.into().abi(), fislocalanchor.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn CacheOLEServer<P0>(&self, pole: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IOleObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CacheOLEServer)(::windows::core::Vtable::as_raw(self), pole.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSetCodePage(&self, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSetCodePage)(::windows::core::Vtable::as_raw(self), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnHttpEquiv<P0, P1>(&self, psv: P0, fdone: P1, pvarargin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OnHttpEquiv)(::windows::core::Vtable::as_raw(self), psv.into().abi(), fdone.into(), pvarargin, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HPALETTE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPalette)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWindow<P0>(&self, fforceregister: P0, swc: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterWindow)(::windows::core::Vtable::as_raw(self), fforceregister.into(), swc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WndProcBS<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.WndProcBS)(::windows::core::Vtable::as_raw(self), hwnd.into(), umsg, wparam.into(), lparam.into())
    }
    pub unsafe fn SetAsDefFolderSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAsDefFolderSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetViewRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSize<P0>(&self, wparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnSize)(::windows::core::Vtable::as_raw(self), wparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnCreate(&self, pcs: *const super::WindowsAndMessaging::CREATESTRUCTW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnCreate)(::windows::core::Vtable::as_raw(self), pcs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCommand<P0, P1>(&self, wparam: P0, lparam: P1) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnCommand)(::windows::core::Vtable::as_raw(self), wparam.into(), lparam.into())
    }
    pub unsafe fn OnDestroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnDestroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn OnNotify(&self, pnm: *const super::Controls::NMHDR) -> super::super::Foundation::LRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.OnNotify)(::windows::core::Vtable::as_raw(self), pnm)
    }
    pub unsafe fn OnSetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnSetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFrameWindowActivateBS<P0>(&self, factive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnFrameWindowActivateBS)(::windows::core::Vtable::as_raw(self), factive.into()).ok()
    }
    pub unsafe fn ReleaseShellView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseShellView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ActivatePendingView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ActivatePendingView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateViewWindow<P0, P1>(&self, psvnew: P0, psvold: P1, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateViewWindow)(::windows::core::Vtable::as_raw(self), psvnew.into().abi(), psvold.into().abi(), prcview, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBrowserPropSheetExt(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBrowserPropSheetExt)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetViewWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn GetBaseBrowserData(&self) -> ::windows::core::Result<*mut BASEBROWSERDATALH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBaseBrowserData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn PutBaseBrowserData(&self) -> *mut BASEBROWSERDATALH {
        (::windows::core::Vtable::vtable(self).base__.base__.PutBaseBrowserData)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn InitializeTravelLog<P0>(&self, ptl: P0, dw: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITravelLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeTravelLog)(::windows::core::Vtable::as_raw(self), ptl.into().abi(), dw).ok()
    }
    pub unsafe fn SetTopBrowser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTopBrowser)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Offline(&self, icmd: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Offline)(::windows::core::Vtable::as_raw(self), icmd).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowViewResize<P0>(&self, f: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AllowViewResize)(::windows::core::Vtable::as_raw(self), f.into()).ok()
    }
    pub unsafe fn SetActivateState(&self, u: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetActivateState)(::windows::core::Vtable::as_raw(self), u).ok()
    }
    pub unsafe fn UpdateSecureLockIcon(&self, esecurelock: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateSecureLockIcon)(::windows::core::Vtable::as_raw(self), esecurelock).ok()
    }
    pub unsafe fn InitializeDownloadManager(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeDownloadManager)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InitializeTransitionSite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeTransitionSite)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _Initialize<P0, P1>(&self, hwnd: P0, pauto: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._Initialize)(::windows::core::Vtable::as_raw(self), hwnd.into(), pauto.into().abi()).ok()
    }
    pub unsafe fn _CancelPendingNavigationAsync(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._CancelPendingNavigationAsync)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _CancelPendingView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._CancelPendingView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _MaySaveChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._MaySaveChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _PauseOrResumeView<P0>(&self, fpaused: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._PauseOrResumeView)(::windows::core::Vtable::as_raw(self), fpaused.into()).ok()
    }
    pub unsafe fn _DisableModeless(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._DisableModeless)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn _NavigateToPidl2(&self, pidl: *const Common::ITEMIDLIST, grfhlnf: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._NavigateToPidl2)(::windows::core::Vtable::as_raw(self), pidl, grfhlnf, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn _TryShell2Rename<P0>(&self, psv: P0, pidlnew: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._TryShell2Rename)(::windows::core::Vtable::as_raw(self), psv.into().abi(), pidlnew).ok()
    }
    pub unsafe fn _SwitchActivationNow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._SwitchActivationNow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn _ExecChildren<P0, P1>(&self, punkbar: P0, fbroadcast: P1, pguidcmdgroup: ::core::option::Option<*const ::windows::core::GUID>, ncmdid: u32, ncmdexecopt: u32, pvarargin: ::core::option::Option<*const super::super::System::Com::VARIANT>, pvarargout: ::core::option::Option<*mut super::super::System::Com::VARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._ExecChildren)(::windows::core::Vtable::as_raw(self), punkbar.into().abi(), fbroadcast.into(), ::core::mem::transmute(pguidcmdgroup.unwrap_or(::std::ptr::null())), ncmdid, ncmdexecopt, ::core::mem::transmute(pvarargin.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvarargout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _SendChildren<P0, P1, P2, P3>(&self, hwndbar: P0, fbroadcast: P1, umsg: u32, wparam: P2, lparam: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P3: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._SendChildren)(::windows::core::Vtable::as_raw(self), hwndbar.into(), fbroadcast.into(), umsg, wparam.into(), lparam.into()).ok()
    }
    pub unsafe fn GetFolderSetData(&self, pfsd: *mut FOLDERSETDATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFolderSetData)(::windows::core::Vtable::as_raw(self), pfsd).ok()
    }
    pub unsafe fn _OnFocusChange(&self, itb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._OnFocusChange)(::windows::core::Vtable::as_raw(self), itb).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn v_ShowHideChildWindows<P0>(&self, fchildonly: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.v_ShowHideChildWindows)(::windows::core::Vtable::as_raw(self), fchildonly.into()).ok()
    }
    pub unsafe fn _get_itbLastFocus(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__._get_itbLastFocus)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn _put_itbLastFocus(&self, itblastfocus: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._put_itbLastFocus)(::windows::core::Vtable::as_raw(self), itblastfocus).ok()
    }
    pub unsafe fn _UIActivateView(&self, ustate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._UIActivateView)(::windows::core::Vtable::as_raw(self), ustate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _GetViewBorderRect(&self, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._GetViewBorderRect)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
    pub unsafe fn _UpdateViewRectSize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._UpdateViewRectSize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn _ResizeNextBorder(&self, itb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._ResizeNextBorder)(::windows::core::Vtable::as_raw(self), itb).ok()
    }
    pub unsafe fn _ResizeView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__._ResizeView)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn _GetEffectiveClientArea<P0>(&self, lprectborder: *mut super::super::Foundation::RECT, hmon: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HMONITOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._GetEffectiveClientArea)(::windows::core::Vtable::as_raw(self), lprectborder, hmon.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn v_GetViewStream<P0>(&self, pidl: *mut Common::ITEMIDLIST, grfmode: u32, pwszname: P0) -> ::core::option::Option<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.v_GetViewStream)(::windows::core::Vtable::as_raw(self), pidl, grfmode, pwszname.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ForwardViewMsg<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1) -> super::super::Foundation::LRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ForwardViewMsg)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into())
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetAcceleratorMenu<P0>(&self, hacc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HACCEL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAcceleratorMenu)(::windows::core::Vtable::as_raw(self), hacc.into()).ok()
    }
    pub unsafe fn _GetToolbarCount(&self) -> i32 {
        (::windows::core::Vtable::vtable(self).base__.base__._GetToolbarCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
    pub unsafe fn _GetToolbarItem(&self, itb: i32) -> *mut TOOLBARITEM {
        (::windows::core::Vtable::vtable(self).base__.base__._GetToolbarItem)(::windows::core::Vtable::as_raw(self), itb)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn _SaveToolbars<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._SaveToolbars)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn _LoadToolbars<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._LoadToolbars)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _CloseAndReleaseToolbars<P0>(&self, fclose: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._CloseAndReleaseToolbars)(::windows::core::Vtable::as_raw(self), fclose.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn v_MayGetNextToolbarFocus(&self, lpmsg: *const super::WindowsAndMessaging::MSG, itbnext: u32, citb: i32, pptbi: *mut *mut TOOLBARITEM, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.v_MayGetNextToolbarFocus)(::windows::core::Vtable::as_raw(self), lpmsg, itbnext, citb, pptbi, phwnd).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _ResizeNextBorderHelper<P0>(&self, itb: u32, busehmonitor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._ResizeNextBorderHelper)(::windows::core::Vtable::as_raw(self), itb, busehmonitor.into()).ok()
    }
    pub unsafe fn _FindTBar<P0>(&self, punksrc: P0) -> u32
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._FindTBar)(::windows::core::Vtable::as_raw(self), punksrc.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn _SetFocus<P0>(&self, ptbi: *const TOOLBARITEM, hwnd: P0, lpmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._SetFocus)(::windows::core::Vtable::as_raw(self), ptbi, hwnd.into(), lpmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn v_MayTranslateAccelerator(&self, pmsg: *mut super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.v_MayTranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _GetBorderDWHelper<P0, P1>(&self, punksrc: P0, lprectborder: *mut super::super::Foundation::RECT, busehmonitor: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__._GetBorderDWHelper)(::windows::core::Vtable::as_raw(self), punksrc.into().abi(), lprectborder, busehmonitor.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn v_CheckZoneCrossing(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.v_CheckZoneCrossing)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn _PositionViewWindow<P0>(&self, hwnd: P0, prc: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__._PositionViewWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn IEParseDisplayNameEx<P0>(&self, uicp: u32, pwszpath: P0, dwflags: u32) -> ::windows::core::Result<*mut Common::ITEMIDLIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IEParseDisplayNameEx)(::windows::core::Vtable::as_raw(self), uicp, pwszpath.into().abi(), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ICDBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICDBurn {}
impl ::core::fmt::Debug for ICDBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICDBurn").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICDBurnExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICDBurnExt {}
impl ::core::fmt::Debug for ICDBurnExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICDBurnExt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICategorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICategorizer {}
impl ::core::fmt::Debug for ICategorizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICategorizer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICategoryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICategoryProvider {}
impl ::core::fmt::Debug for ICategoryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICategoryProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnManager {}
impl ::core::fmt::Debug for IColumnManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnProvider {}
impl ::core::fmt::Debug for IColumnProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommDlgBrowser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommDlgBrowser {}
impl ::core::fmt::Debug for ICommDlgBrowser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommDlgBrowser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommDlgBrowser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommDlgBrowser2 {}
impl ::core::fmt::Debug for ICommDlgBrowser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommDlgBrowser2").field(&self.0).finish()
    }
}
impl ICommDlgBrowser2 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn OnDefaultCommand<P0>(&self, ppshv: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnDefaultCommand)(::windows::core::Vtable::as_raw(self), ppshv.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn OnStateChange<P0>(&self, ppshv: P0, uchange: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnStateChange)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), uchange).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn IncludeObject<P0>(&self, ppshv: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IncludeObject)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), pidl).ok()
    }
}
impl ::core::cmp::PartialEq for ICommDlgBrowser3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommDlgBrowser3 {}
impl ::core::fmt::Debug for ICommDlgBrowser3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommDlgBrowser3").field(&self.0).finish()
    }
}
impl ICommDlgBrowser3 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn OnDefaultCommand<P0>(&self, ppshv: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnDefaultCommand)(::windows::core::Vtable::as_raw(self), ppshv.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn OnStateChange<P0>(&self, ppshv: P0, uchange: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnStateChange)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), uchange).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn IncludeObject<P0>(&self, ppshv: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.IncludeObject)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), pidl).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Notify<P0>(&self, ppshv: P0, dwnotifytype: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Notify)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), dwnotifytype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetDefaultMenuText<P0>(&self, ppshv: P0, psztext: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultMenuText)(::windows::core::Vtable::as_raw(self), ppshv.into().abi(), ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _).ok()
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IComputerInfoChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComputerInfoChangeNotify {}
impl ::core::fmt::Debug for IComputerInfoChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComputerInfoChangeNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IConnectableCredentialProviderCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectableCredentialProviderCredential {}
impl ::core::fmt::Debug for IConnectableCredentialProviderCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectableCredentialProviderCredential").field(&self.0).finish()
    }
}
impl IConnectableCredentialProviderCredential {
    pub unsafe fn Advise<P0>(&self, pcpce: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredentialEvents>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pcpce.into().abi()).ok()
    }
    pub unsafe fn UnAdvise(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnAdvise)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SetSelected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDeselected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDeselected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetFieldState(&self, dwfieldid: u32, pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE, pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFieldState)(::windows::core::Vtable::as_raw(self), dwfieldid, pcpfs, pcpfis).ok()
    }
    pub unsafe fn GetStringValue(&self, dwfieldid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmapValue(&self, dwfieldid: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitmapValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCheckboxValue(&self, dwfieldid: u32, pbchecked: *mut super::super::Foundation::BOOL, ppszlabel: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCheckboxValue)(::windows::core::Vtable::as_raw(self), dwfieldid, pbchecked, ppszlabel).ok()
    }
    pub unsafe fn GetSubmitButtonValue(&self, dwfieldid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSubmitButtonValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetComboBoxValueCount(&self, dwfieldid: u32, pcitems: *mut u32, pdwselecteditem: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetComboBoxValueCount)(::windows::core::Vtable::as_raw(self), dwfieldid, pcitems, pdwselecteditem).ok()
    }
    pub unsafe fn GetComboBoxValueAt(&self, dwfieldid: u32, dwitem: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComboBoxValueAt)(::windows::core::Vtable::as_raw(self), dwfieldid, dwitem, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStringValue<P0>(&self, dwfieldid: u32, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStringValue)(::windows::core::Vtable::as_raw(self), dwfieldid, psz.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCheckboxValue<P0>(&self, dwfieldid: u32, bchecked: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCheckboxValue)(::windows::core::Vtable::as_raw(self), dwfieldid, bchecked.into()).ok()
    }
    pub unsafe fn SetComboBoxSelectedValue(&self, dwfieldid: u32, dwselecteditem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetComboBoxSelectedValue)(::windows::core::Vtable::as_raw(self), dwfieldid, dwselecteditem).ok()
    }
    pub unsafe fn CommandLinkClicked(&self, dwfieldid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CommandLinkClicked)(::windows::core::Vtable::as_raw(self), dwfieldid).ok()
    }
    pub unsafe fn GetSerialization(&self, pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, ppszoptionalstatustext: *mut ::windows::core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSerialization)(::windows::core::Vtable::as_raw(self), pcpgsr, pcpcs, ppszoptionalstatustext, pcpsioptionalstatusicon).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportResult<P0, P1>(&self, ntsstatus: P0, ntssubstatus: P1, ppszoptionalstatustext: *mut ::windows::core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::NTSTATUS>,
        P1: ::std::convert::Into<super::super::Foundation::NTSTATUS>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReportResult)(::windows::core::Vtable::as_raw(self), ntsstatus.into(), ntssubstatus.into(), ppszoptionalstatustext, pcpsioptionalstatusicon).ok()
    }
}
impl ::core::cmp::PartialEq for IContactManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactManagerInterop {}
impl ::core::fmt::Debug for IContactManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactManagerInterop").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenu {}
impl ::core::fmt::Debug for IContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenu").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContextMenu2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenu2 {}
impl ::core::fmt::Debug for IContextMenu2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenu2").field(&self.0).finish()
    }
}
impl IContextMenu2 {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn QueryContextMenu<P0>(&self, hmenu: P0, indexmenu: u32, idcmdfirst: u32, idcmdlast: u32, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).base__.QueryContextMenu)(::windows::core::Vtable::as_raw(self), hmenu.into(), indexmenu, idcmdfirst, idcmdlast, uflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeCommand(&self, pici: *const CMINVOKECOMMANDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvokeCommand)(::windows::core::Vtable::as_raw(self), pici).ok()
    }
    pub unsafe fn GetCommandString(&self, idcmd: usize, utype: u32, preserved: ::core::option::Option<*mut u32>, pszname: ::windows::core::PSTR, cchmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCommandString)(::windows::core::Vtable::as_raw(self), idcmd, utype, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pszname), cchmax).ok()
    }
}
impl ::core::cmp::PartialEq for IContextMenu3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenu3 {}
impl ::core::fmt::Debug for IContextMenu3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenu3").field(&self.0).finish()
    }
}
impl IContextMenu3 {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn QueryContextMenu<P0>(&self, hmenu: P0, indexmenu: u32, idcmdfirst: u32, idcmdlast: u32, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.QueryContextMenu)(::windows::core::Vtable::as_raw(self), hmenu.into(), indexmenu, idcmdfirst, idcmdlast, uflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeCommand(&self, pici: *const CMINVOKECOMMANDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InvokeCommand)(::windows::core::Vtable::as_raw(self), pici).ok()
    }
    pub unsafe fn GetCommandString(&self, idcmd: usize, utype: u32, preserved: ::core::option::Option<*mut u32>, pszname: ::windows::core::PSTR, cchmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCommandString)(::windows::core::Vtable::as_raw(self), idcmd, utype, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pszname), cchmax).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandleMenuMsg<P0, P1>(&self, umsg: u32, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.HandleMenuMsg)(::windows::core::Vtable::as_raw(self), umsg, wparam.into(), lparam.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IContextMenuCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuCB {}
impl ::core::fmt::Debug for IContextMenuCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuCB").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContextMenuSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuSite {}
impl ::core::fmt::Debug for IContextMenuSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICopyHookA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICopyHookA {}
impl ::core::fmt::Debug for ICopyHookA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICopyHookA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICopyHookW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICopyHookW {}
impl ::core::fmt::Debug for ICopyHookW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICopyHookW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateProcessInputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateProcessInputs {}
impl ::core::fmt::Debug for ICreateProcessInputs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateProcessInputs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreatingProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreatingProcess {}
impl ::core::fmt::Debug for ICreatingProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreatingProcess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProvider {}
impl ::core::fmt::Debug for ICredentialProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderCredential {}
impl ::core::fmt::Debug for ICredentialProviderCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderCredential").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderCredential2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderCredential2 {}
impl ::core::fmt::Debug for ICredentialProviderCredential2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderCredential2").field(&self.0).finish()
    }
}
impl ICredentialProviderCredential2 {
    pub unsafe fn Advise<P0>(&self, pcpce: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredentialEvents>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pcpce.into().abi()).ok()
    }
    pub unsafe fn UnAdvise(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnAdvise)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SetSelected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDeselected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDeselected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetFieldState(&self, dwfieldid: u32, pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE, pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFieldState)(::windows::core::Vtable::as_raw(self), dwfieldid, pcpfs, pcpfis).ok()
    }
    pub unsafe fn GetStringValue(&self, dwfieldid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmapValue(&self, dwfieldid: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitmapValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCheckboxValue(&self, dwfieldid: u32, pbchecked: *mut super::super::Foundation::BOOL, ppszlabel: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCheckboxValue)(::windows::core::Vtable::as_raw(self), dwfieldid, pbchecked, ppszlabel).ok()
    }
    pub unsafe fn GetSubmitButtonValue(&self, dwfieldid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSubmitButtonValue)(::windows::core::Vtable::as_raw(self), dwfieldid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetComboBoxValueCount(&self, dwfieldid: u32, pcitems: *mut u32, pdwselecteditem: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetComboBoxValueCount)(::windows::core::Vtable::as_raw(self), dwfieldid, pcitems, pdwselecteditem).ok()
    }
    pub unsafe fn GetComboBoxValueAt(&self, dwfieldid: u32, dwitem: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComboBoxValueAt)(::windows::core::Vtable::as_raw(self), dwfieldid, dwitem, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStringValue<P0>(&self, dwfieldid: u32, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStringValue)(::windows::core::Vtable::as_raw(self), dwfieldid, psz.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCheckboxValue<P0>(&self, dwfieldid: u32, bchecked: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCheckboxValue)(::windows::core::Vtable::as_raw(self), dwfieldid, bchecked.into()).ok()
    }
    pub unsafe fn SetComboBoxSelectedValue(&self, dwfieldid: u32, dwselecteditem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetComboBoxSelectedValue)(::windows::core::Vtable::as_raw(self), dwfieldid, dwselecteditem).ok()
    }
    pub unsafe fn CommandLinkClicked(&self, dwfieldid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CommandLinkClicked)(::windows::core::Vtable::as_raw(self), dwfieldid).ok()
    }
    pub unsafe fn GetSerialization(&self, pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, ppszoptionalstatustext: *mut ::windows::core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSerialization)(::windows::core::Vtable::as_raw(self), pcpgsr, pcpcs, ppszoptionalstatustext, pcpsioptionalstatusicon).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportResult<P0, P1>(&self, ntsstatus: P0, ntssubstatus: P1, ppszoptionalstatustext: *mut ::windows::core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::NTSTATUS>,
        P1: ::std::convert::Into<super::super::Foundation::NTSTATUS>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReportResult)(::windows::core::Vtable::as_raw(self), ntsstatus.into(), ntssubstatus.into(), ppszoptionalstatustext, pcpsioptionalstatusicon).ok()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderCredentialEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderCredentialEvents {}
impl ::core::fmt::Debug for ICredentialProviderCredentialEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderCredentialEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderCredentialEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderCredentialEvents2 {}
impl ::core::fmt::Debug for ICredentialProviderCredentialEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderCredentialEvents2").field(&self.0).finish()
    }
}
impl ICredentialProviderCredentialEvents2 {
    pub unsafe fn SetFieldState<P0>(&self, pcpc: P0, dwfieldid: u32, cpfs: CREDENTIAL_PROVIDER_FIELD_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldState)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, cpfs).ok()
    }
    pub unsafe fn SetFieldInteractiveState<P0>(&self, pcpc: P0, dwfieldid: u32, cpfis: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldInteractiveState)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, cpfis).ok()
    }
    pub unsafe fn SetFieldString<P0, P1>(&self, pcpc: P0, dwfieldid: u32, psz: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldString)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, psz.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFieldCheckbox<P0, P1, P2>(&self, pcpc: P0, dwfieldid: u32, bchecked: P1, pszlabel: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldCheckbox)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, bchecked.into(), pszlabel.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetFieldBitmap<P0, P1>(&self, pcpc: P0, dwfieldid: u32, hbmp: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldBitmap)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, hbmp.into()).ok()
    }
    pub unsafe fn SetFieldComboBoxSelectedItem<P0>(&self, pcpc: P0, dwfieldid: u32, dwselecteditem: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldComboBoxSelectedItem)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, dwselecteditem).ok()
    }
    pub unsafe fn DeleteFieldComboBoxItem<P0>(&self, pcpc: P0, dwfieldid: u32, dwitem: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteFieldComboBoxItem)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, dwitem).ok()
    }
    pub unsafe fn AppendFieldComboBoxItem<P0, P1>(&self, pcpc: P0, dwfieldid: u32, pszitem: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AppendFieldComboBoxItem)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, pszitem.into().abi()).ok()
    }
    pub unsafe fn SetFieldSubmitButton<P0>(&self, pcpc: P0, dwfieldid: u32, dwadjacentto: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICredentialProviderCredential>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFieldSubmitButton)(::windows::core::Vtable::as_raw(self), pcpc.into().abi(), dwfieldid, dwadjacentto).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCreatingWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnCreatingWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderCredentialWithFieldOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderCredentialWithFieldOptions {}
impl ::core::fmt::Debug for ICredentialProviderCredentialWithFieldOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderCredentialWithFieldOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderEvents {}
impl ::core::fmt::Debug for ICredentialProviderEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderFilter {}
impl ::core::fmt::Debug for ICredentialProviderFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderSetUserArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderSetUserArray {}
impl ::core::fmt::Debug for ICredentialProviderSetUserArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderSetUserArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderUser {}
impl ::core::fmt::Debug for ICredentialProviderUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICredentialProviderUserArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICredentialProviderUserArray {}
impl ::core::fmt::Debug for ICredentialProviderUserArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICredentialProviderUserArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICurrentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICurrentItem {}
impl ::core::fmt::Debug for ICurrentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICurrentItem").field(&self.0).finish()
    }
}
impl ICurrentItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ICurrentWorkingDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICurrentWorkingDirectory {}
impl ::core::fmt::Debug for ICurrentWorkingDirectory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICurrentWorkingDirectory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICustomDestinationList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomDestinationList {}
impl ::core::fmt::Debug for ICustomDestinationList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomDestinationList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataObjectAsyncCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObjectAsyncCapability {}
impl ::core::fmt::Debug for IDataObjectAsyncCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObjectAsyncCapability").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObjectProvider {}
impl ::core::fmt::Debug for IDataObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObjectProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataTransferManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataTransferManagerInterop {}
impl ::core::fmt::Debug for IDataTransferManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataTransferManagerInterop").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDefaultExtractIconInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultExtractIconInit {}
impl ::core::fmt::Debug for IDefaultExtractIconInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultExtractIconInit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDefaultFolderMenuInitialize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultFolderMenuInitialize {}
impl ::core::fmt::Debug for IDefaultFolderMenuInitialize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultFolderMenuInitialize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDelegateFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDelegateFolder {}
impl ::core::fmt::Debug for IDelegateFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDelegateFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDelegateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDelegateItem {}
impl ::core::fmt::Debug for IDelegateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDelegateItem").field(&self.0).finish()
    }
}
impl IDelegateItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDeskBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDeskBand {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDeskBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeskBand").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDeskBand {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowDW<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowDW)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn CloseDW(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CloseDW)(::windows::core::Vtable::as_raw(self), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResizeBorderDW<P0, P1>(&self, prcborder: *const super::super::Foundation::RECT, punktoolbarsite: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResizeBorderDW)(::windows::core::Vtable::as_raw(self), prcborder, punktoolbarsite.into().abi(), freserved.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDeskBand2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDeskBand2 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDeskBand2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeskBand2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDeskBand2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowDW<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ShowDW)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn CloseDW(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CloseDW)(::windows::core::Vtable::as_raw(self), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResizeBorderDW<P0, P1>(&self, prcborder: *const super::super::Foundation::RECT, punktoolbarsite: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResizeBorderDW)(::windows::core::Vtable::as_raw(self), prcborder, punktoolbarsite.into().abi(), freserved.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBandInfo(&self, dwbandid: u32, dwviewmode: u32, pdbi: *mut DESKBANDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBandInfo)(::windows::core::Vtable::as_raw(self), dwbandid, dwviewmode, pdbi).ok()
    }
}
impl ::core::cmp::PartialEq for IDeskBandInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeskBandInfo {}
impl ::core::fmt::Debug for IDeskBandInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeskBandInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDeskBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDeskBar {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDeskBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeskBar").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDeskBar {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDeskBarClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDeskBarClient {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDeskBarClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeskBarClient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDeskBarClient {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IDesktopGadget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopGadget {}
impl ::core::fmt::Debug for IDesktopGadget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopGadget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDesktopWallpaper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWallpaper {}
impl ::core::fmt::Debug for IDesktopWallpaper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWallpaper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDestinationStreamFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDestinationStreamFactory {}
impl ::core::fmt::Debug for IDestinationStreamFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDestinationStreamFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDisplayItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayItem {}
impl ::core::fmt::Debug for IDisplayItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayItem").field(&self.0).finish()
    }
}
impl IDisplayItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDocViewSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDocViewSite {}
impl ::core::fmt::Debug for IDocViewSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDocViewSite").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDockingWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDockingWindow {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDockingWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDockingWindow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDockingWindow {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDockingWindowFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDockingWindowFrame {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDockingWindowFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDockingWindowFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDockingWindowFrame {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IDockingWindowSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IDockingWindowSite {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IDockingWindowSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDockingWindowSite").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IDockingWindowSite {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IDragSourceHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragSourceHelper {}
impl ::core::fmt::Debug for IDragSourceHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragSourceHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDragSourceHelper2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragSourceHelper2 {}
impl ::core::fmt::Debug for IDragSourceHelper2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragSourceHelper2").field(&self.0).finish()
    }
}
impl IDragSourceHelper2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn InitializeFromBitmap<P0>(&self, pshdi: *const SHDRAGIMAGE, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromBitmap)(::windows::core::Vtable::as_raw(self), pshdi, pdataobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InitializeFromWindow<P0, P1>(&self, hwnd: P0, ppt: ::core::option::Option<*const super::super::Foundation::POINT>, pdataobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), ::core::mem::transmute(ppt.unwrap_or(::std::ptr::null())), pdataobject.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDropTargetHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropTargetHelper {}
impl ::core::fmt::Debug for IDropTargetHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropTargetHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDynamicHWHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicHWHandler {}
impl ::core::fmt::Debug for IDynamicHWHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicHWHandler").field(&self.0).finish()
    }
}
impl ::core::default::Default for IEPDNFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IEPDNFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEPDNFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IESHORTCUTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IESHORTCUTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IESHORTCUTFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumACString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumACString {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumACString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumACString").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumACString {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::PWSTR], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Skip)(::windows::core::Vtable::as_raw(self), celt)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IEnumAssocHandlers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumAssocHandlers {}
impl ::core::fmt::Debug for IEnumAssocHandlers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumAssocHandlers").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumExplorerCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumExplorerCommand {}
impl ::core::fmt::Debug for IEnumExplorerCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumExplorerCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumExtraSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumExtraSearch {}
impl ::core::fmt::Debug for IEnumExtraSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumExtraSearch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumFullIDList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFullIDList {}
impl ::core::fmt::Debug for IEnumFullIDList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFullIDList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumHLITEM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumHLITEM {}
impl ::core::fmt::Debug for IEnumHLITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumHLITEM").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumIDList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumIDList {}
impl ::core::fmt::Debug for IEnumIDList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumIDList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumObjects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumObjects {}
impl ::core::fmt::Debug for IEnumObjects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumObjects").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPublishedApps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPublishedApps {}
impl ::core::fmt::Debug for IEnumPublishedApps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPublishedApps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumReadyCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumReadyCallback {}
impl ::core::fmt::Debug for IEnumReadyCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumReadyCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumResources {}
impl ::core::fmt::Debug for IEnumResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumResources").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumShellItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumShellItems {}
impl ::core::fmt::Debug for IEnumShellItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumShellItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSyncMgrConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncMgrConflict {}
impl ::core::fmt::Debug for IEnumSyncMgrConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncMgrConflict").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSyncMgrEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncMgrEvents {}
impl ::core::fmt::Debug for IEnumSyncMgrEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncMgrEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSyncMgrSyncItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncMgrSyncItems {}
impl ::core::fmt::Debug for IEnumSyncMgrSyncItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncMgrSyncItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTravelLogEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTravelLogEntry {}
impl ::core::fmt::Debug for IEnumTravelLogEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTravelLogEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumerableView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumerableView {}
impl ::core::fmt::Debug for IEnumerableView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumerableView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExecuteCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExecuteCommand {}
impl ::core::fmt::Debug for IExecuteCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecuteCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExecuteCommandApplicationHostEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExecuteCommandApplicationHostEnvironment {}
impl ::core::fmt::Debug for IExecuteCommandApplicationHostEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecuteCommandApplicationHostEnvironment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExecuteCommandHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExecuteCommandHost {}
impl ::core::fmt::Debug for IExecuteCommandHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecuteCommandHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExpDispSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpDispSupport {}
impl ::core::fmt::Debug for IExpDispSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpDispSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExpDispSupportXP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpDispSupportXP {}
impl ::core::fmt::Debug for IExpDispSupportXP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpDispSupportXP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerBrowser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerBrowser {}
impl ::core::fmt::Debug for IExplorerBrowser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerBrowser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerBrowserEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerBrowserEvents {}
impl ::core::fmt::Debug for IExplorerBrowserEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerBrowserEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerCommand {}
impl ::core::fmt::Debug for IExplorerCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerCommandProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerCommandProvider {}
impl ::core::fmt::Debug for IExplorerCommandProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerCommandProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerCommandState {}
impl ::core::fmt::Debug for IExplorerCommandState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerCommandState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExplorerPaneVisibility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExplorerPaneVisibility {}
impl ::core::fmt::Debug for IExplorerPaneVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExplorerPaneVisibility").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExtensionServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtensionServices {}
impl ::core::fmt::Debug for IExtensionServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtensionServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExtractIconA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtractIconA {}
impl ::core::fmt::Debug for IExtractIconA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtractIconA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExtractIconW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtractIconW {}
impl ::core::fmt::Debug for IExtractIconW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtractIconW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExtractImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtractImage {}
impl ::core::fmt::Debug for IExtractImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtractImage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExtractImage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtractImage2 {}
impl ::core::fmt::Debug for IExtractImage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtractImage2").field(&self.0).finish()
    }
}
impl IExtractImage2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocation(&self, pszpathbuffer: &mut [u16], pdwpriority: *mut u32, prgsize: *const super::super::Foundation::SIZE, dwrecclrdepth: u32, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszpathbuffer.as_ptr()), pszpathbuffer.len() as _, pdwpriority, prgsize, dwrecclrdepth, pdwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Extract(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Extract)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IFileDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileDialog {}
impl ::core::fmt::Debug for IFileDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileDialog").field(&self.0).finish()
    }
}
impl IFileDialog {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IFileDialog2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileDialog2 {}
impl ::core::fmt::Debug for IFileDialog2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileDialog2").field(&self.0).finish()
    }
}
impl IFileDialog2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Show)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetFileTypes(&self, rgfilterspec: &[Common::COMDLG_FILTERSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypes)(::windows::core::Vtable::as_raw(self), rgfilterspec.len() as _, ::core::mem::transmute(rgfilterspec.as_ptr())).ok()
    }
    pub unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypeIndex)(::windows::core::Vtable::as_raw(self), ifiletype).ok()
    }
    pub unsafe fn GetFileTypeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileTypeIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, pfde: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFileDialogEvents>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pfde.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOptions)(::windows::core::Vtable::as_raw(self), fos).ok()
    }
    pub unsafe fn GetOptions(&self) -> ::windows::core::Result<FILEOPENDIALOGOPTIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn SetFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn GetFolder(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileName<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileName)(::windows::core::Vtable::as_raw(self), pszname.into().abi()).ok()
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTitle)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    pub unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOkButtonLabel)(::windows::core::Vtable::as_raw(self), psztext.into().abi()).ok()
    }
    pub unsafe fn SetFileNameLabel<P0>(&self, pszlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileNameLabel)(::windows::core::Vtable::as_raw(self), pszlabel.into().abi()).ok()
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddPlace<P0>(&self, psi: P0, fdap: FDAP) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPlace)(::windows::core::Vtable::as_raw(self), psi.into().abi(), fdap).ok()
    }
    pub unsafe fn SetDefaultExtension<P0>(&self, pszdefaultextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultExtension)(::windows::core::Vtable::as_raw(self), pszdefaultextension.into().abi()).ok()
    }
    pub unsafe fn Close(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn SetClientGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn ClearClientData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearClientData)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFilter<P0>(&self, pfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItemFilter>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFilter)(::windows::core::Vtable::as_raw(self), pfilter.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IFileDialogControlEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileDialogControlEvents {}
impl ::core::fmt::Debug for IFileDialogControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileDialogControlEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileDialogCustomize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileDialogCustomize {}
impl ::core::fmt::Debug for IFileDialogCustomize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileDialogCustomize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileDialogEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileDialogEvents {}
impl ::core::fmt::Debug for IFileDialogEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileDialogEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileIsInUse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileIsInUse {}
impl ::core::fmt::Debug for IFileIsInUse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileIsInUse").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileOpenDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenDialog {}
impl ::core::fmt::Debug for IFileOpenDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenDialog").field(&self.0).finish()
    }
}
impl IFileOpenDialog {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Show)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetFileTypes(&self, rgfilterspec: &[Common::COMDLG_FILTERSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypes)(::windows::core::Vtable::as_raw(self), rgfilterspec.len() as _, ::core::mem::transmute(rgfilterspec.as_ptr())).ok()
    }
    pub unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypeIndex)(::windows::core::Vtable::as_raw(self), ifiletype).ok()
    }
    pub unsafe fn GetFileTypeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileTypeIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, pfde: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFileDialogEvents>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pfde.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOptions)(::windows::core::Vtable::as_raw(self), fos).ok()
    }
    pub unsafe fn GetOptions(&self) -> ::windows::core::Result<FILEOPENDIALOGOPTIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn SetFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn GetFolder(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileName<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileName)(::windows::core::Vtable::as_raw(self), pszname.into().abi()).ok()
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTitle)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    pub unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOkButtonLabel)(::windows::core::Vtable::as_raw(self), psztext.into().abi()).ok()
    }
    pub unsafe fn SetFileNameLabel<P0>(&self, pszlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileNameLabel)(::windows::core::Vtable::as_raw(self), pszlabel.into().abi()).ok()
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddPlace<P0>(&self, psi: P0, fdap: FDAP) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPlace)(::windows::core::Vtable::as_raw(self), psi.into().abi(), fdap).ok()
    }
    pub unsafe fn SetDefaultExtension<P0>(&self, pszdefaultextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultExtension)(::windows::core::Vtable::as_raw(self), pszdefaultextension.into().abi()).ok()
    }
    pub unsafe fn Close(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn SetClientGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn ClearClientData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearClientData)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFilter<P0>(&self, pfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItemFilter>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFilter)(::windows::core::Vtable::as_raw(self), pfilter.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IFileOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOperation {}
impl ::core::fmt::Debug for IFileOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileOperation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOperation2 {}
impl ::core::fmt::Debug for IFileOperation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOperation2").field(&self.0).finish()
    }
}
impl IFileOperation2 {
    pub unsafe fn Advise<P0>(&self, pfops: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pfops.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn SetOperationFlags(&self, dwoperationflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOperationFlags)(::windows::core::Vtable::as_raw(self), dwoperationflags).ok()
    }
    pub unsafe fn SetProgressMessage<P0>(&self, pszmessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProgressMessage)(::windows::core::Vtable::as_raw(self), pszmessage.into().abi()).ok()
    }
    pub unsafe fn SetProgressDialog<P0>(&self, popd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOperationsProgressDialog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProgressDialog)(::windows::core::Vtable::as_raw(self), popd.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn SetProperties<P0>(&self, pproparray: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<PropertiesSystem::IPropertyChangeArray>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProperties)(::windows::core::Vtable::as_raw(self), pproparray.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerWindow<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOwnerWindow)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    pub unsafe fn ApplyPropertiesToItem<P0>(&self, psiitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ApplyPropertiesToItem)(::windows::core::Vtable::as_raw(self), psiitem.into().abi()).ok()
    }
    pub unsafe fn ApplyPropertiesToItems<P0>(&self, punkitems: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ApplyPropertiesToItems)(::windows::core::Vtable::as_raw(self), punkitems.into().abi()).ok()
    }
    pub unsafe fn RenameItem<P0, P1, P2>(&self, psiitem: P0, psznewname: P1, pfopsitem: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RenameItem)(::windows::core::Vtable::as_raw(self), psiitem.into().abi(), psznewname.into().abi(), pfopsitem.into().abi()).ok()
    }
    pub unsafe fn RenameItems<P0, P1>(&self, punkitems: P0, psznewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RenameItems)(::windows::core::Vtable::as_raw(self), punkitems.into().abi(), psznewname.into().abi()).ok()
    }
    pub unsafe fn MoveItem<P0, P1, P2, P3>(&self, psiitem: P0, psidestinationfolder: P1, psznewname: P2, pfopsitem: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MoveItem)(::windows::core::Vtable::as_raw(self), psiitem.into().abi(), psidestinationfolder.into().abi(), psznewname.into().abi(), pfopsitem.into().abi()).ok()
    }
    pub unsafe fn MoveItems<P0, P1>(&self, punkitems: P0, psidestinationfolder: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MoveItems)(::windows::core::Vtable::as_raw(self), punkitems.into().abi(), psidestinationfolder.into().abi()).ok()
    }
    pub unsafe fn CopyItem<P0, P1, P2, P3>(&self, psiitem: P0, psidestinationfolder: P1, pszcopyname: P2, pfopsitem: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyItem)(::windows::core::Vtable::as_raw(self), psiitem.into().abi(), psidestinationfolder.into().abi(), pszcopyname.into().abi(), pfopsitem.into().abi()).ok()
    }
    pub unsafe fn CopyItems<P0, P1>(&self, punkitems: P0, psidestinationfolder: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyItems)(::windows::core::Vtable::as_raw(self), punkitems.into().abi(), psidestinationfolder.into().abi()).ok()
    }
    pub unsafe fn DeleteItem<P0, P1>(&self, psiitem: P0, pfopsitem: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), psiitem.into().abi(), pfopsitem.into().abi()).ok()
    }
    pub unsafe fn DeleteItems<P0>(&self, punkitems: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteItems)(::windows::core::Vtable::as_raw(self), punkitems.into().abi()).ok()
    }
    pub unsafe fn NewItem<P0, P1, P2, P3>(&self, psidestinationfolder: P0, dwfileattributes: u32, pszname: P1, psztemplatename: P2, pfopsitem: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IFileOperationProgressSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.NewItem)(::windows::core::Vtable::as_raw(self), psidestinationfolder.into().abi(), dwfileattributes, pszname.into().abi(), psztemplatename.into().abi(), pfopsitem.into().abi()).ok()
    }
    pub unsafe fn PerformOperations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PerformOperations)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyOperationsAborted(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAnyOperationsAborted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IFileOperationProgressSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOperationProgressSink {}
impl ::core::fmt::Debug for IFileOperationProgressSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOperationProgressSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileSaveDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSaveDialog {}
impl ::core::fmt::Debug for IFileSaveDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSaveDialog").field(&self.0).finish()
    }
}
impl IFileSaveDialog {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Show)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetFileTypes(&self, rgfilterspec: &[Common::COMDLG_FILTERSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypes)(::windows::core::Vtable::as_raw(self), rgfilterspec.len() as _, ::core::mem::transmute(rgfilterspec.as_ptr())).ok()
    }
    pub unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileTypeIndex)(::windows::core::Vtable::as_raw(self), ifiletype).ok()
    }
    pub unsafe fn GetFileTypeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileTypeIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, pfde: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFileDialogEvents>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Advise)(::windows::core::Vtable::as_raw(self), pfde.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOptions)(::windows::core::Vtable::as_raw(self), fos).ok()
    }
    pub unsafe fn GetOptions(&self) -> ::windows::core::Result<FILEOPENDIALOGOPTIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn SetFolder<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFolder)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn GetFolder(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileName<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileName)(::windows::core::Vtable::as_raw(self), pszname.into().abi()).ok()
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTitle)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    pub unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOkButtonLabel)(::windows::core::Vtable::as_raw(self), psztext.into().abi()).ok()
    }
    pub unsafe fn SetFileNameLabel<P0>(&self, pszlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFileNameLabel)(::windows::core::Vtable::as_raw(self), pszlabel.into().abi()).ok()
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddPlace<P0>(&self, psi: P0, fdap: FDAP) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPlace)(::windows::core::Vtable::as_raw(self), psi.into().abi(), fdap).ok()
    }
    pub unsafe fn SetDefaultExtension<P0>(&self, pszdefaultextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultExtension)(::windows::core::Vtable::as_raw(self), pszdefaultextension.into().abi()).ok()
    }
    pub unsafe fn Close(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn SetClientGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn ClearClientData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearClientData)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetFilter<P0>(&self, pfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItemFilter>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFilter)(::windows::core::Vtable::as_raw(self), pfilter.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSearchBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSearchBand {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSearchBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSearchBand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileSyncMergeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSyncMergeHandler {}
impl ::core::fmt::Debug for IFileSyncMergeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSyncMergeHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileSystemBindData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSystemBindData {}
impl ::core::fmt::Debug for IFileSystemBindData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemBindData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileSystemBindData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSystemBindData2 {}
impl ::core::fmt::Debug for IFileSystemBindData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemBindData2").field(&self.0).finish()
    }
}
impl IFileSystemBindData2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub unsafe fn SetFindData(&self, pfd: *const super::super::Storage::FileSystem::WIN32_FIND_DATAW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFindData)(::windows::core::Vtable::as_raw(self), pfd).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub unsafe fn GetFindData(&self, pfd: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFindData)(::windows::core::Vtable::as_raw(self), pfd).ok()
    }
}
impl ::core::cmp::PartialEq for IFolderBandPriv {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderBandPriv {}
impl ::core::fmt::Debug for IFolderBandPriv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderBandPriv").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderFilter {}
impl ::core::fmt::Debug for IFolderFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderFilterSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderFilterSite {}
impl ::core::fmt::Debug for IFolderFilterSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderFilterSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderView {}
impl ::core::fmt::Debug for IFolderView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderView2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderView2 {}
impl ::core::fmt::Debug for IFolderView2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderView2").field(&self.0).finish()
    }
}
impl IFolderView2 {
    pub unsafe fn GetCurrentViewMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentViewMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentViewMode(&self, viewmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentViewMode)(::windows::core::Vtable::as_raw(self), viewmode).ok()
    }
    pub unsafe fn GetFolder<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFolder)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn Item(&self, iitemindex: i32) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Item)(::windows::core::Vtable::as_raw(self), iitemindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self, uflags: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ItemCount)(::windows::core::Vtable::as_raw(self), uflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Items<T>(&self, uflags: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Items)(::windows::core::Vtable::as_raw(self), uflags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelectionMarkedItem(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelectionMarkedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedItem(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFocusedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn GetItemPosition(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<super::super::Foundation::POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemPosition)(::windows::core::Vtable::as_raw(self), pidl, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSpacing(&self, ppt: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpacing)(::windows::core::Vtable::as_raw(self), ppt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultSpacing(&self) -> ::windows::core::Result<super::super::Foundation::POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDefaultSpacing)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAutoArrange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAutoArrange)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SelectItem(&self, iitem: i32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectItem)(::windows::core::Vtable::as_raw(self), iitem, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn SelectAndPositionItems(&self, cidl: u32, apidl: *const *const Common::ITEMIDLIST, apt: ::core::option::Option<*const super::super::Foundation::POINT>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectAndPositionItems)(::windows::core::Vtable::as_raw(self), cidl, apidl, ::core::mem::transmute(apt.unwrap_or(::std::ptr::null())), dwflags).ok()
    }
}
impl ::core::cmp::PartialEq for IFolderViewHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderViewHost {}
impl ::core::fmt::Debug for IFolderViewHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderViewHost").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFolderViewOC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFolderViewOC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFolderViewOC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderViewOC").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderViewOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderViewOptions {}
impl ::core::fmt::Debug for IFolderViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderViewOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFolderViewSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderViewSettings {}
impl ::core::fmt::Debug for IFolderViewSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderViewSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFrameworkInputPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkInputPane {}
impl ::core::fmt::Debug for IFrameworkInputPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkInputPane").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFrameworkInputPaneHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkInputPaneHandler {}
impl ::core::fmt::Debug for IFrameworkInputPaneHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkInputPaneHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetServiceIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetServiceIds {}
impl ::core::fmt::Debug for IGetServiceIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetServiceIds").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHWEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHWEventHandler {}
impl ::core::fmt::Debug for IHWEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHWEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHWEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHWEventHandler2 {}
impl ::core::fmt::Debug for IHWEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHWEventHandler2").field(&self.0).finish()
    }
}
impl IHWEventHandler2 {
    pub unsafe fn Initialize<P0>(&self, pszparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pszparams.into().abi()).ok()
    }
    pub unsafe fn HandleEvent<P0, P1, P2>(&self, pszdeviceid: P0, pszaltdeviceid: P1, pszeventtype: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.HandleEvent)(::windows::core::Vtable::as_raw(self), pszdeviceid.into().abi(), pszaltdeviceid.into().abi(), pszeventtype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleEventWithContent<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pszaltdeviceid: P1, pszeventtype: P2, pszcontenttypehandler: P3, pdataobject: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.HandleEventWithContent)(::windows::core::Vtable::as_raw(self), pszdeviceid.into().abi(), pszaltdeviceid.into().abi(), pszeventtype.into().abi(), pszcontenttypehandler.into().abi(), pdataobject.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IHandlerActivationHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHandlerActivationHost {}
impl ::core::fmt::Debug for IHandlerActivationHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHandlerActivationHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHandlerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHandlerInfo {}
impl ::core::fmt::Debug for IHandlerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHandlerInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHandlerInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHandlerInfo2 {}
impl ::core::fmt::Debug for IHandlerInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHandlerInfo2").field(&self.0).finish()
    }
}
impl IHandlerInfo2 {
    pub unsafe fn GetApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetApplicationDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplicationPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetApplicationPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplicationIconReference(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetApplicationIconReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IHlink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHlink {}
impl ::core::fmt::Debug for IHlink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHlink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHlinkBrowseContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHlinkBrowseContext {}
impl ::core::fmt::Debug for IHlinkBrowseContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHlinkBrowseContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHlinkFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHlinkFrame {}
impl ::core::fmt::Debug for IHlinkFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHlinkFrame").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHlinkSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHlinkSite {}
impl ::core::fmt::Debug for IHlinkSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHlinkSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHlinkTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHlinkTarget {}
impl ::core::fmt::Debug for IHlinkTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHlinkTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHomeGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHomeGroup {}
impl ::core::fmt::Debug for IHomeGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHomeGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IIOCancelInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIOCancelInformation {}
impl ::core::fmt::Debug for IIOCancelInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIOCancelInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IIdentityName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIdentityName {}
impl ::core::fmt::Debug for IIdentityName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdentityName").field(&self.0).finish()
    }
}
impl IIdentityName {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IImageRecompress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageRecompress {}
impl ::core::fmt::Debug for IImageRecompress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageRecompress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeCommand {}
impl ::core::fmt::Debug for IInitializeCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeNetworkFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeNetworkFolder {}
impl ::core::fmt::Debug for IInitializeNetworkFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeNetworkFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeObject {}
impl ::core::fmt::Debug for IInitializeObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeWithBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithBindCtx {}
impl ::core::fmt::Debug for IInitializeWithBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithBindCtx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeWithItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithItem {}
impl ::core::fmt::Debug for IInitializeWithItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeWithPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithPropertyStore {}
impl ::core::fmt::Debug for IInitializeWithPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithPropertyStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeWithWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithWindow {}
impl ::core::fmt::Debug for IInitializeWithWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputObject {}
impl ::core::fmt::Debug for IInputObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputObject2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputObject2 {}
impl ::core::fmt::Debug for IInputObject2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputObject2").field(&self.0).finish()
    }
}
impl IInputObject2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn UIActivateIO<P0>(&self, factivate: P0, pmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.UIActivateIO)(::windows::core::Vtable::as_raw(self), factivate.into(), pmsg).ok()
    }
    pub unsafe fn HasFocusIO(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HasFocusIO)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAcceleratorIO(&self, pmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TranslateAcceleratorIO)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
}
impl ::core::cmp::PartialEq for IInputObjectSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputObjectSite {}
impl ::core::fmt::Debug for IInputObjectSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputObjectSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputPaneAnimationCoordinator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPaneAnimationCoordinator {}
impl ::core::fmt::Debug for IInputPaneAnimationCoordinator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPaneAnimationCoordinator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputPanelConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPanelConfiguration {}
impl ::core::fmt::Debug for IInputPanelConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPanelConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputPanelInvocationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPanelInvocationConfiguration {}
impl ::core::fmt::Debug for IInputPanelInvocationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPanelInvocationConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInsertItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInsertItem {}
impl ::core::fmt::Debug for IInsertItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInsertItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IItemNameLimits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemNameLimits {}
impl ::core::fmt::Debug for IItemNameLimits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemNameLimits").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKnownFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKnownFolder {}
impl ::core::fmt::Debug for IKnownFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKnownFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKnownFolderManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKnownFolderManager {}
impl ::core::fmt::Debug for IKnownFolderManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKnownFolderManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchSourceAppUserModelId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchSourceAppUserModelId {}
impl ::core::fmt::Debug for ILaunchSourceAppUserModelId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchSourceAppUserModelId").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchSourceViewSizePreference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchSourceViewSizePreference {}
impl ::core::fmt::Debug for ILaunchSourceViewSizePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchSourceViewSizePreference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchTargetMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchTargetMonitor {}
impl ::core::fmt::Debug for ILaunchTargetMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchTargetMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchTargetViewSizePreference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchTargetViewSizePreference {}
impl ::core::fmt::Debug for ILaunchTargetViewSizePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchTargetViewSizePreference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchUIContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchUIContext {}
impl ::core::fmt::Debug for ILaunchUIContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchUIContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILaunchUIContextProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchUIContextProvider {}
impl ::core::fmt::Debug for ILaunchUIContextProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchUIContextProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMenuBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMenuBand {}
impl ::core::fmt::Debug for IMenuBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMenuBand").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IMenuPopup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IMenuPopup {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IMenuPopup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMenuPopup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IMenuPopup {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn SetClient<P0>(&self, punkclient: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClient)(::windows::core::Vtable::as_raw(self), punkclient.into().abi()).ok()
    }
    pub unsafe fn GetClient(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClient)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChangeDB(&self, prc: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnPosRectChangeDB)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
}
impl ::core::cmp::PartialEq for IModalWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IModalWindow {}
impl ::core::fmt::Debug for IModalWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IModalWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeAccessible {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeAccessible {}
impl ::core::fmt::Debug for INameSpaceTreeAccessible {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeAccessible").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControl {}
impl ::core::fmt::Debug for INameSpaceTreeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControl2 {}
impl ::core::fmt::Debug for INameSpaceTreeControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControl2").field(&self.0).finish()
    }
}
impl INameSpaceTreeControl2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, hwndparent: P0, prc: *const super::super::Foundation::RECT, nsctsflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), hwndparent.into(), prc, nsctsflags).ok()
    }
    pub unsafe fn TreeAdvise<P0>(&self, punk: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TreeAdvise)(::windows::core::Vtable::as_raw(self), punk.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TreeUnadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TreeUnadvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn AppendRoot<P0, P1>(&self, psiroot: P0, grfenumflags: u32, grfrootstyle: u32, pif: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItemFilter>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AppendRoot)(::windows::core::Vtable::as_raw(self), psiroot.into().abi(), grfenumflags, grfrootstyle, pif.into().abi()).ok()
    }
    pub unsafe fn InsertRoot<P0, P1>(&self, iindex: i32, psiroot: P0, grfenumflags: u32, grfrootstyle: u32, pif: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellItemFilter>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InsertRoot)(::windows::core::Vtable::as_raw(self), iindex, psiroot.into().abi(), grfenumflags, grfrootstyle, pif.into().abi()).ok()
    }
    pub unsafe fn RemoveRoot<P0>(&self, psiroot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveRoot)(::windows::core::Vtable::as_raw(self), psiroot.into().abi()).ok()
    }
    pub unsafe fn RemoveAllRoots(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllRoots)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetRootItems(&self) -> ::windows::core::Result<IShellItemArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetItemState<P0>(&self, psi: P0, nstcismask: u32, nstcisflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetItemState)(::windows::core::Vtable::as_raw(self), psi.into().abi(), nstcismask, nstcisflags).ok()
    }
    pub unsafe fn GetItemState<P0>(&self, psi: P0, nstcismask: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemState)(::windows::core::Vtable::as_raw(self), psi.into().abi(), nstcismask, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelectedItems(&self) -> ::windows::core::Result<IShellItemArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelectedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItemCustomState<P0>(&self, psi: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemCustomState)(::windows::core::Vtable::as_raw(self), psi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetItemCustomState<P0>(&self, psi: P0, istatenumber: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetItemCustomState)(::windows::core::Vtable::as_raw(self), psi.into().abi(), istatenumber).ok()
    }
    pub unsafe fn EnsureItemVisible<P0>(&self, psi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnsureItemVisible)(::windows::core::Vtable::as_raw(self), psi.into().abi()).ok()
    }
    pub unsafe fn SetTheme<P0>(&self, psztheme: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTheme)(::windows::core::Vtable::as_raw(self), psztheme.into().abi()).ok()
    }
    pub unsafe fn GetNextItem<P0>(&self, psi: P0, nstcgi: NSTCGNI) -> ::windows::core::Result<IShellItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNextItem)(::windows::core::Vtable::as_raw(self), psi.into().abi(), nstcgi, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTest(&self, ppt: *const super::super::Foundation::POINT) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HitTest)(::windows::core::Vtable::as_raw(self), ppt, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemRect<P0>(&self, psi: P0) -> ::windows::core::Result<super::super::Foundation::RECT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemRect)(::windows::core::Vtable::as_raw(self), psi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CollapseAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CollapseAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControlCustomDraw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControlCustomDraw {}
impl ::core::fmt::Debug for INameSpaceTreeControlCustomDraw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControlCustomDraw").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControlDropHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControlDropHandler {}
impl ::core::fmt::Debug for INameSpaceTreeControlDropHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControlDropHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControlEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControlEvents {}
impl ::core::fmt::Debug for INameSpaceTreeControlEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControlEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INameSpaceTreeControlFolderCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INameSpaceTreeControlFolderCapabilities {}
impl ::core::fmt::Debug for INameSpaceTreeControlFolderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INameSpaceTreeControlFolderCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamedPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedPropertyBag {}
impl ::core::fmt::Debug for INamedPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedPropertyBag").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamespaceWalk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamespaceWalk {}
impl ::core::fmt::Debug for INamespaceWalk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamespaceWalk").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamespaceWalkCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamespaceWalkCB {}
impl ::core::fmt::Debug for INamespaceWalkCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamespaceWalkCB").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamespaceWalkCB2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamespaceWalkCB2 {}
impl ::core::fmt::Debug for INamespaceWalkCB2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamespaceWalkCB2").field(&self.0).finish()
    }
}
impl INamespaceWalkCB2 {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn FoundItem<P0>(&self, psf: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellFolder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FoundItem)(::windows::core::Vtable::as_raw(self), psf.into().abi(), pidl).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn EnterFolder<P0>(&self, psf: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellFolder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnterFolder)(::windows::core::Vtable::as_raw(self), psf.into().abi(), pidl).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn LeaveFolder<P0>(&self, psf: P0, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellFolder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LeaveFolder)(::windows::core::Vtable::as_raw(self), psf.into().abi(), pidl).ok()
    }
    pub unsafe fn InitializeProgressDialog(&self, ppsztitle: *mut ::windows::core::PWSTR, ppszcancel: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeProgressDialog)(::windows::core::Vtable::as_raw(self), ppsztitle, ppszcancel).ok()
    }
}
impl ::core::cmp::PartialEq for INetworkFolderInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkFolderInternal {}
impl ::core::fmt::Debug for INetworkFolderInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkFolderInternal").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INewMenuClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INewMenuClient {}
impl ::core::fmt::Debug for INewMenuClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INewMenuClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INewShortcutHookA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INewShortcutHookA {}
impl ::core::fmt::Debug for INewShortcutHookA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INewShortcutHookA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INewShortcutHookW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INewShortcutHookW {}
impl ::core::fmt::Debug for INewShortcutHookW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INewShortcutHookW").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INewWDEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INewWDEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INewWDEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INewWDEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl INewWDEvents {
    pub unsafe fn FinalBack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinalBack)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FinalNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinalNext)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetCaption(&self, bstrcaption: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaption)).ok()
    }
    pub unsafe fn Caption(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Caption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_Property(&self, bstrpropertyname: &::windows::core::BSTR, pvproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_Property)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), pvproperty).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Property(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Property)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWizardButtons<P0, P1, P2>(&self, vfenableback: P0, vfenablenext: P1, vflastpage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWizardButtons)(::windows::core::Vtable::as_raw(self), vfenableback.into(), vfenablenext.into(), vflastpage.into()).ok()
    }
    pub unsafe fn SetHeaderText(&self, bstrheadertitle: &::windows::core::BSTR, bstrheadersubtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHeaderText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheadertitle), ::core::mem::transmute_copy(bstrheadersubtitle)).ok()
    }
}
impl ::core::cmp::PartialEq for INewWindowManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INewWindowManager {}
impl ::core::fmt::Debug for INewWindowManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INewWindowManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INotifyReplica {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyReplica {}
impl ::core::fmt::Debug for INotifyReplica {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyReplica").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjMgr {}
impl ::core::fmt::Debug for IObjMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectProvider {}
impl ::core::fmt::Debug for IObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithAppUserModelID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithAppUserModelID {}
impl ::core::fmt::Debug for IObjectWithAppUserModelID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithAppUserModelID").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithBackReferences {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithBackReferences {}
impl ::core::fmt::Debug for IObjectWithBackReferences {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithBackReferences").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithCancelEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithCancelEvent {}
impl ::core::fmt::Debug for IObjectWithCancelEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithCancelEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithFolderEnumMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithFolderEnumMode {}
impl ::core::fmt::Debug for IObjectWithFolderEnumMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithFolderEnumMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithProgID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithProgID {}
impl ::core::fmt::Debug for IObjectWithProgID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithProgID").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithSelection {}
impl ::core::fmt::Debug for IObjectWithSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithSelection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOpenControlPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenControlPanel {}
impl ::core::fmt::Debug for IOpenControlPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenControlPanel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOpenSearchSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenSearchSource {}
impl ::core::fmt::Debug for IOpenSearchSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenSearchSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOperationsProgressDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOperationsProgressDialog {}
impl ::core::fmt::Debug for IOperationsProgressDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOperationsProgressDialog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPackageDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPackageDebugSettings {}
impl ::core::fmt::Debug for IPackageDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageDebugSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPackageDebugSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPackageDebugSettings2 {}
impl ::core::fmt::Debug for IPackageDebugSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageDebugSettings2").field(&self.0).finish()
    }
}
impl IPackageDebugSettings2 {
    pub unsafe fn EnableDebugging<P0, P1, P2>(&self, packagefullname: P0, debuggercommandline: P1, environment: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableDebugging)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi(), debuggercommandline.into().abi(), environment.into().abi()).ok()
    }
    pub unsafe fn DisableDebugging<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DisableDebugging)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn Suspend<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Suspend)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn Resume<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Resume)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn TerminateAllProcesses<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TerminateAllProcesses)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn SetTargetSessionId(&self, sessionid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTargetSessionId)(::windows::core::Vtable::as_raw(self), sessionid).ok()
    }
    pub unsafe fn EnumerateBackgroundTasks<P0>(&self, packagefullname: P0, taskcount: *mut u32, taskids: *mut *mut ::windows::core::GUID, tasknames: *mut *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumerateBackgroundTasks)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi(), taskcount, taskids, tasknames).ok()
    }
    pub unsafe fn ActivateBackgroundTask(&self, taskid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ActivateBackgroundTask)(::windows::core::Vtable::as_raw(self), taskid).ok()
    }
    pub unsafe fn StartServicing<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartServicing)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn StopServicing<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StopServicing)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn StartSessionRedirection<P0>(&self, packagefullname: P0, sessionid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartSessionRedirection)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi(), sessionid).ok()
    }
    pub unsafe fn StopSessionRedirection<P0>(&self, packagefullname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StopSessionRedirection)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi()).ok()
    }
    pub unsafe fn GetPackageExecutionState<P0>(&self, packagefullname: P0) -> ::windows::core::Result<PACKAGE_EXECUTION_STATE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPackageExecutionState)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterForPackageStateChanges<P0, P1>(&self, packagefullname: P0, ppackageexecutionstatechangenotification: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPackageExecutionStateChangeNotification>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterForPackageStateChanges)(::windows::core::Vtable::as_raw(self), packagefullname.into().abi(), ppackageexecutionstatechangenotification.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterForPackageStateChanges(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterForPackageStateChanges)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
}
impl ::core::cmp::PartialEq for IPackageExecutionStateChangeNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPackageExecutionStateChangeNotification {}
impl ::core::fmt::Debug for IPackageExecutionStateChangeNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPackageExecutionStateChangeNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IParentAndItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IParentAndItem {}
impl ::core::fmt::Debug for IParentAndItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IParentAndItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IParseAndCreateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IParseAndCreateItem {}
impl ::core::fmt::Debug for IParseAndCreateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IParseAndCreateItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistFolder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFolder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistFolder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistFolder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistFolder2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistFolder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFolder2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistFolder2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn Initialize(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistFolder3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistFolder3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistFolder3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFolder3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistFolder3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn Initialize(&self, pidl: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), pidl).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetCurFolder(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurFolder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistIDList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistIDList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistIDList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistIDList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistIDList {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPreviewHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreviewHandler {}
impl ::core::fmt::Debug for IPreviewHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreviewHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPreviewHandlerFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreviewHandlerFrame {}
impl ::core::fmt::Debug for IPreviewHandlerFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreviewHandlerFrame").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPreviewHandlerVisuals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreviewHandlerVisuals {}
impl ::core::fmt::Debug for IPreviewHandlerVisuals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreviewHandlerVisuals").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPreviewItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreviewItem {}
impl ::core::fmt::Debug for IPreviewItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreviewItem").field(&self.0).finish()
    }
}
impl IPreviewItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPreviousVersionsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPreviousVersionsInfo {}
impl ::core::fmt::Debug for IPreviousVersionsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPreviousVersionsInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProfferService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProfferService {}
impl ::core::fmt::Debug for IProfferService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProfferService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProgressDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressDialog {}
impl ::core::fmt::Debug for IProgressDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressDialog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyKeyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyKeyStore {}
impl ::core::fmt::Debug for IPropertyKeyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyKeyStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPublishedApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublishedApp {}
impl ::core::fmt::Debug for IPublishedApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublishedApp").field(&self.0).finish()
    }
}
impl IPublishedApp {
    pub unsafe fn GetAppInfo(&self, pai: *mut APPINFODATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAppInfo)(::windows::core::Vtable::as_raw(self), pai).ok()
    }
    pub unsafe fn GetPossibleActions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPossibleActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSlowAppInfo(&self, psaid: *mut SLOWAPPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSlowAppInfo)(::windows::core::Vtable::as_raw(self), psaid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCachedSlowAppInfo(&self, psaid: *mut SLOWAPPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCachedSlowAppInfo)(::windows::core::Vtable::as_raw(self), psaid).ok()
    }
    pub unsafe fn IsInstalled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsInstalled)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IPublishedApp2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublishedApp2 {}
impl ::core::fmt::Debug for IPublishedApp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublishedApp2").field(&self.0).finish()
    }
}
impl IPublishedApp2 {
    pub unsafe fn GetAppInfo(&self, pai: *mut APPINFODATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAppInfo)(::windows::core::Vtable::as_raw(self), pai).ok()
    }
    pub unsafe fn GetPossibleActions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPossibleActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSlowAppInfo(&self, psaid: *mut SLOWAPPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSlowAppInfo)(::windows::core::Vtable::as_raw(self), psaid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCachedSlowAppInfo(&self, psaid: *mut SLOWAPPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedSlowAppInfo)(::windows::core::Vtable::as_raw(self), psaid).ok()
    }
    pub unsafe fn IsInstalled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsInstalled)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install(&self, pstinstall: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Install)(::windows::core::Vtable::as_raw(self), pstinstall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublishedAppInfo(&self, ppai: *mut PUBAPPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPublishedAppInfo)(::windows::core::Vtable::as_raw(self), ppai).ok()
    }
    pub unsafe fn Unschedule(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unschedule)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IPublishingWizard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublishingWizard {}
impl ::core::fmt::Debug for IPublishingWizard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublishingWizard").field(&self.0).finish()
    }
}
impl IPublishingWizard {
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPages(&self, apages: &mut [super::Controls::HPROPSHEETPAGE], pnpagesadded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddPages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(apages.as_ptr()), apages.len() as _, pnpagesadded).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetFirstPage(&self) -> ::windows::core::Result<super::Controls::HPROPSHEETPAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFirstPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetLastPage(&self) -> ::windows::core::Result<super::Controls::HPROPSHEETPAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IQueryAssociations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryAssociations {}
impl ::core::fmt::Debug for IQueryAssociations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryAssociations").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryCancelAutoPlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryCancelAutoPlay {}
impl ::core::fmt::Debug for IQueryCancelAutoPlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryCancelAutoPlay").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryCodePage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryCodePage {}
impl ::core::fmt::Debug for IQueryCodePage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryCodePage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryContinue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryContinue {}
impl ::core::fmt::Debug for IQueryContinue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryContinue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryContinueWithStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryContinueWithStatus {}
impl ::core::fmt::Debug for IQueryContinueWithStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryContinueWithStatus").field(&self.0).finish()
    }
}
impl IQueryContinueWithStatus {
    pub unsafe fn QueryContinue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryContinue)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IQueryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryInfo {}
impl ::core::fmt::Debug for IQueryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRegTreeItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRegTreeItem {}
impl ::core::fmt::Debug for IRegTreeItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegTreeItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRelatedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRelatedItem {}
impl ::core::fmt::Debug for IRelatedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRelatedItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteComputer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteComputer {}
impl ::core::fmt::Debug for IRemoteComputer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteComputer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IResolveShellLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResolveShellLink {}
impl ::core::fmt::Debug for IResolveShellLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResolveShellLink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IResultsFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultsFolder {}
impl ::core::fmt::Debug for IResultsFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultsFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRunnableTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunnableTask {}
impl ::core::fmt::Debug for IRunnableTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableTask").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IScriptErrorList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IScriptErrorList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IScriptErrorList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptErrorList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchBoxInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchBoxInfo {}
impl ::core::fmt::Debug for ISearchBoxInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchBoxInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchContext {}
impl ::core::fmt::Debug for ISearchContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchFolderItemFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchFolderItemFactory {}
impl ::core::fmt::Debug for ISearchFolderItemFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchFolderItemFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISharedBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISharedBitmap {}
impl ::core::fmt::Debug for ISharedBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedBitmap").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISharingConfigurationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISharingConfigurationManager {}
impl ::core::fmt::Debug for ISharingConfigurationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharingConfigurationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellApp {}
impl ::core::fmt::Debug for IShellApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellApp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IShellBrowser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IShellBrowser {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IShellBrowser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellBrowser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IShellBrowser {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IShellChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellChangeNotify {}
impl ::core::fmt::Debug for IShellChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellChangeNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellDetails {}
impl ::core::fmt::Debug for IShellDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellDispatch2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NameSpace(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NameSpace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &::windows::core::BSTR, options: i32, rootfolder: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BrowseForFolder)(::windows::core::Vtable::as_raw(self), hwnd, ::core::mem::transmute_copy(title), options, ::core::mem::transmute(rootfolder), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Windows)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Explore(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Explore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    pub unsafe fn MinimizeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MinimizeAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UndoMinimizeALL(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UndoMinimizeALL)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FileRun(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FileRun)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CascadeWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CascadeWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileVertically(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TileVertically)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileHorizontally(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TileHorizontally)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShutdownWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EjectPC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EjectPC)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTime)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TrayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TrayProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Help)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindFiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FindFiles)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindComputer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FindComputer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RefreshMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ControlPanelItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdir)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellDispatch3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NameSpace(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NameSpace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &::windows::core::BSTR, options: i32, rootfolder: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BrowseForFolder)(::windows::core::Vtable::as_raw(self), hwnd, ::core::mem::transmute_copy(title), options, ::core::mem::transmute(rootfolder), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Windows)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Explore(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Explore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    pub unsafe fn MinimizeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MinimizeAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UndoMinimizeALL(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UndoMinimizeALL)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FileRun(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FileRun)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CascadeWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CascadeWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileVertically(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TileVertically)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileHorizontally(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TileHorizontally)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ShutdownWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EjectPC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EjectPC)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTime)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TrayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TrayProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Help)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindFiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FindFiles)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindComputer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FindComputer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RefreshMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ControlPanelItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdir)).ok()
    }
    pub unsafe fn IsRestricted(&self, group: &::windows::core::BSTR, restriction: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRestricted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(restriction), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShellExecute(&self, file: &::windows::core::BSTR, vargs: super::super::System::Com::VARIANT, vdir: super::super::System::Com::VARIANT, voperation: super::super::System::Com::VARIANT, vshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShellExecute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(file), ::core::mem::transmute(vargs), ::core::mem::transmute(vdir), ::core::mem::transmute(voperation), ::core::mem::transmute(vshow)).ok()
    }
    pub unsafe fn FindPrinter(&self, name: &::windows::core::BSTR, location: &::windows::core::BSTR, model: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FindPrinter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(location), ::core::mem::transmute_copy(model)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSystemInformation(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemInformation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStart(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStop(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceStop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsServiceRunning(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsServiceRunning)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanStartStopService(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanStartStopService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserBar(&self, bstrclsid: &::windows::core::BSTR, bshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ShowBrowserBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclsid), ::core::mem::transmute(bshow), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellDispatch4 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NameSpace(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NameSpace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &::windows::core::BSTR, options: i32, rootfolder: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BrowseForFolder)(::windows::core::Vtable::as_raw(self), hwnd, ::core::mem::transmute_copy(title), options, ::core::mem::transmute(rootfolder), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Windows)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Explore(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Explore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    pub unsafe fn MinimizeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MinimizeAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UndoMinimizeALL(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UndoMinimizeALL)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FileRun(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FileRun)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CascadeWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CascadeWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileVertically(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TileVertically)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileHorizontally(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TileHorizontally)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShutdownWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EjectPC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EjectPC)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTime)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TrayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TrayProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Help)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindFiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFiles)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindComputer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindComputer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RefreshMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ControlPanelItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdir)).ok()
    }
    pub unsafe fn IsRestricted(&self, group: &::windows::core::BSTR, restriction: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsRestricted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(restriction), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShellExecute(&self, file: &::windows::core::BSTR, vargs: super::super::System::Com::VARIANT, vdir: super::super::System::Com::VARIANT, voperation: super::super::System::Com::VARIANT, vshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ShellExecute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(file), ::core::mem::transmute(vargs), ::core::mem::transmute(vdir), ::core::mem::transmute(voperation), ::core::mem::transmute(vshow)).ok()
    }
    pub unsafe fn FindPrinter(&self, name: &::windows::core::BSTR, location: &::windows::core::BSTR, model: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FindPrinter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(location), ::core::mem::transmute_copy(model)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSystemInformation(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemInformation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStart(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ServiceStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStop(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ServiceStop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsServiceRunning(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsServiceRunning)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanStartStopService(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CanStartStopService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserBar(&self, bstrclsid: &::windows::core::BSTR, bshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ShowBrowserBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclsid), ::core::mem::transmute(bshow), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToRecent(&self, varfile: super::super::System::Com::VARIANT, bstrcategory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddToRecent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varfile), ::core::mem::transmute_copy(bstrcategory)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch5").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellDispatch5 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NameSpace(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.NameSpace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &::windows::core::BSTR, options: i32, rootfolder: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BrowseForFolder)(::windows::core::Vtable::as_raw(self), hwnd, ::core::mem::transmute_copy(title), options, ::core::mem::transmute(rootfolder), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Windows)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Explore(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Explore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    pub unsafe fn MinimizeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.MinimizeAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UndoMinimizeALL(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UndoMinimizeALL)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FileRun(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FileRun)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CascadeWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CascadeWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileVertically(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TileVertically)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileHorizontally(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TileHorizontally)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShutdownWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EjectPC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EjectPC)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTime)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TrayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TrayProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Help)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindFiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindFiles)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindComputer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindComputer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RefreshMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ControlPanelItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdir)).ok()
    }
    pub unsafe fn IsRestricted(&self, group: &::windows::core::BSTR, restriction: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsRestricted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(restriction), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShellExecute(&self, file: &::windows::core::BSTR, vargs: super::super::System::Com::VARIANT, vdir: super::super::System::Com::VARIANT, voperation: super::super::System::Com::VARIANT, vshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShellExecute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(file), ::core::mem::transmute(vargs), ::core::mem::transmute(vdir), ::core::mem::transmute(voperation), ::core::mem::transmute(vshow)).ok()
    }
    pub unsafe fn FindPrinter(&self, name: &::windows::core::BSTR, location: &::windows::core::BSTR, model: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindPrinter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(location), ::core::mem::transmute_copy(model)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSystemInformation(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSystemInformation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStart(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ServiceStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStop(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ServiceStop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsServiceRunning(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsServiceRunning)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanStartStopService(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CanStartStopService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserBar(&self, bstrclsid: &::windows::core::BSTR, bshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowBrowserBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclsid), ::core::mem::transmute(bshow), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToRecent(&self, varfile: super::super::System::Com::VARIANT, bstrcategory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddToRecent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varfile), ::core::mem::transmute_copy(bstrcategory)).ok()
    }
    pub unsafe fn WindowsSecurity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WindowsSecurity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ToggleDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ToggleDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExplorerPolicy(&self, bstrpolicyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExplorerPolicy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpolicyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSetting(&self, lsetting: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSetting)(::windows::core::Vtable::as_raw(self), lsetting, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellDispatch6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellDispatch6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellDispatch6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellDispatch6").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellDispatch6 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NameSpace(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.NameSpace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &::windows::core::BSTR, options: i32, rootfolder: super::super::System::Com::VARIANT) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BrowseForFolder)(::windows::core::Vtable::as_raw(self), hwnd, ::core::mem::transmute_copy(title), options, ::core::mem::transmute(rootfolder), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Windows)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Open)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Explore(&self, vdir: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Explore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdir)).ok()
    }
    pub unsafe fn MinimizeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.MinimizeAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UndoMinimizeALL(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UndoMinimizeALL)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FileRun(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FileRun)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CascadeWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CascadeWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileVertically(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TileVertically)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TileHorizontally(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TileHorizontally)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownWindows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ShutdownWindows)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EjectPC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EjectPC)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTime(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTime)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TrayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TrayProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Help)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindFiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindFiles)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FindComputer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindComputer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RefreshMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ControlPanelItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdir)).ok()
    }
    pub unsafe fn IsRestricted(&self, group: &::windows::core::BSTR, restriction: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsRestricted)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(group), ::core::mem::transmute_copy(restriction), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShellExecute(&self, file: &::windows::core::BSTR, vargs: super::super::System::Com::VARIANT, vdir: super::super::System::Com::VARIANT, voperation: super::super::System::Com::VARIANT, vshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShellExecute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(file), ::core::mem::transmute(vargs), ::core::mem::transmute(vdir), ::core::mem::transmute(voperation), ::core::mem::transmute(vshow)).ok()
    }
    pub unsafe fn FindPrinter(&self, name: &::windows::core::BSTR, location: &::windows::core::BSTR, model: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindPrinter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(location), ::core::mem::transmute_copy(model)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSystemInformation(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSystemInformation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStart(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ServiceStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ServiceStop(&self, servicename: &::windows::core::BSTR, persistent: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ServiceStop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), ::core::mem::transmute(persistent), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsServiceRunning(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsServiceRunning)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanStartStopService(&self, servicename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CanStartStopService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(servicename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserBar(&self, bstrclsid: &::windows::core::BSTR, bshow: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShowBrowserBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclsid), ::core::mem::transmute(bshow), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToRecent(&self, varfile: super::super::System::Com::VARIANT, bstrcategory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddToRecent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varfile), ::core::mem::transmute_copy(bstrcategory)).ok()
    }
    pub unsafe fn WindowsSecurity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.WindowsSecurity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ToggleDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ToggleDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExplorerPolicy(&self, bstrpolicyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ExplorerPolicy)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpolicyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSetting(&self, lsetting: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSetting)(::windows::core::Vtable::as_raw(self), lsetting, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn WindowSwitcher(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WindowSwitcher)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IShellExtInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellExtInit {}
impl ::core::fmt::Debug for IShellExtInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellExtInit").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellFavoritesNameSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellFavoritesNameSpace {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellFavoritesNameSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFavoritesNameSpace").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellFolder {}
impl ::core::fmt::Debug for IShellFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellFolder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellFolder2 {}
impl ::core::fmt::Debug for IShellFolder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolder2").field(&self.0).finish()
    }
}
impl IShellFolder2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn ParseDisplayName<P0, P1, P2>(&self, hwnd: P0, pbc: P1, pszdisplayname: P2, pcheaten: ::core::option::Option<*mut u32>, ppidl: *mut *mut Common::ITEMIDLIST, pdwattributes: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IBindCtx>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ParseDisplayName)(::windows::core::Vtable::as_raw(self), hwnd.into(), pbc.into().abi(), pszdisplayname.into().abi(), ::core::mem::transmute(pcheaten.unwrap_or(::std::ptr::null_mut())), ppidl, pdwattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects<P0>(&self, hwnd: P0, grfflags: u32, ppenumidlist: *mut ::core::option::Option<IEnumIDList>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), hwnd.into(), grfflags, ::core::mem::transmute(ppenumidlist))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn BindToObject<P0, T>(&self, pidl: *const Common::ITEMIDLIST, pbc: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IBindCtx>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BindToObject)(::windows::core::Vtable::as_raw(self), pidl, pbc.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn BindToStorage<P0, T>(&self, pidl: *const Common::ITEMIDLIST, pbc: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IBindCtx>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BindToStorage)(::windows::core::Vtable::as_raw(self), pidl, pbc.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn CompareIDs<P0>(&self, lparam: P0, pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.CompareIDs)(::windows::core::Vtable::as_raw(self), lparam.into(), pidl1, pidl2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewObject<P0, T>(&self, hwndowner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateViewObject)(::windows::core::Vtable::as_raw(self), hwndowner.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetAttributesOf(&self, apidl: &[*const Common::ITEMIDLIST], rgfinout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributesOf)(::windows::core::Vtable::as_raw(self), apidl.len() as _, ::core::mem::transmute(apidl.as_ptr()), rgfinout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn GetUIObjectOf<P0, T>(&self, hwndowner: P0, apidl: &[*const Common::ITEMIDLIST], rgfreserved: ::core::option::Option<*mut u32>) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUIObjectOf)(::windows::core::Vtable::as_raw(self), hwndowner.into(), apidl.len() as _, ::core::mem::transmute(apidl.as_ptr()), &<T as ::windows::core::Interface>::IID, ::core::mem::transmute(rgfreserved.unwrap_or(::std::ptr::null_mut())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetDisplayNameOf(&self, pidl: *const Common::ITEMIDLIST, uflags: SHGDNF, pname: *mut Common::STRRET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayNameOf)(::windows::core::Vtable::as_raw(self), pidl, uflags, pname).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn SetNameOf<P0, P1>(&self, hwnd: P0, pidl: *const Common::ITEMIDLIST, pszname: P1, uflags: SHGDNF, ppidlout: ::core::option::Option<*mut *mut Common::ITEMIDLIST>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNameOf)(::windows::core::Vtable::as_raw(self), hwnd.into(), pidl, pszname.into().abi(), uflags, ::core::mem::transmute(ppidlout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IShellFolderBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellFolderBand {}
impl ::core::fmt::Debug for IShellFolderBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderBand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellFolderView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellFolderView {}
impl ::core::fmt::Debug for IShellFolderView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellFolderViewCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellFolderViewCB {}
impl ::core::fmt::Debug for IShellFolderViewCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderViewCB").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellFolderViewDual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellFolderViewDual {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellFolderViewDual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderViewDual").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellFolderViewDual2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellFolderViewDual2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellFolderViewDual2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderViewDual2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellFolderViewDual2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folder(&self) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Folder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectedItems(&self) -> ::windows::core::Result<FolderItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SelectedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FocusedItem(&self) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FocusedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SelectItem(&self, pvfi: *const super::super::System::Com::VARIANT, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectItem)(::windows::core::Vtable::as_raw(self), pvfi, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PopupItemMenu<P0>(&self, pfi: P0, vx: super::super::System::Com::VARIANT, vy: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<FolderItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PopupItemMenu)(::windows::core::Vtable::as_raw(self), pfi.into().abi(), ::core::mem::transmute(vx), ::core::mem::transmute(vy), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Script(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Script)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ViewOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ViewOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellFolderViewDual3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellFolderViewDual3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellFolderViewDual3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellFolderViewDual3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellFolderViewDual3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folder(&self) -> ::windows::core::Result<Folder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Folder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectedItems(&self) -> ::windows::core::Result<FolderItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SelectedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FocusedItem(&self) -> ::windows::core::Result<FolderItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FocusedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SelectItem(&self, pvfi: *const super::super::System::Com::VARIANT, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SelectItem)(::windows::core::Vtable::as_raw(self), pvfi, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PopupItemMenu<P0>(&self, pfi: P0, vx: super::super::System::Com::VARIANT, vy: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<FolderItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PopupItemMenu)(::windows::core::Vtable::as_raw(self), pfi.into().abi(), ::core::mem::transmute(vx), ::core::mem::transmute(vy), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Script(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Script)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ViewOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ViewOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentViewMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentViewMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentViewMode(&self, viewmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentViewMode)(::windows::core::Vtable::as_raw(self), viewmode).ok()
    }
    pub unsafe fn SelectItemRelative(&self, irelative: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectItemRelative)(::windows::core::Vtable::as_raw(self), irelative).ok()
    }
}
impl ::core::cmp::PartialEq for IShellIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellIcon {}
impl ::core::fmt::Debug for IShellIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellIcon").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellIconOverlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellIconOverlay {}
impl ::core::fmt::Debug for IShellIconOverlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellIconOverlay").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellIconOverlayIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellIconOverlayIdentifier {}
impl ::core::fmt::Debug for IShellIconOverlayIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellIconOverlayIdentifier").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellIconOverlayManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellIconOverlayManager {}
impl ::core::fmt::Debug for IShellIconOverlayManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellIconOverlayManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellImageData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellImageData {}
impl ::core::fmt::Debug for IShellImageData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellImageData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellImageDataAbort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellImageDataAbort {}
impl ::core::fmt::Debug for IShellImageDataAbort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellImageDataAbort").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellImageDataFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellImageDataFactory {}
impl ::core::fmt::Debug for IShellImageDataFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellImageDataFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItem {}
impl ::core::fmt::Debug for IShellItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItem2 {}
impl ::core::fmt::Debug for IShellItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItem2").field(&self.0).finish()
    }
}
impl IShellItem2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToHandler<P0, T>(&self, pbc: P0, bhid: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IBindCtx>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BindToHandler)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), bhid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self, sigdnname: SIGDN) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), sigdnname, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn GetAttributes(&self, sfgaomask: super::super::System::SystemServices::SFGAO_FLAGS) -> ::windows::core::Result<super::super::System::SystemServices::SFGAO_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributes)(::windows::core::Vtable::as_raw(self), sfgaomask, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Compare<P0>(&self, psi: P0, hint: u32) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellItem>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), psi.into().abi(), hint, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IShellItemArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItemArray {}
impl ::core::fmt::Debug for IShellItemArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItemArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellItemFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItemFilter {}
impl ::core::fmt::Debug for IShellItemFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItemFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellItemImageFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItemImageFactory {}
impl ::core::fmt::Debug for IShellItemImageFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItemImageFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellItemResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellItemResources {}
impl ::core::fmt::Debug for IShellItemResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellItemResources").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellLibrary {}
impl ::core::fmt::Debug for IShellLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLibrary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellLinkA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellLinkA {}
impl ::core::fmt::Debug for IShellLinkA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLinkA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellLinkDataList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellLinkDataList {}
impl ::core::fmt::Debug for IShellLinkDataList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLinkDataList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellLinkDual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellLinkDual {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellLinkDual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLinkDual").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellLinkDual2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellLinkDual2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellLinkDual2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLinkDual2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellLinkDual2 {
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath(&self, bs: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bs: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs)).ok()
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.WorkingDirectory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWorkingDirectory(&self, bs: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWorkingDirectory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs)).ok()
    }
    pub unsafe fn Arguments(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Arguments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetArguments(&self, bs: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetArguments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs)).ok()
    }
    pub unsafe fn Hotkey(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Hotkey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHotkey(&self, ihk: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHotkey)(::windows::core::Vtable::as_raw(self), ihk).ok()
    }
    pub unsafe fn ShowCommand(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ShowCommand)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetShowCommand(&self, ishowcommand: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetShowCommand)(::windows::core::Vtable::as_raw(self), ishowcommand).ok()
    }
    pub unsafe fn Resolve(&self, fflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Resolve)(::windows::core::Vtable::as_raw(self), fflags).ok()
    }
    pub unsafe fn GetIconLocation(&self, pbs: *mut ::windows::core::BSTR, piicon: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIconLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbs), piicon).ok()
    }
    pub unsafe fn SetIconLocation(&self, bs: &::windows::core::BSTR, iicon: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIconLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bs), iicon).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, vwhere: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vwhere)).ok()
    }
}
impl ::core::cmp::PartialEq for IShellLinkW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellLinkW {}
impl ::core::fmt::Debug for IShellLinkW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellLinkW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellMenu {}
impl ::core::fmt::Debug for IShellMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellMenu").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellMenuCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellMenuCallback {}
impl ::core::fmt::Debug for IShellMenuCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellMenuCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellNameSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellNameSpace {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellNameSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellNameSpace").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellNameSpace {
    pub unsafe fn MoveSelectionUp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveSelectionUp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MoveSelectionDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveSelectionDown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetSort)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn NewFolder(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NewFolder)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Synchronize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Synchronize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Import(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Import)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Export(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Export)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InvokeContextMenuCommand(&self, strcommand: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvokeContextMenuCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strcommand)).ok()
    }
    pub unsafe fn MoveSelectionTo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveSelectionTo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SubscriptionsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSubscriptionForSelection(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSubscriptionForSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSubscriptionForSelection(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DeleteSubscriptionForSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRoot(&self, bstrfullpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRoot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfullpath)).ok()
    }
}
impl ::core::cmp::PartialEq for IShellPropSheetExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellPropSheetExt {}
impl ::core::fmt::Debug for IShellPropSheetExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellPropSheetExt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellRunDll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellRunDll {}
impl ::core::fmt::Debug for IShellRunDll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellRunDll").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellService {}
impl ::core::fmt::Debug for IShellService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IShellTaskScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShellTaskScheduler {}
impl ::core::fmt::Debug for IShellTaskScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellTaskScheduler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper2 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper3 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper4 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper5").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper5 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msIsSiteMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msIsSiteMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeShowThumbBar)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msSiteModeAddThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<P0, P1>(&self, buttonid: super::super::System::Com::VARIANT, fenabled: P0, fvisible: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeUpdateThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buttonid), fenabled.into(), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &::windows::core::BSTR, pvardescription: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeSetIconOverlay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(iconurl), pvardescription).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeClearIconOverlay)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msAddSiteMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeCreateJumpList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheader)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &::windows::core::BSTR, bstractionuri: &::windows::core::BSTR, bstriconuri: &::windows::core::BSTR, pvarwindowtype: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeAddJumpListItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstractionuri), ::core::mem::transmute_copy(bstriconuri), pvarwindowtype).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeClearJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeShowJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msSiteModeAddButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, uistyleid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeShowButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute(uistyleid)).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msIsSiteModeFirstRun<P0>(&self, fpreservestate: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msIsSiteModeFirstRun)(::windows::core::Vtable::as_raw(self), fpreservestate.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &::windows::core::BSTR, bstrfiltername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msAddTrackingProtectionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(bstrfiltername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msTrackingProtectionEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msActiveXFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper6").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper6 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msIsSiteMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msIsSiteMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeShowThumbBar)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeAddThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<P0, P1>(&self, buttonid: super::super::System::Com::VARIANT, fenabled: P0, fvisible: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeUpdateThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buttonid), fenabled.into(), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &::windows::core::BSTR, pvardescription: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeSetIconOverlay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(iconurl), pvardescription).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeClearIconOverlay)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msAddSiteMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeCreateJumpList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheader)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &::windows::core::BSTR, bstractionuri: &::windows::core::BSTR, bstriconuri: &::windows::core::BSTR, pvarwindowtype: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeAddJumpListItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstractionuri), ::core::mem::transmute_copy(bstriconuri), pvarwindowtype).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeClearJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeShowJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeAddButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, uistyleid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeShowButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute(uistyleid)).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msIsSiteModeFirstRun<P0>(&self, fpreservestate: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msIsSiteModeFirstRun)(::windows::core::Vtable::as_raw(self), fpreservestate.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &::windows::core::BSTR, bstrfiltername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msAddTrackingProtectionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(bstrfiltername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msTrackingProtectionEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msActiveXFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msProvisionNetworks(&self, bstrprovisioningxml: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msProvisionNetworks)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovisioningxml), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msReportSafeUrl)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeRefreshBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msSiteModeClearBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msDiagnoseConnectionUILess)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msLaunchNetworkClientHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msChangeDefaultBrowser<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msChangeDefaultBrowser)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper7").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper7 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msIsSiteMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msIsSiteMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeShowThumbBar)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeAddThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<P0, P1>(&self, buttonid: super::super::System::Com::VARIANT, fenabled: P0, fvisible: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeUpdateThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buttonid), fenabled.into(), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &::windows::core::BSTR, pvardescription: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeSetIconOverlay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(iconurl), pvardescription).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeClearIconOverlay)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msAddSiteMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeCreateJumpList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheader)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &::windows::core::BSTR, bstractionuri: &::windows::core::BSTR, bstriconuri: &::windows::core::BSTR, pvarwindowtype: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeAddJumpListItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstractionuri), ::core::mem::transmute_copy(bstriconuri), pvarwindowtype).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeClearJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeShowJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeAddButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, uistyleid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeShowButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute(uistyleid)).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msIsSiteModeFirstRun<P0>(&self, fpreservestate: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msIsSiteModeFirstRun)(::windows::core::Vtable::as_raw(self), fpreservestate.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &::windows::core::BSTR, bstrfiltername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msAddTrackingProtectionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(bstrfiltername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msTrackingProtectionEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msActiveXFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msProvisionNetworks(&self, bstrprovisioningxml: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msProvisionNetworks)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovisioningxml), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msReportSafeUrl)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeRefreshBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msSiteModeClearBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msDiagnoseConnectionUILess)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msLaunchNetworkClientHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msChangeDefaultBrowser<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msChangeDefaultBrowser)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msStopPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdate(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msStartPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msStartPeriodicTileUpdateBatch)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msClearTile)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueue<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msEnableTileNotificationQueue)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.msPinnedSiteState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msEnableTileNotificationQueueForSquare150x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForWide310x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msEnableTileNotificationQueueForWide310x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.msEnableTileNotificationQueueForSquare310x310)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msScheduledTileNotification(&self, bstrnotificationxml: &::windows::core::BSTR, bstrnotificationid: &::windows::core::BSTR, bstrnotificationtag: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, expirationtime: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationxml), ::core::mem::transmute_copy(bstrnotificationid), ::core::mem::transmute_copy(bstrnotificationtag), ::core::mem::transmute(starttime), ::core::mem::transmute(expirationtime)).ok()
    }
    pub unsafe fn msRemoveScheduledTileNotification(&self, bstrnotificationid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msRemoveScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicBadgeUpdate(&self, pollinguri: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msStartPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pollinguri), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msStopPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.msLaunchInternetOptions)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper8 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper8").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper8 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msIsSiteMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msIsSiteMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeShowThumbBar)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeAddThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<P0, P1>(&self, buttonid: super::super::System::Com::VARIANT, fenabled: P0, fvisible: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeUpdateThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buttonid), fenabled.into(), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &::windows::core::BSTR, pvardescription: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeSetIconOverlay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(iconurl), pvardescription).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeClearIconOverlay)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msAddSiteMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeCreateJumpList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheader)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &::windows::core::BSTR, bstractionuri: &::windows::core::BSTR, bstriconuri: &::windows::core::BSTR, pvarwindowtype: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeAddJumpListItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstractionuri), ::core::mem::transmute_copy(bstriconuri), pvarwindowtype).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeClearJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeShowJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeAddButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, uistyleid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeShowButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute(uistyleid)).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msIsSiteModeFirstRun<P0>(&self, fpreservestate: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msIsSiteModeFirstRun)(::windows::core::Vtable::as_raw(self), fpreservestate.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &::windows::core::BSTR, bstrfiltername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msAddTrackingProtectionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(bstrfiltername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msTrackingProtectionEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msActiveXFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msProvisionNetworks(&self, bstrprovisioningxml: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msProvisionNetworks)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovisioningxml), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msReportSafeUrl)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeRefreshBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msSiteModeClearBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msDiagnoseConnectionUILess)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msLaunchNetworkClientHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msChangeDefaultBrowser<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msChangeDefaultBrowser)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msStopPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdate(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msStartPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msStartPeriodicTileUpdateBatch)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msClearTile)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueue<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msEnableTileNotificationQueue)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.msPinnedSiteState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msEnableTileNotificationQueueForSquare150x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForWide310x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msEnableTileNotificationQueueForWide310x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.msEnableTileNotificationQueueForSquare310x310)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msScheduledTileNotification(&self, bstrnotificationxml: &::windows::core::BSTR, bstrnotificationid: &::windows::core::BSTR, bstrnotificationtag: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, expirationtime: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationxml), ::core::mem::transmute_copy(bstrnotificationid), ::core::mem::transmute_copy(bstrnotificationtag), ::core::mem::transmute(starttime), ::core::mem::transmute(expirationtime)).ok()
    }
    pub unsafe fn msRemoveScheduledTileNotification(&self, bstrnotificationid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msRemoveScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicBadgeUpdate(&self, pollinguri: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msStartPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pollinguri), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msStopPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.msLaunchInternetOptions)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalFlag<P0>(&self, bstrflagstring: &::windows::core::BSTR, vfflag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetExperimentalFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrflagstring), vfflag.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalFlag(&self, bstrflagstring: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetExperimentalFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrflagstring), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExperimentalValue(&self, bstrvaluestring: &::windows::core::BSTR, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetExperimentalValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluestring), dwvalue).ok()
    }
    pub unsafe fn GetExperimentalValue(&self, bstrvaluestring: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetExperimentalValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluestring), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetAllExperimentalFlagsAndValues)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNeedIEAutoLaunchFlag(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedIEAutoLaunchFlag<P0>(&self, bstrurl: &::windows::core::BSTR, flag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), flag.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasNeedIEAutoLaunchFlag(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HasNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchIE<P0>(&self, bstrurl: &::windows::core::BSTR, automated: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LaunchIE)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), automated.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellUIHelper9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellUIHelper9 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellUIHelper9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellUIHelper9").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IShellUIHelper9 {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ResetFirstBootMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ResetSafeMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RefreshOfflineDesktop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddFavorite(&self, url: &::windows::core::BSTR, title: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AddFavorite)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(title.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn AddChannel(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AddChannel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDesktopComponent(&self, url: &::windows::core::BSTR, r#type: &::windows::core::BSTR, left: ::core::option::Option<*const super::super::System::Com::VARIANT>, top: ::core::option::Option<*const super::super::System::Com::VARIANT>, width: ::core::option::Option<*const super::super::System::Com::VARIANT>, height: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AddDesktopComponent)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(r#type), ::core::mem::transmute(left.unwrap_or(::std::ptr::null())), ::core::mem::transmute(top.unwrap_or(::std::ptr::null())), ::core::mem::transmute(width.unwrap_or(::std::ptr::null())), ::core::mem::transmute(height.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NavigateAndFind(&self, url: &::windows::core::BSTR, strquery: &::windows::core::BSTR, vartargetframe: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.NavigateAndFind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(strquery), vartargetframe).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<P0>(&self, fimport: P0, strimpexppath: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ImportExportFavorites)(::windows::core::Vtable::as_raw(self), fimport.into(), ::core::mem::transmute_copy(strimpexppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AutoCompleteSaveForm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(form.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoScan(&self, strsearch: &::windows::core::BSTR, strfailureurl: &::windows::core::BSTR, pvartargetframe: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AutoScan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsearch), ::core::mem::transmute_copy(strfailureurl), ::core::mem::transmute(pvartargetframe.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AutoCompleteAttach)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &::windows::core::BSTR, pvarin: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ShowBrowserUI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvarin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSearchProvider(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.AddSearchProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RunOnceShown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SkipRunOnce)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<P0, P1>(&self, fsqm: P0, fphishing: P1, bstrlocale: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CustomizeSettings)(::windows::core::Vtable::as_raw(self), fsqm.into(), fphishing.into(), ::core::mem::transmute_copy(bstrlocale)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqmEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SqmEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhishingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.PhishingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BrandImageUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.BrandImageUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SkipTabsWelcome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DiagnoseConnection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeClearType<P0>(&self, fset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CustomizeClearType)(::windows::core::Vtable::as_raw(self), fset.into()).ok()
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.IsSearchProviderInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.IsSearchMigrated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DefaultSearchProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceRequiredSettingsComplete<P0>(&self, fcomplete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RunOnceRequiredSettingsComplete)(::windows::core::Vtable::as_raw(self), fcomplete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RunOnceHasShown)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SearchGuideUrl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddService(&self, url: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url)).ok()
    }
    pub unsafe fn IsServiceInstalled(&self, url: &::windows::core::BSTR, verb: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsServiceInstalled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(verb), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.InPrivateFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &::windows::core::BSTR, title: &::windows::core::BSTR, r#type: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.AddToFavoritesBar)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(title), r#type).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BuildNewTabPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecentlyClosedVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetRecentlyClosedVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActivitiesVisible<P0>(&self, fvisible: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetActivitiesVisible)(::windows::core::Vtable::as_raw(self), fvisible.into()).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ContentDiscoveryReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSuggestedSitesEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSuggestedSites<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EnableSuggestedSites)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.NavigateToSuggestedSites)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrelativeurl)).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ShowTabsHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ShowInPrivateHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msIsSiteMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msIsSiteMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeShowThumbBar)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeAddThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<P0, P1>(&self, buttonid: super::super::System::Com::VARIANT, fenabled: P0, fvisible: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeUpdateThumbBarButton)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(buttonid), fenabled.into(), fvisible.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &::windows::core::BSTR, pvardescription: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeSetIconOverlay)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(iconurl), pvardescription).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeClearIconOverlay)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msAddSiteMode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeCreateJumpList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheader)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &::windows::core::BSTR, bstractionuri: &::windows::core::BSTR, bstriconuri: &::windows::core::BSTR, pvarwindowtype: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeAddJumpListItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstractionuri), ::core::mem::transmute_copy(bstriconuri), pvarwindowtype).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeClearJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeShowJumpList)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, bstriconurl: &::windows::core::BSTR, bstrtooltip: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeAddButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute_copy(bstriconurl), ::core::mem::transmute_copy(bstrtooltip), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: super::super::System::Com::VARIANT, uistyleid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeShowButtonStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(uibuttonid), ::core::mem::transmute(uistyleid)).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msSiteModeActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msIsSiteModeFirstRun<P0>(&self, fpreservestate: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msIsSiteModeFirstRun)(::windows::core::Vtable::as_raw(self), fpreservestate.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &::windows::core::BSTR, bstrfiltername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msAddTrackingProtectionList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute_copy(bstrfiltername)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msTrackingProtectionEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.msActiveXFilteringEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msProvisionNetworks(&self, bstrprovisioningxml: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msProvisionNetworks)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovisioningxml), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msReportSafeUrl)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeRefreshBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msSiteModeClearBadge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msDiagnoseConnectionUILess)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msLaunchNetworkClientHelp)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msChangeDefaultBrowser<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.msChangeDefaultBrowser)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msStopPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdate(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msStartPeriodicTileUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch(&self, pollinguris: super::super::System::Com::VARIANT, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msStartPeriodicTileUpdateBatch)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pollinguris), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msClearTile)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueue<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msEnableTileNotificationQueue)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msPinnedSiteState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msEnableTileNotificationQueueForSquare150x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForWide310x150<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msEnableTileNotificationQueueForWide310x150)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310<P0>(&self, fchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msEnableTileNotificationQueueForSquare310x310)(::windows::core::Vtable::as_raw(self), fchange.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msScheduledTileNotification(&self, bstrnotificationxml: &::windows::core::BSTR, bstrnotificationid: &::windows::core::BSTR, bstrnotificationtag: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, expirationtime: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationxml), ::core::mem::transmute_copy(bstrnotificationid), ::core::mem::transmute_copy(bstrnotificationtag), ::core::mem::transmute(starttime), ::core::mem::transmute(expirationtime)).ok()
    }
    pub unsafe fn msRemoveScheduledTileNotification(&self, bstrnotificationid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msRemoveScheduledTileNotification)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotificationid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn msStartPeriodicBadgeUpdate(&self, pollinguri: &::windows::core::BSTR, starttime: super::super::System::Com::VARIANT, uiupdaterecurrence: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msStartPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pollinguri), ::core::mem::transmute(starttime), ::core::mem::transmute(uiupdaterecurrence)).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msStopPeriodicBadgeUpdate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.msLaunchInternetOptions)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalFlag<P0>(&self, bstrflagstring: &::windows::core::BSTR, vfflag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetExperimentalFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrflagstring), vfflag.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalFlag(&self, bstrflagstring: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetExperimentalFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrflagstring), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExperimentalValue(&self, bstrvaluestring: &::windows::core::BSTR, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetExperimentalValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluestring), dwvalue).ok()
    }
    pub unsafe fn GetExperimentalValue(&self, bstrvaluestring: &::windows::core::BSTR) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetExperimentalValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvaluestring), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetAllExperimentalFlagsAndValues)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNeedIEAutoLaunchFlag(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedIEAutoLaunchFlag<P0>(&self, bstrurl: &::windows::core::BSTR, flag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), flag.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasNeedIEAutoLaunchFlag(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HasNeedIEAutoLaunchFlag)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchIE<P0>(&self, bstrurl: &::windows::core::BSTR, automated: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.LaunchIE)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl), automated.into()).ok()
    }
    pub unsafe fn GetCVListData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCVListData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCVListLocalData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCVListLocalData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEMIEListData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEMIEListData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEMIEListLocalData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEMIEListLocalData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenFavoritesPane(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenFavoritesPane)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OpenFavoritesSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenFavoritesSettings)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn LaunchInHVSI(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LaunchInHVSI)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrurl)).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IShellView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IShellView {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IShellView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellView").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IShellView {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IShellView2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IShellView2 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IShellView2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellView2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IShellView2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableModeless)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn UIActivate(&self, ustate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UIActivate)(::windows::core::Vtable::as_raw(self), ustate).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateViewWindow<P0, P1>(&self, psvprevious: P0, pfs: *const FOLDERSETTINGS, psb: P1, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellBrowser>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateViewWindow)(::windows::core::Vtable::as_raw(self), psvprevious.into().abi(), pfs, psb.into().abi(), prcview, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestroyViewWindow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DestroyViewWindow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrentInfo(&self) -> ::windows::core::Result<FOLDERSETTINGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn AddPropertySheetPages<P0>(&self, dwreserved: u32, pfn: super::Controls::LPFNSVADDPROPSHEETPAGE, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPropertySheetPages)(::windows::core::Vtable::as_raw(self), dwreserved, pfn, lparam.into()).ok()
    }
    pub unsafe fn SaveViewState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveViewState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SelectItem(&self, pidlitem: *const Common::ITEMIDLIST, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectItem)(::windows::core::Vtable::as_raw(self), pidlitem, uflags).ok()
    }
    pub unsafe fn GetItemObject<T>(&self, uitem: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemObject)(::windows::core::Vtable::as_raw(self), uitem, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for IShellView3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for IShellView3 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for IShellView3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellView3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl IShellView3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnableModeless)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn UIActivate(&self, ustate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UIActivate)(::windows::core::Vtable::as_raw(self), ustate).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateViewWindow<P0, P1>(&self, psvprevious: P0, pfs: *const FOLDERSETTINGS, psb: P1, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellView>>,
        P1: ::std::convert::Into<::windows::core::InParam<IShellBrowser>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateViewWindow)(::windows::core::Vtable::as_raw(self), psvprevious.into().abi(), pfs, psb.into().abi(), prcview, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestroyViewWindow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DestroyViewWindow)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrentInfo(&self) -> ::windows::core::Result<FOLDERSETTINGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn AddPropertySheetPages<P0>(&self, dwreserved: u32, pfn: super::Controls::LPFNSVADDPROPSHEETPAGE, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddPropertySheetPages)(::windows::core::Vtable::as_raw(self), dwreserved, pfn, lparam.into()).ok()
    }
    pub unsafe fn SaveViewState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveViewState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SelectItem(&self, pidlitem: *const Common::ITEMIDLIST, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SelectItem)(::windows::core::Vtable::as_raw(self), pidlitem, uflags).ok()
    }
    pub unsafe fn GetItemObject<T>(&self, uitem: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemObject)(::windows::core::Vtable::as_raw(self), uitem, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetView(&self, pvid: *mut ::windows::core::GUID, uview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetView)(::windows::core::Vtable::as_raw(self), pvid, uview).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateViewWindow2(&self, lpparams: *const SV2CVW2_PARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateViewWindow2)(::windows::core::Vtable::as_raw(self), lpparams).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn HandleRename(&self, pidlnew: *const Common::ITEMIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HandleRename)(::windows::core::Vtable::as_raw(self), pidlnew).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn SelectAndPositionItem(&self, pidlitem: *const Common::ITEMIDLIST, uflags: u32, ppt: *const super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SelectAndPositionItem)(::windows::core::Vtable::as_raw(self), pidlitem, uflags, ppt).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShellWindows {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShellWindows {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShellWindows {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShellWindows").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISortColumnArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISortColumnArray {}
impl ::core::fmt::Debug for ISortColumnArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISortColumnArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStartMenuPinnedList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStartMenuPinnedList {}
impl ::core::fmt::Debug for IStartMenuPinnedList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStartMenuPinnedList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStorageProviderBanners {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderBanners {}
impl ::core::fmt::Debug for IStorageProviderBanners {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderBanners").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStorageProviderCopyHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderCopyHook {}
impl ::core::fmt::Debug for IStorageProviderCopyHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderCopyHook").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStorageProviderHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderHandler {}
impl ::core::fmt::Debug for IStorageProviderHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStorageProviderPropertyHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderPropertyHandler {}
impl ::core::fmt::Debug for IStorageProviderPropertyHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderPropertyHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IStreamAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IStreamAsync {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IStreamAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamAsync").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamAsync {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSize)(::windows::core::Vtable::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Revert)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stat)(::windows::core::Vtable::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IStreamUnbufferedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStreamUnbufferedInfo {}
impl ::core::fmt::Debug for IStreamUnbufferedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamUnbufferedInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISuspensionDependencyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspensionDependencyManager {}
impl ::core::fmt::Debug for ISuspensionDependencyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspensionDependencyManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflict {}
impl ::core::fmt::Debug for ISyncMgrConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflict").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictFolder {}
impl ::core::fmt::Debug for ISyncMgrConflictFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictFolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictItems {}
impl ::core::fmt::Debug for ISyncMgrConflictItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictPresenter {}
impl ::core::fmt::Debug for ISyncMgrConflictPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictPresenter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictResolutionItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictResolutionItems {}
impl ::core::fmt::Debug for ISyncMgrConflictResolutionItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictResolutionItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictResolveInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictResolveInfo {}
impl ::core::fmt::Debug for ISyncMgrConflictResolveInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictResolveInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrConflictStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrConflictStore {}
impl ::core::fmt::Debug for ISyncMgrConflictStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrConflictStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrControl {}
impl ::core::fmt::Debug for ISyncMgrControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrEnumItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrEnumItems {}
impl ::core::fmt::Debug for ISyncMgrEnumItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrEnumItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrEvent {}
impl ::core::fmt::Debug for ISyncMgrEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrEventLinkUIOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrEventLinkUIOperation {}
impl ::core::fmt::Debug for ISyncMgrEventLinkUIOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrEventLinkUIOperation").field(&self.0).finish()
    }
}
impl ISyncMgrEventLinkUIOperation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Run<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Run)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrEventStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrEventStore {}
impl ::core::fmt::Debug for ISyncMgrEventStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrEventStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrHandler {}
impl ::core::fmt::Debug for ISyncMgrHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrHandlerCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrHandlerCollection {}
impl ::core::fmt::Debug for ISyncMgrHandlerCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrHandlerCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrHandlerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrHandlerInfo {}
impl ::core::fmt::Debug for ISyncMgrHandlerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrHandlerInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrRegister {}
impl ::core::fmt::Debug for ISyncMgrRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrRegister").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrResolutionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrResolutionHandler {}
impl ::core::fmt::Debug for ISyncMgrResolutionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrResolutionHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrScheduleWizardUIOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrScheduleWizardUIOperation {}
impl ::core::fmt::Debug for ISyncMgrScheduleWizardUIOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrScheduleWizardUIOperation").field(&self.0).finish()
    }
}
impl ISyncMgrScheduleWizardUIOperation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Run<P0>(&self, hwndowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Run)(::windows::core::Vtable::as_raw(self), hwndowner.into()).ok()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSessionCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSessionCreator {}
impl ::core::fmt::Debug for ISyncMgrSessionCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSessionCreator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSyncCallback {}
impl ::core::fmt::Debug for ISyncMgrSyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSyncCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSyncItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSyncItem {}
impl ::core::fmt::Debug for ISyncMgrSyncItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSyncItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSyncItemContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSyncItemContainer {}
impl ::core::fmt::Debug for ISyncMgrSyncItemContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSyncItemContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSyncItemInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSyncItemInfo {}
impl ::core::fmt::Debug for ISyncMgrSyncItemInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSyncItemInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSyncResult {}
impl ::core::fmt::Debug for ISyncMgrSyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSyncResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSynchronize {}
impl ::core::fmt::Debug for ISyncMgrSynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSynchronize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSynchronizeCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSynchronizeCallback {}
impl ::core::fmt::Debug for ISyncMgrSynchronizeCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSynchronizeCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrSynchronizeInvoke {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrSynchronizeInvoke {}
impl ::core::fmt::Debug for ISyncMgrSynchronizeInvoke {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrSynchronizeInvoke").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISyncMgrUIOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMgrUIOperation {}
impl ::core::fmt::Debug for ISyncMgrUIOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMgrUIOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for ITEMSPACING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ITEMSPACING {
    fn eq(&self, other: &Self) -> bool {
        self.cxSmall == other.cxSmall && self.cySmall == other.cySmall && self.cxLarge == other.cxLarge && self.cyLarge == other.cyLarge
    }
}
impl ::core::cmp::Eq for ITEMSPACING {}
impl ::core::fmt::Debug for ITEMSPACING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ITEMSPACING").field("cxSmall", &self.cxSmall).field("cySmall", &self.cySmall).field("cxLarge", &self.cxLarge).field("cyLarge", &self.cyLarge).finish()
    }
}
impl ::core::cmp::PartialEq for ITaskbarList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskbarList {}
impl ::core::fmt::Debug for ITaskbarList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskbarList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITaskbarList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskbarList2 {}
impl ::core::fmt::Debug for ITaskbarList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskbarList2").field(&self.0).finish()
    }
}
impl ITaskbarList2 {
    pub unsafe fn HrInit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HrInit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ActivateTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ActivateTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActiveAlt<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetActiveAlt)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
}
impl ::core::cmp::PartialEq for ITaskbarList3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskbarList3 {}
impl ::core::fmt::Debug for ITaskbarList3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskbarList3").field(&self.0).finish()
    }
}
impl ITaskbarList3 {
    pub unsafe fn HrInit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.HrInit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ActivateTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ActivateTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActiveAlt<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetActiveAlt)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MarkFullscreenWindow<P0, P1>(&self, hwnd: P0, ffullscreen: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.MarkFullscreenWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), ffullscreen.into()).ok()
    }
}
impl ::core::cmp::PartialEq for ITaskbarList4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskbarList4 {}
impl ::core::fmt::Debug for ITaskbarList4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskbarList4").field(&self.0).finish()
    }
}
impl ITaskbarList4 {
    pub unsafe fn HrInit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HrInit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DeleteTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ActivateTab<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ActivateTab)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActiveAlt<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetActiveAlt)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MarkFullscreenWindow<P0, P1>(&self, hwnd: P0, ffullscreen: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MarkFullscreenWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), ffullscreen.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProgressValue<P0>(&self, hwnd: P0, ullcompleted: u64, ulltotal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProgressValue)(::windows::core::Vtable::as_raw(self), hwnd.into(), ullcompleted, ulltotal).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProgressState<P0>(&self, hwnd: P0, tbpflags: TBPFLAG) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProgressState)(::windows::core::Vtable::as_raw(self), hwnd.into(), tbpflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterTab<P0, P1>(&self, hwndtab: P0, hwndmdi: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterTab)(::windows::core::Vtable::as_raw(self), hwndtab.into(), hwndmdi.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterTab<P0>(&self, hwndtab: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterTab)(::windows::core::Vtable::as_raw(self), hwndtab.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTabOrder<P0, P1>(&self, hwndtab: P0, hwndinsertbefore: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTabOrder)(::windows::core::Vtable::as_raw(self), hwndtab.into(), hwndinsertbefore.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTabActive<P0, P1>(&self, hwndtab: P0, hwndmdi: P1, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTabActive)(::windows::core::Vtable::as_raw(self), hwndtab.into(), hwndmdi.into(), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ThumbBarAddButtons<P0>(&self, hwnd: P0, pbutton: &[THUMBBUTTON]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ThumbBarAddButtons)(::windows::core::Vtable::as_raw(self), hwnd.into(), pbutton.len() as _, ::core::mem::transmute(pbutton.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ThumbBarUpdateButtons<P0>(&self, hwnd: P0, pbutton: &[THUMBBUTTON]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ThumbBarUpdateButtons)(::windows::core::Vtable::as_raw(self), hwnd.into(), pbutton.len() as _, ::core::mem::transmute(pbutton.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn ThumbBarSetImageList<P0, P1>(&self, hwnd: P0, himl: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::Controls::HIMAGELIST>,
    {
        (::windows::core::Vtable::vtable(self).base__.ThumbBarSetImageList)(::windows::core::Vtable::as_raw(self), hwnd.into(), himl.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetOverlayIcon<P0, P1, P2>(&self, hwnd: P0, hicon: P1, pszdescription: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::WindowsAndMessaging::HICON>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOverlayIcon)(::windows::core::Vtable::as_raw(self), hwnd.into(), hicon.into(), pszdescription.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetThumbnailTooltip<P0, P1>(&self, hwnd: P0, psztip: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetThumbnailTooltip)(::windows::core::Vtable::as_raw(self), hwnd.into(), psztip.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetThumbnailClip<P0>(&self, hwnd: P0, prcclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetThumbnailClip)(::windows::core::Vtable::as_raw(self), hwnd.into(), prcclip).ok()
    }
}
impl ::core::cmp::PartialEq for IThumbnailCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailCache {}
impl ::core::fmt::Debug for IThumbnailCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailCachePrimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailCachePrimer {}
impl ::core::fmt::Debug for IThumbnailCachePrimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailCachePrimer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailCapture {}
impl ::core::fmt::Debug for IThumbnailCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailCapture").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailHandlerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailHandlerFactory {}
impl ::core::fmt::Debug for IThumbnailHandlerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailHandlerFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailProvider {}
impl ::core::fmt::Debug for IThumbnailProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailSettings {}
impl ::core::fmt::Debug for IThumbnailSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailStreamCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailStreamCache {}
impl ::core::fmt::Debug for IThumbnailStreamCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailStreamCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITrackShellMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrackShellMenu {}
impl ::core::fmt::Debug for ITrackShellMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrackShellMenu").field(&self.0).finish()
    }
}
impl ITrackShellMenu {
    pub unsafe fn Initialize<P0>(&self, psmc: P0, uid: u32, uidancestor: u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellMenuCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), psmc.into().abi(), uid, uidancestor, dwflags).ok()
    }
    pub unsafe fn GetMenuInfo(&self, ppsmc: ::core::option::Option<*mut ::core::option::Option<IShellMenuCallback>>, puid: ::core::option::Option<*mut u32>, puidancestor: ::core::option::Option<*mut u32>, pdwflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMenuInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppsmc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(puid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(puidancestor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub unsafe fn SetShellFolder<P0, P1>(&self, psf: P0, pidlfolder: ::core::option::Option<*const Common::ITEMIDLIST>, hkey: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IShellFolder>>,
        P1: ::std::convert::Into<super::super::System::Registry::HKEY>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetShellFolder)(::windows::core::Vtable::as_raw(self), psf.into().abi(), ::core::mem::transmute(pidlfolder.unwrap_or(::std::ptr::null())), hkey.into(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetShellFolder(&self, pdwflags: *mut u32, ppidl: *mut *mut Common::ITEMIDLIST, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetShellFolder)(::windows::core::Vtable::as_raw(self), pdwflags, ppidl, riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetMenu<P0, P1>(&self, hmenu: P0, hwnd: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMenu)(::windows::core::Vtable::as_raw(self), hmenu.into(), hwnd.into(), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetMenu(&self, phmenu: ::core::option::Option<*mut super::WindowsAndMessaging::HMENU>, phwnd: ::core::option::Option<*mut super::super::Foundation::HWND>, pdwflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMenu)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phmenu.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phwnd.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn InvalidateItem(&self, psmd: ::core::option::Option<*const SMDATA>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvalidateItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psmd.unwrap_or(::std::ptr::null())), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_Common\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetState(&self, psmd: *mut SMDATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), psmd).ok()
    }
    pub unsafe fn SetMenuToolbar<P0>(&self, punk: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMenuToolbar)(::windows::core::Vtable::as_raw(self), punk.into().abi(), dwflags).ok()
    }
}
impl ::core::cmp::PartialEq for ITranscodeImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITranscodeImage {}
impl ::core::fmt::Debug for ITranscodeImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITranscodeImage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransferAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransferAdviseSink {}
impl ::core::fmt::Debug for ITransferAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransferAdviseSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransferDestination {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransferDestination {}
impl ::core::fmt::Debug for ITransferDestination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransferDestination").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransferMediumItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransferMediumItem {}
impl ::core::fmt::Debug for ITransferMediumItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransferMediumItem").field(&self.0).finish()
    }
}
impl ITransferMediumItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITransferSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransferSource {}
impl ::core::fmt::Debug for ITransferSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransferSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITravelEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITravelEntry {}
impl ::core::fmt::Debug for ITravelEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITravelEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITravelLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITravelLog {}
impl ::core::fmt::Debug for ITravelLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITravelLog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITravelLogClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITravelLogClient {}
impl ::core::fmt::Debug for ITravelLogClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITravelLogClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITravelLogEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITravelLogEntry {}
impl ::core::fmt::Debug for ITravelLogEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITravelLogEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITravelLogStg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITravelLogStg {}
impl ::core::fmt::Debug for ITravelLogStg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITravelLogStg").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITrayDeskBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrayDeskBand {}
impl ::core::fmt::Debug for ITrayDeskBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrayDeskBand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IURLSearchHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IURLSearchHook {}
impl ::core::fmt::Debug for IURLSearchHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IURLSearchHook").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IURLSearchHook2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IURLSearchHook2 {}
impl ::core::fmt::Debug for IURLSearchHook2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IURLSearchHook2").field(&self.0).finish()
    }
}
impl IURLSearchHook2 {
    pub unsafe fn Translate(&self, pwszsearchurl: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Translate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszsearchurl.as_ptr()), pwszsearchurl.len() as _).ok()
    }
}
impl ::core::default::Default for IURL_INVOKECOMMAND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IURL_INVOKECOMMAND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IURL_INVOKECOMMAND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IURL_SETURL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IURL_SETURL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IURL_SETURL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUniformResourceLocatorA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUniformResourceLocatorA {}
impl ::core::fmt::Debug for IUniformResourceLocatorA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUniformResourceLocatorA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUniformResourceLocatorW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUniformResourceLocatorW {}
impl ::core::fmt::Debug for IUniformResourceLocatorW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUniformResourceLocatorW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUpdateIDList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUpdateIDList {}
impl ::core::fmt::Debug for IUpdateIDList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateIDList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUseToBrowseItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUseToBrowseItem {}
impl ::core::fmt::Debug for IUseToBrowseItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUseToBrowseItem").field(&self.0).finish()
    }
}
impl IUseToBrowseItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUserAccountChangeCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserAccountChangeCallback {}
impl ::core::fmt::Debug for IUserAccountChangeCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserAccountChangeCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUserNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserNotification {}
impl ::core::fmt::Debug for IUserNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUserNotification2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserNotification2 {}
impl ::core::fmt::Debug for IUserNotification2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserNotification2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUserNotificationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserNotificationCallback {}
impl ::core::fmt::Debug for IUserNotificationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserNotificationCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewStateIdentityItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewStateIdentityItem {}
impl ::core::fmt::Debug for IViewStateIdentityItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewStateIdentityItem").field(&self.0).finish()
    }
}
impl IViewStateIdentityItem {
    #[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn GetItemIDList(&self) -> ::windows::core::Result<*mut Common::ITEMIDLIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemIDList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<IShellItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IVirtualDesktopManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualDesktopManager {}
impl ::core::fmt::Debug for IVirtualDesktopManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualDesktopManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVisualProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualProperties {}
impl ::core::fmt::Debug for IVisualProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebBrowser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebBrowser {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebBrowser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebBrowser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebBrowser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebBrowser2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebBrowser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebBrowser2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWebBrowser2 {
    pub unsafe fn GoBack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GoBack)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GoForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoHome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GoHome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoSearch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GoSearch)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Navigate(&self, url: &::windows::core::BSTR, flags: ::core::option::Option<*const super::super::System::Com::VARIANT>, targetframename: ::core::option::Option<*const super::super::System::Com::VARIANT>, postdata: ::core::option::Option<*const super::super::System::Com::VARIANT>, headers: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Navigate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(flags.unwrap_or(::std::ptr::null())), ::core::mem::transmute(targetframename.unwrap_or(::std::ptr::null())), ::core::mem::transmute(postdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(headers.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Refresh2(&self, level: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Refresh2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(level.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Container(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Container)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Document)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TopLevelContainer(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TopLevelContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Left(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Left)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLeft)(::windows::core::Vtable::as_raw(self), left).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Top)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTop)(::windows::core::Vtable::as_raw(self), top).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Width)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWidth)(::windows::core::Vtable::as_raw(self), width).ok()
    }
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Height)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHeight(&self, height: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHeight)(::windows::core::Vtable::as_raw(self), height).ok()
    }
    pub unsafe fn LocationName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.LocationName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocationURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.LocationURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Busy(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Busy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Quit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Quit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ClientToWindow(&self, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClientToWindow)(::windows::core::Vtable::as_raw(self), pcx, pcy).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutProperty(&self, property: &::windows::core::BSTR, vtvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(property), ::core::mem::transmute(vtvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, property: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(property), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HWND(&self) -> ::windows::core::Result<super::super::Foundation::SHANDLE_PTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HWND)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FullName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FullName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Visible)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisible<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVisible)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusBar(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusBar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusBar<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStatusBar)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StatusText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStatusText(&self, statustext: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStatusText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(statustext)).ok()
    }
    pub unsafe fn ToolBar(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ToolBar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetToolBar(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetToolBar)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MenuBar(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MenuBar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMenuBar<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMenuBar)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullScreen(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FullScreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullScreen<P0>(&self, bfullscreen: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFullScreen)(::windows::core::Vtable::as_raw(self), bfullscreen.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebBrowserApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebBrowserApp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebBrowserApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebBrowserApp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWebBrowserApp {
    pub unsafe fn GoBack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GoBack)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoForward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GoForward)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoHome(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GoHome)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GoSearch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GoSearch)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Navigate(&self, url: &::windows::core::BSTR, flags: ::core::option::Option<*const super::super::System::Com::VARIANT>, targetframename: ::core::option::Option<*const super::super::System::Com::VARIANT>, postdata: ::core::option::Option<*const super::super::System::Com::VARIANT>, headers: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Navigate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(url), ::core::mem::transmute(flags.unwrap_or(::std::ptr::null())), ::core::mem::transmute(targetframename.unwrap_or(::std::ptr::null())), ::core::mem::transmute(postdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(headers.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Refresh2(&self, level: ::core::option::Option<*const super::super::System::Com::VARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(level.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Application)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Container(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Container)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Document)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TopLevelContainer(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TopLevelContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Left(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Left)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLeft)(::windows::core::Vtable::as_raw(self), left).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Top)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTop)(::windows::core::Vtable::as_raw(self), top).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Width)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWidth)(::windows::core::Vtable::as_raw(self), width).ok()
    }
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Height)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHeight(&self, height: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHeight)(::windows::core::Vtable::as_raw(self), height).ok()
    }
    pub unsafe fn LocationName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocationName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocationURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocationURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Busy(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Busy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWebWizardExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebWizardExtension {}
impl ::core::fmt::Debug for IWebWizardExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebWizardExtension").field(&self.0).finish()
    }
}
impl IWebWizardExtension {
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPages(&self, apages: &mut [super::Controls::HPROPSHEETPAGE], pnpagesadded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddPages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(apages.as_ptr()), apages.len() as _, pnpagesadded).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetFirstPage(&self) -> ::windows::core::Result<super::Controls::HPROPSHEETPAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFirstPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetLastPage(&self) -> ::windows::core::Result<super::Controls::HPROPSHEETPAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebWizardHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebWizardHost {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebWizardHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebWizardHost").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebWizardHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebWizardHost2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebWizardHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebWizardHost2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWebWizardHost2 {
    pub unsafe fn FinalBack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinalBack)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FinalNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FinalNext)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetCaption(&self, bstrcaption: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCaption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaption)).ok()
    }
    pub unsafe fn Caption(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Caption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_Property(&self, bstrpropertyname: &::windows::core::BSTR, pvproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_Property)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), pvproperty).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Property(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Property)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpropertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWizardButtons<P0, P1, P2>(&self, vfenableback: P0, vfenablenext: P1, vflastpage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWizardButtons)(::windows::core::Vtable::as_raw(self), vfenableback.into(), vfenablenext.into(), vflastpage.into()).ok()
    }
    pub unsafe fn SetHeaderText(&self, bstrheadertitle: &::windows::core::BSTR, bstrheadersubtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHeaderText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrheadertitle), ::core::mem::transmute_copy(bstrheadersubtitle)).ok()
    }
}
impl ::core::cmp::PartialEq for IWizardExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWizardExtension {}
impl ::core::fmt::Debug for IWizardExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWizardExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWizardSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWizardSite {}
impl ::core::fmt::Debug for IWizardSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWizardSite").field(&self.0).finish()
    }
}
impl ::core::default::Default for KF_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KF_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KF_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KNOWNDESTCATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KNOWNDESTCATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KNOWNDESTCATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KNOWNFOLDER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KNOWNFOLDER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.pszName == other.pszName && self.pszDescription == other.pszDescription && self.fidParent == other.fidParent && self.pszRelativePath == other.pszRelativePath && self.pszParsingName == other.pszParsingName && self.pszTooltip == other.pszTooltip && self.pszLocalizedName == other.pszLocalizedName && self.pszIcon == other.pszIcon && self.pszSecurity == other.pszSecurity && self.dwAttributes == other.dwAttributes && self.kfdFlags == other.kfdFlags && self.ftidType == other.ftidType
    }
}
impl ::core::cmp::Eq for KNOWNFOLDER_DEFINITION {}
impl ::core::fmt::Debug for KNOWNFOLDER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KNOWNFOLDER_DEFINITION")
            .field("category", &self.category)
            .field("pszName", &self.pszName)
            .field("pszDescription", &self.pszDescription)
            .field("fidParent", &self.fidParent)
            .field("pszRelativePath", &self.pszRelativePath)
            .field("pszParsingName", &self.pszParsingName)
            .field("pszTooltip", &self.pszTooltip)
            .field("pszLocalizedName", &self.pszLocalizedName)
            .field("pszIcon", &self.pszIcon)
            .field("pszSecurity", &self.pszSecurity)
            .field("dwAttributes", &self.dwAttributes)
            .field("kfdFlags", &self.kfdFlags)
            .field("ftidType", &self.ftidType)
            .finish()
    }
}
impl ::core::default::Default for KNOWN_FOLDER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KNOWN_FOLDER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KNOWN_FOLDER_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIBRARYFOLDERFILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIBRARYFOLDERFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBRARYFOLDERFILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIBRARYMANAGEDIALOGOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIBRARYMANAGEDIALOGOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBRARYMANAGEDIALOGOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIBRARYOPTIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIBRARYOPTIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBRARYOPTIONFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIBRARYSAVEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIBRARYSAVEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBRARYSAVEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MENUBANDHANDLERCID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUBANDHANDLERCID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUBANDHANDLERCID").field(&self.0).finish()
    }
}
impl ::core::default::Default for MENUPOPUPPOPUPFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUPOPUPPOPUPFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUPOPUPPOPUPFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MENUPOPUPSELECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUPOPUPSELECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUPOPUPSELECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MERGE_UPDATE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MERGE_UPDATE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MERGE_UPDATE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIMEASSOCIATIONDIALOG_IN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIMEASSOCIATIONDIALOG_IN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIMEASSOCIATIONDIALOG_IN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MONITOR_APP_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONITOR_APP_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_APP_VISIBILITY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MULTIKEYHELPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MULTIKEYHELPA {
    fn eq(&self, other: &Self) -> bool {
        self.mkSize == other.mkSize && self.mkKeylist == other.mkKeylist && self.szKeyphrase == other.szKeyphrase
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MULTIKEYHELPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MULTIKEYHELPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTIKEYHELPA").field("mkSize", &self.mkSize).field("mkKeylist", &self.mkKeylist).field("szKeyphrase", &self.szKeyphrase).finish()
    }
}
impl ::core::default::Default for MULTIKEYHELPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MULTIKEYHELPW {
    fn eq(&self, other: &Self) -> bool {
        self.mkSize == other.mkSize && self.mkKeylist == other.mkKeylist && self.szKeyphrase == other.szKeyphrase
    }
}
impl ::core::cmp::Eq for MULTIKEYHELPW {}
impl ::core::fmt::Debug for MULTIKEYHELPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTIKEYHELPW").field("mkSize", &self.mkSize).field("mkKeylist", &self.mkKeylist).field("szKeyphrase", &self.szKeyphrase).finish()
    }
}
impl ::core::default::Default for NAMESPACEWALKFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAMESPACEWALKFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAMESPACEWALKFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for NATIVE_DISPLAY_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NATIVE_DISPLAY_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NATIVE_DISPLAY_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for NC_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NC_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.pAddrInfo == other.pAddrInfo && self.PortNumber == other.PortNumber && self.PrefixLength == other.PrefixLength
    }
}
impl ::core::cmp::Eq for NC_ADDRESS {}
impl ::core::fmt::Debug for NC_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NC_ADDRESS").field("pAddrInfo", &self.pAddrInfo).field("PortNumber", &self.PortNumber).field("PrefixLength", &self.PrefixLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for NEWCPLINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for NEWCPLINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NOTIFY_ICON_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFY_ICON_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_ICON_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NOTIFY_ICON_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NOTIFY_ICON_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NOTIFY_ICON_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NOTIFY_ICON_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NOTIFY_ICON_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NOTIFY_ICON_INFOTIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFY_ICON_INFOTIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_ICON_INFOTIP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NOTIFY_ICON_MESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFY_ICON_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_ICON_MESSAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NOTIFY_ICON_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NOTIFY_ICON_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_ICON_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl ::core::default::Default for NRESARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl ::core::cmp::PartialEq for NRESARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.nr == other.nr
    }
}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl ::core::cmp::Eq for NRESARRAY {}
#[cfg(feature = "Win32_NetworkManagement_WNet")]
impl ::core::fmt::Debug for NRESARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NRESARRAY").field("cItems", &self.cItems).field("nr", &self.nr).finish()
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::default::Default for NSTCCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::PartialEq for NSTCCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.psi == other.psi && self.uItemState == other.uItemState && self.nstcis == other.nstcis && self.pszText == other.pszText && self.iImage == other.iImage && self.himl == other.himl && self.iLevel == other.iLevel && self.iIndent == other.iIndent
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::Eq for NSTCCUSTOMDRAW {}
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::fmt::Debug for NSTCCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NSTCCUSTOMDRAW").field("psi", &self.psi).field("uItemState", &self.uItemState).field("nstcis", &self.nstcis).field("pszText", &self.pszText).field("iImage", &self.iImage).field("himl", &self.himl).field("iLevel", &self.iLevel).field("iIndent", &self.iIndent).finish()
    }
}
impl ::core::default::Default for NSTCFOLDERCAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NSTCFOLDERCAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NSTCFOLDERCAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NSTCGNI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NSTCGNI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NSTCGNI").field(&self.0).finish()
    }
}
impl ::core::default::Default for NSTCSTYLE2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NSTCSTYLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NSTCSTYLE2").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::default::Default for NT_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NT_FE_CONSOLE_PROPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NWMF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NWMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NWMF").field(&self.0).finish()
    }
}
impl ::core::default::Default for NewProcessCauseConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NewProcessCauseConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NewProcessCauseConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPENASINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPENASINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pcszFile == other.pcszFile && self.pcszClass == other.pcszClass && self.oaifInFlags == other.oaifInFlags
    }
}
impl ::core::cmp::Eq for OPENASINFO {}
impl ::core::fmt::Debug for OPENASINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENASINFO").field("pcszFile", &self.pcszFile).field("pcszClass", &self.pcszClass).field("oaifInFlags", &self.oaifInFlags).finish()
    }
}
impl ::core::default::Default for OPEN_AS_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPEN_AS_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_AS_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPEN_AS_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPEN_AS_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPEN_AS_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPEN_AS_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPEN_AS_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OfflineFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OfflineFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineFolderStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for PACKAGE_EXECUTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PACKAGE_EXECUTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PACKAGE_EXECUTION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PARSEDURLA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARSEDURLA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszProtocol == other.pszProtocol && self.cchProtocol == other.cchProtocol && self.pszSuffix == other.pszSuffix && self.cchSuffix == other.cchSuffix && self.nScheme == other.nScheme
    }
}
impl ::core::cmp::Eq for PARSEDURLA {}
impl ::core::fmt::Debug for PARSEDURLA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARSEDURLA").field("cbSize", &self.cbSize).field("pszProtocol", &self.pszProtocol).field("cchProtocol", &self.cchProtocol).field("pszSuffix", &self.pszSuffix).field("cchSuffix", &self.cchSuffix).field("nScheme", &self.nScheme).finish()
    }
}
impl ::core::default::Default for PARSEDURLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARSEDURLW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszProtocol == other.pszProtocol && self.cchProtocol == other.cchProtocol && self.pszSuffix == other.pszSuffix && self.cchSuffix == other.cchSuffix && self.nScheme == other.nScheme
    }
}
impl ::core::cmp::Eq for PARSEDURLW {}
impl ::core::fmt::Debug for PARSEDURLW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARSEDURLW").field("cbSize", &self.cbSize).field("pszProtocol", &self.pszProtocol).field("cchProtocol", &self.cchProtocol).field("pszSuffix", &self.pszSuffix).field("cchSuffix", &self.cchSuffix).field("nScheme", &self.nScheme).finish()
    }
}
impl ::core::default::Default for PATHCCH_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PATHCCH_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PATHCCH_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PATHCCH_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PATHCCH_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PATHCCH_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PATHCCH_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PATHCCH_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PCS_RET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PCS_RET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PCS_RET").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PCS_RET {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PCS_RET {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PCS_RET {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PCS_RET {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PCS_RET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::default::Default for PERSIST_FOLDER_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::PartialEq for PERSIST_FOLDER_TARGET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pidlTargetFolder == other.pidlTargetFolder && self.szTargetParsingName == other.szTargetParsingName && self.szNetworkProvider == other.szNetworkProvider && self.dwAttributes == other.dwAttributes && self.csidl == other.csidl
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::Eq for PERSIST_FOLDER_TARGET_INFO {}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::fmt::Debug for PERSIST_FOLDER_TARGET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSIST_FOLDER_TARGET_INFO").field("pidlTargetFolder", &self.pidlTargetFolder).field("szTargetParsingName", &self.szTargetParsingName).field("szNetworkProvider", &self.szNetworkProvider).field("dwAttributes", &self.dwAttributes).field("csidl", &self.csidl).finish()
    }
}
impl ::core::default::Default for PIDISF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIDISF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDISF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PIDISF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PIDISF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PIDISF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PIDISF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PIDISF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PIDISM_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIDISM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDISM_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PIDISR_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIDISR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDISR_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for PID_INTSITE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PID_INTSITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PID_INTSITE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PID_IS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PID_IS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PID_IS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for PREVIEWHANDLERFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for PREVIEWHANDLERFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.haccel == other.haccel && self.cAccelEntries == other.cAccelEntries
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for PREVIEWHANDLERFRAMEINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for PREVIEWHANDLERFRAMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PREVIEWHANDLERFRAMEINFO").field("haccel", &self.haccel).field("cAccelEntries", &self.cAccelEntries).finish()
    }
}
impl ::core::default::Default for PRF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROFILEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROFILEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.lpUserName == other.lpUserName && self.lpProfilePath == other.lpProfilePath && self.lpDefaultPath == other.lpDefaultPath && self.lpServerName == other.lpServerName && self.lpPolicyPath == other.lpPolicyPath && self.hProfile == other.hProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROFILEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILEINFOA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpUserName", &self.lpUserName).field("lpProfilePath", &self.lpProfilePath).field("lpDefaultPath", &self.lpDefaultPath).field("lpServerName", &self.lpServerName).field("lpPolicyPath", &self.lpPolicyPath).field("hProfile", &self.hProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROFILEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROFILEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.lpUserName == other.lpUserName && self.lpProfilePath == other.lpProfilePath && self.lpDefaultPath == other.lpDefaultPath && self.lpServerName == other.lpServerName && self.lpPolicyPath == other.lpPolicyPath && self.hProfile == other.hProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROFILEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROFILEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILEINFOW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpUserName", &self.lpUserName).field("lpProfilePath", &self.lpProfilePath).field("lpDefaultPath", &self.lpDefaultPath).field("lpServerName", &self.lpServerName).field("lpPolicyPath", &self.lpPolicyPath).field("hProfile", &self.hProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUBAPPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUBAPPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.pszSource == other.pszSource && self.stAssigned == other.stAssigned && self.stPublished == other.stPublished && self.stScheduled == other.stScheduled && self.stExpire == other.stExpire
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUBAPPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PUBAPPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBAPPINFO").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("pszSource", &self.pszSource).field("stAssigned", &self.stAssigned).field("stPublished", &self.stPublished).field("stScheduled", &self.stScheduled).field("stExpire", &self.stExpire).finish()
    }
}
impl ::core::default::Default for PUBAPPINFOFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PUBAPPINFOFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUBAPPINFOFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for QCMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for QCMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hmenu == other.hmenu && self.indexMenu == other.indexMenu && self.idCmdFirst == other.idCmdFirst && self.idCmdLast == other.idCmdLast && self.pIdMap == other.pIdMap
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for QCMINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for QCMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QCMINFO").field("hmenu", &self.hmenu).field("indexMenu", &self.indexMenu).field("idCmdFirst", &self.idCmdFirst).field("idCmdLast", &self.idCmdLast).field("pIdMap", &self.pIdMap).finish()
    }
}
impl ::core::default::Default for QCMINFO_IDMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QCMINFO_IDMAP {
    fn eq(&self, other: &Self) -> bool {
        self.nMaxIds == other.nMaxIds && self.pIdList == other.pIdList
    }
}
impl ::core::cmp::Eq for QCMINFO_IDMAP {}
impl ::core::fmt::Debug for QCMINFO_IDMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QCMINFO_IDMAP").field("nMaxIds", &self.nMaxIds).field("pIdList", &self.pIdList).finish()
    }
}
impl ::core::default::Default for QCMINFO_IDMAP_PLACEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QCMINFO_IDMAP_PLACEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.fFlags == other.fFlags
    }
}
impl ::core::cmp::Eq for QCMINFO_IDMAP_PLACEMENT {}
impl ::core::fmt::Debug for QCMINFO_IDMAP_PLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QCMINFO_IDMAP_PLACEMENT").field("id", &self.id).field("fFlags", &self.fFlags).finish()
    }
}
impl ::core::default::Default for QITAB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QITAB {
    fn eq(&self, other: &Self) -> bool {
        self.piid == other.piid && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for QITAB {}
impl ::core::fmt::Debug for QITAB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QITAB").field("piid", &self.piid).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::core::default::Default for QITIPF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QITIPF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QITIPF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for QITIPF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QITIPF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QITIPF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QITIPF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QITIPF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for QUERY_USER_NOTIFICATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUERY_USER_NOTIFICATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERY_USER_NOTIFICATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESTRICTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTRICTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RefreshConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RefreshConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RefreshConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCALE_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCALE_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCALE_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCALE_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCALE_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCALE_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCALE_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCALE_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SCNRT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCNRT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCNRT_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECURELOCKCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURELOCKCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURELOCKCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SFBS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SFBS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SFBS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SFVM_HELPTOPIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SFVM_HELPTOPIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wszHelpFile == other.wszHelpFile && self.wszHelpTopic == other.wszHelpTopic
    }
}
impl ::core::cmp::Eq for SFVM_HELPTOPIC_DATA {}
impl ::core::fmt::Debug for SFVM_HELPTOPIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SFVM_HELPTOPIC_DATA").field("wszHelpFile", &self.wszHelpFile).field("wszHelpTopic", &self.wszHelpTopic).finish()
    }
}
impl ::core::default::Default for SFVM_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SFVM_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SFVM_MESSAGE_ID").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for SFVM_PROPPAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SFVS_SELECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SFVS_SELECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SFVS_SELECT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for SFV_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for SFV_CREATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pshf == other.pshf && self.psvOuter == other.psvOuter && self.psfvcb == other.psfvcb
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for SFV_CREATE {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for SFV_CREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SFV_CREATE").field("cbSize", &self.cbSize).field("pshf", &self.pshf).field("psvOuter", &self.psvOuter).field("psfvcb", &self.psfvcb).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for SFV_SETITEMPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::PartialEq for SFV_SETITEMPOS {
    fn eq(&self, other: &Self) -> bool {
        self.pidl == other.pidl && self.pt == other.pt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::cmp::Eq for SFV_SETITEMPOS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::fmt::Debug for SFV_SETITEMPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SFV_SETITEMPOS").field("pidl", &self.pidl).field("pt", &self.pt).finish()
    }
}
impl ::core::default::Default for SHARD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARD").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHARDAPPIDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::default::Default for SHARDAPPIDINFOIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHARDAPPIDINFOLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHARE_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARE_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHCNE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHCNE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCNE_ID").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHCNE_ID {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHCNE_ID {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHCNE_ID {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHCNE_ID {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHCNE_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHCNF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHCNF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCNF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHCNF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHCNF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHCNF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHCNF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHCNF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHCNRF_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHCNRF_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCNRF_SOURCE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHCNRF_SOURCE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHCNRF_SOURCE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHCNRF_SOURCE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHCNRF_SOURCE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHCNRF_SOURCE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHCOLUMNDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwFileAttributes == other.dwFileAttributes && self.dwReserved == other.dwReserved && self.pwszExt == other.pwszExt && self.wszFile == other.wszFile
    }
}
impl ::core::cmp::Eq for SHCOLUMNDATA {}
impl ::core::fmt::Debug for SHCOLUMNDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHCOLUMNDATA").field("dwFlags", &self.dwFlags).field("dwFileAttributes", &self.dwFileAttributes).field("dwReserved", &self.dwReserved).field("pwszExt", &self.pwszExt).field("wszFile", &self.wszFile).finish()
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHCOLUMNINIT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved && self.wszFolder == other.wszFolder
    }
}
impl ::core::cmp::Eq for SHCOLUMNINIT {}
impl ::core::fmt::Debug for SHCOLUMNINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHCOLUMNINIT").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).field("wszFolder", &self.wszFolder).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl ::core::default::Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
impl ::core::default::Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHChangeDWORDAsIDList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
impl ::core::default::Default for SHChangeNotifyEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHChangeUpdateImageIDList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHDESCRIPTIONID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHDESCRIPTIONID {
    fn eq(&self, other: &Self) -> bool {
        self.dwDescriptionId == other.dwDescriptionId && self.clsid == other.clsid
    }
}
impl ::core::cmp::Eq for SHDESCRIPTIONID {}
impl ::core::fmt::Debug for SHDESCRIPTIONID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHDESCRIPTIONID").field("dwDescriptionId", &self.dwDescriptionId).field("clsid", &self.clsid).finish()
    }
}
impl ::core::default::Default for SHDID_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHDID_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHDID_ID").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for SHDRAGIMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for SHDRAGIMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sizeDragImage == other.sizeDragImage && self.ptOffset == other.ptOffset && self.hbmpDragImage == other.hbmpDragImage && self.crColorKey == other.crColorKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for SHDRAGIMAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for SHDRAGIMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHDRAGIMAGE").field("sizeDragImage", &self.sizeDragImage).field("ptOffset", &self.ptOffset).field("hbmpDragImage", &self.hbmpDragImage).field("crColorKey", &self.crColorKey).finish()
    }
}
impl ::core::default::Default for SHELLBROWSERSHOWCONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHELLBROWSERSHOWCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHELLBROWSERSHOWCONTROL").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHELLFLAGSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHELLSTATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHELLSTATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHELL_AUTOCOMPLETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHELL_AUTOCOMPLETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHELL_AUTOCOMPLETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHELL_AUTOCOMPLETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHELL_AUTOCOMPLETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHELL_AUTOCOMPLETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHELL_AUTOCOMPLETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHELL_AUTOCOMPLETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHELL_ITEM_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHELL_ITEM_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.guidType == other.guidType && self.szName == other.szName
    }
}
impl ::core::cmp::Eq for SHELL_ITEM_RESOURCE {}
impl ::core::fmt::Debug for SHELL_ITEM_RESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHELL_ITEM_RESOURCE").field("guidType", &self.guidType).field("szName", &self.szName).finish()
    }
}
impl ::core::default::Default for SHELL_LINK_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHELL_LINK_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHELL_LINK_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHELL_UI_COMPONENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHELL_UI_COMPONENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHELL_UI_COMPONENT").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHFMT_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHFMT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHFMT_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHFMT_OPT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHFMT_OPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHFMT_OPT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHFMT_OPT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHFMT_OPT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHFMT_OPT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHFMT_OPT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHFMT_OPT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHFMT_RET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHFMT_RET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHFMT_RET").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHFOLDERCUSTOMSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHFOLDERCUSTOMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.pvid == other.pvid && self.pszWebViewTemplate == other.pszWebViewTemplate && self.cchWebViewTemplate == other.cchWebViewTemplate && self.pszWebViewTemplateVersion == other.pszWebViewTemplateVersion && self.pszInfoTip == other.pszInfoTip && self.cchInfoTip == other.cchInfoTip && self.pclsid == other.pclsid && self.dwFlags == other.dwFlags && self.pszIconFile == other.pszIconFile && self.cchIconFile == other.cchIconFile && self.iIconIndex == other.iIconIndex && self.pszLogo == other.pszLogo && self.cchLogo == other.cchLogo
    }
}
impl ::core::cmp::Eq for SHFOLDERCUSTOMSETTINGS {}
impl ::core::fmt::Debug for SHFOLDERCUSTOMSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHFOLDERCUSTOMSETTINGS")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("pvid", &self.pvid)
            .field("pszWebViewTemplate", &self.pszWebViewTemplate)
            .field("cchWebViewTemplate", &self.cchWebViewTemplate)
            .field("pszWebViewTemplateVersion", &self.pszWebViewTemplateVersion)
            .field("pszInfoTip", &self.pszInfoTip)
            .field("cchInfoTip", &self.cchInfoTip)
            .field("pclsid", &self.pclsid)
            .field("dwFlags", &self.dwFlags)
            .field("pszIconFile", &self.pszIconFile)
            .field("cchIconFile", &self.cchIconFile)
            .field("iIconIndex", &self.iIconIndex)
            .field("pszLogo", &self.pszLogo)
            .field("cchLogo", &self.cchLogo)
            .finish()
    }
}
impl ::core::default::Default for SHGDFIL_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGDFIL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGDFIL_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHGDNF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGDNF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGDNF").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHGFI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGFI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGFI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHGFI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHGFI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHGFI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHGFI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHGFI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHGFP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGFP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGFP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHGLOBALCOUNTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGLOBALCOUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGLOBALCOUNTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHGSI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHGSI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHGSI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHGSI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHGSI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHGSI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHGSI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHGSI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHOP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHOP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHOP_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHOP_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHOP_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHOP_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHOP_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHOP_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SHQUERYRBINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SHQUERYRBINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHREGDEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHREGDEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHREGDEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHREGENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHREGENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHREGENUM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHSTOCKICONID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHSTOCKICONID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHSTOCKICONID").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIATTRIBFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIATTRIBFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIATTRIBFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SIGDN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIGDN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIGDN").field(&self.0).finish()
    }
}
impl ::core::default::Default for SIIGBF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIIGBF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIIGBF").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SIIGBF {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SIIGBF {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SIIGBF {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SIIGBF {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SIIGBF {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SLGP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLGP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLGP_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SLOWAPPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SLOWAPPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ullSize == other.ullSize && self.ftLastUsed == other.ftLastUsed && self.iTimesUsed == other.iTimesUsed && self.pszImage == other.pszImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SLOWAPPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SLOWAPPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLOWAPPINFO").field("ullSize", &self.ullSize).field("ftLastUsed", &self.ftLastUsed).field("iTimesUsed", &self.iTimesUsed).field("pszImage", &self.pszImage).finish()
    }
}
impl ::core::default::Default for SLR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLR_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::default::Default for SMCSHCHANGENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::PartialEq for SMCSHCHANGENOTIFYSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lEvent == other.lEvent && self.pidl1 == other.pidl1 && self.pidl2 == other.pidl2
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::Eq for SMCSHCHANGENOTIFYSTRUCT {}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::fmt::Debug for SMCSHCHANGENOTIFYSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMCSHCHANGENOTIFYSTRUCT").field("lEvent", &self.lEvent).field("pidl1", &self.pidl1).field("pidl2", &self.pidl2).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for SMDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for SMDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.dwFlags == other.dwFlags && self.hmenu == other.hmenu && self.hwnd == other.hwnd && self.uId == other.uId && self.uIdParent == other.uIdParent && self.uIdAncestor == other.uIdAncestor && self.punk == other.punk && self.pidlFolder == other.pidlFolder && self.pidlItem == other.pidlItem && self.psf == other.psf && self.pvUserData == other.pvUserData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for SMDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for SMDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMDATA").field("dwMask", &self.dwMask).field("dwFlags", &self.dwFlags).field("hmenu", &self.hmenu).field("hwnd", &self.hwnd).field("uId", &self.uId).field("uIdParent", &self.uIdParent).field("uIdAncestor", &self.uIdAncestor).field("punk", &self.punk).field("pidlFolder", &self.pidlFolder).field("pidlItem", &self.pidlItem).field("psf", &self.psf).field("pvUserData", &self.pvUserData).finish()
    }
}
impl ::core::default::Default for SMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.iIcon == other.iIcon
    }
}
impl ::core::cmp::Eq for SMINFO {}
impl ::core::fmt::Debug for SMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMINFO").field("dwMask", &self.dwMask).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).field("iIcon", &self.iIcon).finish()
    }
}
impl ::core::default::Default for SMINFOFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMINFOFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMINFOFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMINFOMASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMINFOMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMINFOMASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMINFOTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMINFOTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMINFOTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for SORTCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for SORTCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.propkey == other.propkey && self.direction == other.direction
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for SORTCOLUMN {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for SORTCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SORTCOLUMN").field("propkey", &self.propkey).field("direction", &self.direction).finish()
    }
}
impl ::core::default::Default for SORTDIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SORTDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SORTDIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SORT_ORDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SORT_ORDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SORT_ORDER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPTEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SSF_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SSF_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SSF_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SSF_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SSF_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SSF_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SSF_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SSF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for STGOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STGOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PROVIDER_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROVIDER_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROVIDER_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STPFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STPFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STPFLAG").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SV2CVW2_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for SV2CVW2_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.psvPrev == other.psvPrev && self.pfs == other.pfs && self.psbOwner == other.psbOwner && self.prcView == other.prcView && self.pvid == other.pvid && self.hwndView == other.hwndView
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for SV2CVW2_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for SV2CVW2_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SV2CVW2_PARAMS").field("cbSize", &self.cbSize).field("psvPrev", &self.psvPrev).field("pfs", &self.pfs).field("psbOwner", &self.psbOwner).field("prcView", &self.prcView).field("pvid", &self.pvid).field("hwndView", &self.hwndView).finish()
    }
}
impl ::core::default::Default for SVUIA_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SVUIA_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SVUIA_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRERRORFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRERRORFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRERRORFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRHANDLERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRHANDLERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRHANDLERFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for SYNCMGRHANDLERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for SYNCMGRHANDLERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hIcon == other.hIcon && self.SyncMgrHandlerFlags == other.SyncMgrHandlerFlags && self.wszHandlerName == other.wszHandlerName
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for SYNCMGRHANDLERINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for SYNCMGRHANDLERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNCMGRHANDLERINFO").field("cbSize", &self.cbSize).field("hIcon", &self.hIcon).field("SyncMgrHandlerFlags", &self.SyncMgrHandlerFlags).field("wszHandlerName", &self.wszHandlerName).finish()
    }
}
impl ::core::default::Default for SYNCMGRINVOKEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRINVOKEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRINVOKEFLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for SYNCMGRITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for SYNCMGRITEM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.ItemID == other.ItemID && self.dwItemState == other.dwItemState && self.hIcon == other.hIcon && self.wszItemName == other.wszItemName && self.ftLastUpdate == other.ftLastUpdate
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for SYNCMGRITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for SYNCMGRITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNCMGRITEM").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("ItemID", &self.ItemID).field("dwItemState", &self.dwItemState).field("hIcon", &self.hIcon).field("wszItemName", &self.wszItemName).field("ftLastUpdate", &self.ftLastUpdate).finish()
    }
}
impl ::core::default::Default for SYNCMGRITEMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRITEMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRITEMFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRITEMSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRITEMSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRITEMSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRLOGERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYNCMGRLOGERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.mask == other.mask && self.dwSyncMgrErrorFlags == other.dwSyncMgrErrorFlags && self.ErrorID == other.ErrorID && self.ItemID == other.ItemID
    }
}
impl ::core::cmp::Eq for SYNCMGRLOGERRORINFO {}
impl ::core::fmt::Debug for SYNCMGRLOGERRORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNCMGRLOGERRORINFO").field("cbSize", &self.cbSize).field("mask", &self.mask).field("dwSyncMgrErrorFlags", &self.dwSyncMgrErrorFlags).field("ErrorID", &self.ErrorID).field("ItemID", &self.ItemID).finish()
    }
}
impl ::core::default::Default for SYNCMGRLOGLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRLOGLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRLOGLEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRPROGRESSITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYNCMGRPROGRESSITEM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.mask == other.mask && self.lpcStatusText == other.lpcStatusText && self.dwStatusType == other.dwStatusType && self.iProgValue == other.iProgValue && self.iMaxValue == other.iMaxValue
    }
}
impl ::core::cmp::Eq for SYNCMGRPROGRESSITEM {}
impl ::core::fmt::Debug for SYNCMGRPROGRESSITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNCMGRPROGRESSITEM").field("cbSize", &self.cbSize).field("mask", &self.mask).field("lpcStatusText", &self.lpcStatusText).field("dwStatusType", &self.dwStatusType).field("iProgValue", &self.iProgValue).field("iMaxValue", &self.iMaxValue).finish()
    }
}
impl ::core::default::Default for SYNCMGRREGISTERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRREGISTERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRREGISTERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGRSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGRSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGRSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_CANCEL_REQUEST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_CANCEL_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_CANCEL_REQUEST").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SYNCMGR_CONFLICT_ID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SYNCMGR_CONFLICT_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pblobID == other.pblobID && self.pblobExtra == other.pblobExtra
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SYNCMGR_CONFLICT_ID_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SYNCMGR_CONFLICT_ID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNCMGR_CONFLICT_ID_INFO").field("pblobID", &self.pblobID).field("pblobExtra", &self.pblobExtra).finish()
    }
}
impl ::core::default::Default for SYNCMGR_CONFLICT_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_CONFLICT_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_CONFLICT_ITEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_EVENT_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_EVENT_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_EVENT_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_HANDLER_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_HANDLER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_HANDLER_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_HANDLER_POLICIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_HANDLER_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_HANDLER_POLICIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_HANDLER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_HANDLER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_HANDLER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_ITEM_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_ITEM_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_ITEM_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_ITEM_POLICIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_ITEM_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_ITEM_POLICIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_PRESENTER_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_PRESENTER_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_PRESENTER_CHOICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_PRESENTER_NEXT_STEP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_PRESENTER_NEXT_STEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_PRESENTER_NEXT_STEP").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_PROGRESS_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_PROGRESS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_PROGRESS_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_RESOLUTION_ABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_RESOLUTION_ABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_RESOLUTION_ABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_RESOLUTION_FEEDBACK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_RESOLUTION_FEEDBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_RESOLUTION_FEEDBACK").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_SYNC_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_SYNC_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_SYNC_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYNCMGR_UPDATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCMGR_UPDATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCMGR_UPDATE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for SecureLockIconConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SecureLockIconConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecureLockIconConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for ShellFolderViewOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ShellFolderViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShellFolderViewOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for ShellSpecialFolderConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ShellSpecialFolderConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShellSpecialFolderConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for ShellWindowFindWindowOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ShellWindowFindWindowOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShellWindowFindWindowOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for ShellWindowTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ShellWindowTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShellWindowTypeConstants").field(&self.0).finish()
    }
}
impl ::core::default::Default for TBINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbuttons == other.cbuttons && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for TBINFO {}
impl ::core::fmt::Debug for TBINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBINFO").field("cbuttons", &self.cbuttons).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for TBPFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TBPFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBPFLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for THUMBBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for THUMBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.dwMask == other.dwMask && self.iId == other.iId && self.iBitmap == other.iBitmap && self.hIcon == other.hIcon && self.szTip == other.szTip && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for THUMBBUTTON {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for THUMBBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THUMBBUTTON").field("dwMask", &self.dwMask).field("iId", &self.iId).field("iBitmap", &self.iBitmap).field("hIcon", &self.hIcon).field("szTip", &self.szTip).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for THUMBBUTTONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBBUTTONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBBUTTONFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBBUTTONMASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBBUTTONMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBBUTTONMASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for TI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TLENUMF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TLENUMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TLENUMF").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TOOLBARITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TOOLBARITEM {
    fn eq(&self, other: &Self) -> bool {
        self.ptbar == other.ptbar && self.rcBorderTool == other.rcBorderTool && self.pwszItem == other.pwszItem && self.fShow == other.fShow && self.hMon == other.hMon
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TOOLBARITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for TOOLBARITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOOLBARITEM").field("ptbar", &self.ptbar).field("rcBorderTool", &self.rcBorderTool).field("pwszItem", &self.pwszItem).field("fShow", &self.fShow).field("hMon", &self.hMon).finish()
    }
}
impl ::core::default::Default for TRANSLATEURL_IN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSLATEURL_IN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSLATEURL_IN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ThumbnailStreamCacheOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ThumbnailStreamCacheOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailStreamCacheOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNDOCK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNDOCK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNDOCK_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for URLASSOCIATIONDIALOG_IN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URLASSOCIATIONDIALOG_IN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLASSOCIATIONDIALOG_IN_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URLINVOKECOMMANDINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URLINVOKECOMMANDINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwcbSize == other.dwcbSize && self.dwFlags == other.dwFlags && self.hwndParent == other.hwndParent && self.pcszVerb == other.pcszVerb
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URLINVOKECOMMANDINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URLINVOKECOMMANDINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URLINVOKECOMMANDINFOA").field("dwcbSize", &self.dwcbSize).field("dwFlags", &self.dwFlags).field("hwndParent", &self.hwndParent).field("pcszVerb", &self.pcszVerb).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URLINVOKECOMMANDINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URLINVOKECOMMANDINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwcbSize == other.dwcbSize && self.dwFlags == other.dwFlags && self.hwndParent == other.hwndParent && self.pcszVerb == other.pcszVerb
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URLINVOKECOMMANDINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URLINVOKECOMMANDINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("URLINVOKECOMMANDINFOW").field("dwcbSize", &self.dwcbSize).field("dwFlags", &self.dwFlags).field("hwndParent", &self.hwndParent).field("pcszVerb", &self.pcszVerb).finish()
    }
}
impl ::core::default::Default for URLIS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URLIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLIS").field(&self.0).finish()
    }
}
impl ::core::default::Default for URL_PART {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URL_PART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_PART").field(&self.0).finish()
    }
}
impl ::core::default::Default for URL_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URL_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_SCHEME").field(&self.0).finish()
    }
}
impl ::core::default::Default for VALIDATEUNC_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VALIDATEUNC_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALIDATEUNC_OPTION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VALIDATEUNC_OPTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VALIDATEUNC_OPTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VALIDATEUNC_OPTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VALIDATEUNC_OPTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VALIDATEUNC_OPTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for VPCOLORFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VPCOLORFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VPCOLORFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VPWATERMARKFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VPWATERMARKFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VPWATERMARKFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::default::Default for WINDOWDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::PartialEq for WINDOWDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwWindowID == other.dwWindowID && self.uiCP == other.uiCP && self.pidl == other.pidl && self.lpszUrl == other.lpszUrl && self.lpszUrlLocation == other.lpszUrlLocation && self.lpszTitle == other.lpszTitle
    }
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::cmp::Eq for WINDOWDATA {}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl ::core::fmt::Debug for WINDOWDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWDATA").field("dwWindowID", &self.dwWindowID).field("uiCP", &self.uiCP).field("pidl", &self.pidl).field("lpszUrl", &self.lpszUrl).field("lpszUrlLocation", &self.lpszUrlLocation).field("lpszTitle", &self.lpszTitle).finish()
    }
}
impl ::core::default::Default for WTS_ALPHATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_ALPHATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_ALPHATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_CACHEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CACHEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CACHEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_CONTEXTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CONTEXTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONTEXTFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_THUMBNAILID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_THUMBNAILID {
    fn eq(&self, other: &Self) -> bool {
        self.rgbKey == other.rgbKey
    }
}
impl ::core::cmp::Eq for WTS_THUMBNAILID {}
impl ::core::fmt::Debug for WTS_THUMBNAILID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_THUMBNAILID").field("rgbKey", &self.rgbKey).finish()
    }
}
impl ::core::default::Default for _BROWSERFRAMEOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _BROWSERFRAMEOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_BROWSERFRAMEOPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _CDBE_ACTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _CDBE_ACTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_CDBE_ACTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _EXPCMDFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _EXPCMDFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EXPCMDFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _EXPCMDSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _EXPCMDSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EXPCMDSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _EXPLORERPANESTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _EXPLORERPANESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EXPLORERPANESTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _EXPPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _EXPPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EXPPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _KF_DEFINITION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _KF_DEFINITION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_KF_DEFINITION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _KF_REDIRECTION_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _KF_REDIRECTION_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_KF_REDIRECTION_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for _KF_REDIRECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _KF_REDIRECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_KF_REDIRECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NMCII_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NMCII_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NMCII_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NMCSAEI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NMCSAEI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NMCSAEI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NSTCECLICKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NSTCECLICKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NSTCECLICKTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NSTCEHITTEST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NSTCEHITTEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NSTCEHITTEST").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NSTCITEMSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NSTCITEMSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NSTCITEMSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NSTCROOTSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NSTCROOTSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NSTCROOTSTYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _NSTCSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _NSTCSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_NSTCSTYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _OPPROGDLGF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _OPPROGDLGF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_OPPROGDLGF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _PDMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _PDMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_PDMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SHCONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SHCONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SHCONTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SICHINTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SICHINTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SICHINTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SPBEGINF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SPBEGINF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SPBEGINF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SPINITF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SPINITF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SPINITF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SV3CVW3_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SV3CVW3_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SV3CVW3_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SVGIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SVGIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SVGIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for _SVSIF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _SVSIF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_SVSIF").field(&self.0).finish()
    }
}
impl ::core::default::Default for _TRANSFER_ADVISE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _TRANSFER_ADVISE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_TRANSFER_ADVISE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for _TRANSFER_SOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _TRANSFER_SOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_TRANSFER_SOURCE_FLAGS").field(&self.0).finish()
    }
}
