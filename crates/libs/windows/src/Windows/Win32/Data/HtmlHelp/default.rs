impl ::core::default::Default for COLUMNSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLUMNSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cPropCount == other.cPropCount && self.cPropsLoaded == other.cPropsLoaded
    }
}
impl ::core::cmp::Eq for COLUMNSTATUS {}
impl ::core::fmt::Debug for COLUMNSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLUMNSTATUS").field("cPropCount", &self.cPropCount).field("cPropsLoaded", &self.cPropsLoaded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHNTRACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHNTRACK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszCurUrl == other.pszCurUrl && self.idAction == other.idAction && self.phhWinType == other.phhWinType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHNTRACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HHNTRACK").field("hdr", &self.hdr).field("pszCurUrl", &self.pszCurUrl).field("idAction", &self.idAction).field("phhWinType", &self.phhWinType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHN_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHN_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszUrl == other.pszUrl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHN_NOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HHN_NOTIFY").field("hdr", &self.hdr).field("pszUrl", &self.pszUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_AKLINK {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fReserved == other.fReserved && self.pszKeywords == other.pszKeywords && self.pszUrl == other.pszUrl && self.pszMsgText == other.pszMsgText && self.pszMsgTitle == other.pszMsgTitle && self.pszWindow == other.pszWindow && self.fIndexOnFail == other.fIndexOnFail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_AKLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_AKLINK").field("cbStruct", &self.cbStruct).field("fReserved", &self.fReserved).field("pszKeywords", &self.pszKeywords).field("pszUrl", &self.pszUrl).field("pszMsgText", &self.pszMsgText).field("pszMsgTitle", &self.pszMsgTitle).field("pszWindow", &self.pszWindow).field("fIndexOnFail", &self.fIndexOnFail).finish()
    }
}
impl ::core::default::Default for HH_ENUM_CAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HH_ENUM_CAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszCatDescription == other.pszCatDescription
    }
}
impl ::core::cmp::Eq for HH_ENUM_CAT {}
impl ::core::fmt::Debug for HH_ENUM_CAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_ENUM_CAT").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszCatDescription", &self.pszCatDescription).finish()
    }
}
impl ::core::default::Default for HH_ENUM_IT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HH_ENUM_IT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.iType == other.iType && self.pszCatName == other.pszCatName && self.pszITName == other.pszITName && self.pszITDescription == other.pszITDescription
    }
}
impl ::core::cmp::Eq for HH_ENUM_IT {}
impl ::core::fmt::Debug for HH_ENUM_IT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_ENUM_IT").field("cbStruct", &self.cbStruct).field("iType", &self.iType).field("pszCatName", &self.pszCatName).field("pszITName", &self.pszITName).field("pszITDescription", &self.pszITDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_FTS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fUniCodeStrings == other.fUniCodeStrings && self.pszSearchQuery == other.pszSearchQuery && self.iProximity == other.iProximity && self.fStemmedSearch == other.fStemmedSearch && self.fTitleOnly == other.fTitleOnly && self.fExecute == other.fExecute && self.pszWindow == other.pszWindow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_FTS_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_FTS_QUERY").field("cbStruct", &self.cbStruct).field("fUniCodeStrings", &self.fUniCodeStrings).field("pszSearchQuery", &self.pszSearchQuery).field("iProximity", &self.iProximity).field("fStemmedSearch", &self.fStemmedSearch).field("fTitleOnly", &self.fTitleOnly).field("fExecute", &self.fExecute).field("pszWindow", &self.pszWindow).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HH_GPROPID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HH_GPROPID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HH_GPROPID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_POPUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_POPUP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hinst == other.hinst && self.idString == other.idString && self.pszText == other.pszText && self.pt == other.pt && self.clrForeground == other.clrForeground && self.clrBackground == other.clrBackground && self.rcMargins == other.rcMargins && self.pszFont == other.pszFont
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_POPUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_POPUP").field("cbStruct", &self.cbStruct).field("hinst", &self.hinst).field("idString", &self.idString).field("pszText", &self.pszText).field("pt", &self.pt).field("clrForeground", &self.clrForeground).field("clrBackground", &self.clrBackground).field("rcMargins", &self.rcMargins).field("pszFont", &self.pszFont).finish()
    }
}
impl ::core::default::Default for HH_SET_INFOTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HH_SET_INFOTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszInfoTypeName == other.pszInfoTypeName
    }
}
impl ::core::cmp::Eq for HH_SET_INFOTYPE {}
impl ::core::fmt::Debug for HH_SET_INFOTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_SET_INFOTYPE").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszInfoTypeName", &self.pszInfoTypeName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_WINTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fUniCodeStrings == other.fUniCodeStrings
            && self.pszType == other.pszType
            && self.fsValidMembers == other.fsValidMembers
            && self.fsWinProperties == other.fsWinProperties
            && self.pszCaption == other.pszCaption
            && self.dwStyles == other.dwStyles
            && self.dwExStyles == other.dwExStyles
            && self.rcWindowPos == other.rcWindowPos
            && self.nShowState == other.nShowState
            && self.hwndHelp == other.hwndHelp
            && self.hwndCaller == other.hwndCaller
            && self.paInfoTypes == other.paInfoTypes
            && self.hwndToolBar == other.hwndToolBar
            && self.hwndNavigation == other.hwndNavigation
            && self.hwndHTML == other.hwndHTML
            && self.iNavWidth == other.iNavWidth
            && self.rcHTML == other.rcHTML
            && self.pszToc == other.pszToc
            && self.pszIndex == other.pszIndex
            && self.pszFile == other.pszFile
            && self.pszHome == other.pszHome
            && self.fsToolBarFlags == other.fsToolBarFlags
            && self.fNotExpanded == other.fNotExpanded
            && self.curNavType == other.curNavType
            && self.tabpos == other.tabpos
            && self.idNotify == other.idNotify
            && self.tabOrder == other.tabOrder
            && self.cHistory == other.cHistory
            && self.pszJump1 == other.pszJump1
            && self.pszJump2 == other.pszJump2
            && self.pszUrlJump1 == other.pszUrlJump1
            && self.pszUrlJump2 == other.pszUrlJump2
            && self.rcMinSize == other.rcMinSize
            && self.cbInfoTypes == other.cbInfoTypes
            && self.pszCustomTabs == other.pszCustomTabs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_WINTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_WINTYPE")
            .field("cbStruct", &self.cbStruct)
            .field("fUniCodeStrings", &self.fUniCodeStrings)
            .field("pszType", &self.pszType)
            .field("fsValidMembers", &self.fsValidMembers)
            .field("fsWinProperties", &self.fsWinProperties)
            .field("pszCaption", &self.pszCaption)
            .field("dwStyles", &self.dwStyles)
            .field("dwExStyles", &self.dwExStyles)
            .field("rcWindowPos", &self.rcWindowPos)
            .field("nShowState", &self.nShowState)
            .field("hwndHelp", &self.hwndHelp)
            .field("hwndCaller", &self.hwndCaller)
            .field("paInfoTypes", &self.paInfoTypes)
            .field("hwndToolBar", &self.hwndToolBar)
            .field("hwndNavigation", &self.hwndNavigation)
            .field("hwndHTML", &self.hwndHTML)
            .field("iNavWidth", &self.iNavWidth)
            .field("rcHTML", &self.rcHTML)
            .field("pszToc", &self.pszToc)
            .field("pszIndex", &self.pszIndex)
            .field("pszFile", &self.pszFile)
            .field("pszHome", &self.pszHome)
            .field("fsToolBarFlags", &self.fsToolBarFlags)
            .field("fNotExpanded", &self.fNotExpanded)
            .field("curNavType", &self.curNavType)
            .field("tabpos", &self.tabpos)
            .field("idNotify", &self.idNotify)
            .field("tabOrder", &self.tabOrder)
            .field("cHistory", &self.cHistory)
            .field("pszJump1", &self.pszJump1)
            .field("pszJump2", &self.pszJump2)
            .field("pszUrlJump1", &self.pszUrlJump1)
            .field("pszUrlJump2", &self.pszUrlJump2)
            .field("rcMinSize", &self.rcMinSize)
            .field("cbInfoTypes", &self.cbInfoTypes)
            .field("pszCustomTabs", &self.pszCustomTabs)
            .finish()
    }
}
impl ::core::default::Default for HTML_HELP_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTML_HELP_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTML_HELP_COMMAND").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IITDatabase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITDatabase {}
impl ::core::fmt::Debug for IITDatabase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITDatabase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IITPropList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IITPropList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IITPropList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITPropList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IITPropList {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitNew)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IITResultSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITResultSet {}
impl ::core::fmt::Debug for IITResultSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITResultSet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IITWordWheel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITWordWheel {}
impl ::core::fmt::Debug for IITWordWheel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITWordWheel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStemSink {}
impl ::core::fmt::Debug for IStemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStemSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStemmerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStemmerConfig {}
impl ::core::fmt::Debug for IStemmerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStemmerConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWordBreakerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWordBreakerConfig {}
impl ::core::fmt::Debug for IWordBreakerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWordBreakerConfig").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for ROWSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ROWSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.lRowFirst == other.lRowFirst && self.cRows == other.cRows && self.cProperties == other.cProperties && self.cRowsTotal == other.cRowsTotal
    }
}
impl ::core::cmp::Eq for ROWSTATUS {}
impl ::core::fmt::Debug for ROWSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROWSTATUS").field("lRowFirst", &self.lRowFirst).field("cRows", &self.cRows).field("cProperties", &self.cProperties).field("cRowsTotal", &self.cRowsTotal).finish()
    }
}
impl ::core::default::Default for WORD_WHEEL_OPEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORD_WHEEL_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORD_WHEEL_OPEN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
