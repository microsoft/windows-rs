#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved1 == other.ulReserved1 && self.cValues == other.cValues && self.rgPropVals == other.rgPropVals
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADRENTRY").field("ulReserved1", &self.ulReserved1).field("cValues", &self.cValues).field("rgPropVals", &self.rgPropVals).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.aEntries == other.aEntries
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADRLIST").field("cEntries", &self.cEntries).field("aEntries", &self.aEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRPARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DTBLBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulPRControl == other.ulPRControl
    }
}
impl ::core::cmp::Eq for DTBLBUTTON {}
impl ::core::fmt::Debug for DTBLBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRControl", &self.ulPRControl).finish()
    }
}
impl ::core::default::Default for DTBLCHECKBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLCHECKBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulPRPropertyName == other.ulPRPropertyName
    }
}
impl ::core::cmp::Eq for DTBLCHECKBOX {}
impl ::core::fmt::Debug for DTBLCHECKBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLCHECKBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRPropertyName", &self.ulPRPropertyName).finish()
    }
}
impl ::core::default::Default for DTBLCOMBOBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLCOMBOBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszCharsAllowed == other.ulbLpszCharsAllowed && self.ulFlags == other.ulFlags && self.ulNumCharsAllowed == other.ulNumCharsAllowed && self.ulPRPropertyName == other.ulPRPropertyName && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLCOMBOBOX {}
impl ::core::fmt::Debug for DTBLCOMBOBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLCOMBOBOX").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPRPropertyName", &self.ulPRPropertyName).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::default::Default for DTBLDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLDDLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulPRDisplayProperty == other.ulPRDisplayProperty && self.ulPRSetProperty == other.ulPRSetProperty && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLDDLBX {}
impl ::core::fmt::Debug for DTBLDDLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLDDLBX").field("ulFlags", &self.ulFlags).field("ulPRDisplayProperty", &self.ulPRDisplayProperty).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::default::Default for DTBLEDIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLEDIT {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszCharsAllowed == other.ulbLpszCharsAllowed && self.ulFlags == other.ulFlags && self.ulNumCharsAllowed == other.ulNumCharsAllowed && self.ulPropTag == other.ulPropTag
    }
}
impl ::core::cmp::Eq for DTBLEDIT {}
impl ::core::fmt::Debug for DTBLEDIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLEDIT").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPropTag", &self.ulPropTag).finish()
    }
}
impl ::core::default::Default for DTBLGROUPBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLGROUPBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DTBLGROUPBOX {}
impl ::core::fmt::Debug for DTBLGROUPBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLGROUPBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::default::Default for DTBLLABEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLLABEL {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabelName == other.ulbLpszLabelName && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DTBLLABEL {}
impl ::core::fmt::Debug for DTBLLABEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLLABEL").field("ulbLpszLabelName", &self.ulbLpszLabelName).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::default::Default for DTBLLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulPRSetProperty == other.ulPRSetProperty && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLLBX {}
impl ::core::fmt::Debug for DTBLLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLLBX").field("ulFlags", &self.ulFlags).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::default::Default for DTBLMVDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLMVDDLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulMVPropTag == other.ulMVPropTag
    }
}
impl ::core::cmp::Eq for DTBLMVDDLBX {}
impl ::core::fmt::Debug for DTBLMVDDLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLMVDDLBX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
impl ::core::default::Default for DTBLMVLISTBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLMVLISTBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulMVPropTag == other.ulMVPropTag
    }
}
impl ::core::cmp::Eq for DTBLMVLISTBOX {}
impl ::core::fmt::Debug for DTBLMVLISTBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLMVLISTBOX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
impl ::core::default::Default for DTBLPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLPAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulbLpszComponent == other.ulbLpszComponent && self.ulContext == other.ulContext
    }
}
impl ::core::cmp::Eq for DTBLPAGE {}
impl ::core::fmt::Debug for DTBLPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLPAGE").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulbLpszComponent", &self.ulbLpszComponent).field("ulContext", &self.ulContext).finish()
    }
}
impl ::core::default::Default for DTBLRADIOBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTBLRADIOBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulcButtons == other.ulcButtons && self.ulPropTag == other.ulPropTag && self.lReturnValue == other.lReturnValue
    }
}
impl ::core::cmp::Eq for DTBLRADIOBUTTON {}
impl ::core::fmt::Debug for DTBLRADIOBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLRADIOBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulcButtons", &self.ulcButtons).field("ulPropTag", &self.ulPropTag).field("lReturnValue", &self.lReturnValue).finish()
    }
}
impl ::core::default::Default for DTCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DTPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ENTRYID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENTRYID {
    fn eq(&self, other: &Self) -> bool {
        self.abFlags == other.abFlags && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for ENTRYID {}
impl ::core::fmt::Debug for ENTRYID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENTRYID").field("abFlags", &self.abFlags).field("ab", &self.ab).finish()
    }
}
impl ::core::default::Default for ERROR_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ERROR_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.scode == other.scode && self.ulFlags == other.ulFlags && self.lpMAPIError == other.lpMAPIError
    }
}
impl ::core::cmp::Eq for ERROR_NOTIFICATION {}
impl ::core::fmt::Debug for ERROR_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERROR_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("scode", &self.scode).field("ulFlags", &self.ulFlags).field("lpMAPIError", &self.lpMAPIError).finish()
    }
}
impl ::core::default::Default for EXTENDED_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTENDED_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.cb == other.cb && self.pbEventParameters == other.pbEventParameters
    }
}
impl ::core::cmp::Eq for EXTENDED_NOTIFICATION {}
impl ::core::fmt::Debug for EXTENDED_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENDED_NOTIFICATION").field("ulEvent", &self.ulEvent).field("cb", &self.cb).field("pbEventParameters", &self.pbEventParameters).finish()
    }
}
impl ::core::default::Default for FLATENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLATENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.abEntry == other.abEntry
    }
}
impl ::core::cmp::Eq for FLATENTRY {}
impl ::core::fmt::Debug for FLATENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATENTRY").field("cb", &self.cb).field("abEntry", &self.abEntry).finish()
    }
}
impl ::core::default::Default for FLATENTRYLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLATENTRYLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.cbEntries == other.cbEntries && self.abEntries == other.abEntries
    }
}
impl ::core::cmp::Eq for FLATENTRYLIST {}
impl ::core::fmt::Debug for FLATENTRYLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATENTRYLIST").field("cEntries", &self.cEntries).field("cbEntries", &self.cbEntries).field("abEntries", &self.abEntries).finish()
    }
}
impl ::core::default::Default for FLATMTSIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLATMTSIDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cMTSIDs == other.cMTSIDs && self.cbMTSIDs == other.cbMTSIDs && self.abMTSIDs == other.abMTSIDs
    }
}
impl ::core::cmp::Eq for FLATMTSIDLIST {}
impl ::core::fmt::Debug for FLATMTSIDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATMTSIDLIST").field("cMTSIDs", &self.cMTSIDs).field("cbMTSIDs", &self.cbMTSIDs).field("abMTSIDs", &self.abMTSIDs).finish()
    }
}
impl ::core::default::Default for FlagList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FlagList {
    fn eq(&self, other: &Self) -> bool {
        self.cFlags == other.cFlags && self.ulFlag == other.ulFlag
    }
}
impl ::core::cmp::Eq for FlagList {}
impl ::core::fmt::Debug for FlagList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FlagList").field("cFlags", &self.cFlags).field("ulFlag", &self.ulFlag).finish()
    }
}
impl ::core::default::Default for Gender {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Gender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gender").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IABContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IABContainer {}
impl ::core::fmt::Debug for IABContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IABContainer").field(&self.0).finish()
    }
}
impl IABContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContentsTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHierarchyTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenEntry)(::windows::core::Vtable::as_raw(self), cbentryid, lpentryid, lpinterface, ulflags, lpulobjtype, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: ::core::option::Option<*const SRestriction>, lpcontainerlist: ::core::option::Option<*const SBinaryArray>, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSearchCriteria)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lprestriction.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcontainerlist.unwrap_or(::std::ptr::null())), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSearchCriteria)(::windows::core::Vtable::as_raw(self), ulflags, lpprestriction, lppcontainerlist, ::core::mem::transmute(lpulsearchstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IAddrBook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrBook {}
impl ::core::fmt::Debug for IAddrBook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrBook").field(&self.0).finish()
    }
}
impl IAddrBook {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IAttach {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAttach {}
impl ::core::fmt::Debug for IAttach {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAttach").field(&self.0).finish()
    }
}
impl IAttach {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IDistList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDistList {}
impl ::core::fmt::Debug for IDistList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDistList").field(&self.0).finish()
    }
}
impl IDistList {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContentsTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHierarchyTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenEntry)(::windows::core::Vtable::as_raw(self), cbentryid, lpentryid, lpinterface, ulflags, lpulobjtype, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: ::core::option::Option<*const SRestriction>, lpcontainerlist: ::core::option::Option<*const SBinaryArray>, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSearchCriteria)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lprestriction.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcontainerlist.unwrap_or(::std::ptr::null())), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSearchCriteria)(::windows::core::Vtable::as_raw(self), ulflags, lpprestriction, lppcontainerlist, ::core::mem::transmute(lpulsearchstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IMAPIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIAdviseSink {}
impl ::core::fmt::Debug for IMAPIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIAdviseSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMAPIContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIContainer {}
impl ::core::fmt::Debug for IMAPIContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIContainer").field(&self.0).finish()
    }
}
impl IMAPIContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IMAPIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIControl {}
impl ::core::fmt::Debug for IMAPIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMAPIFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIFolder {}
impl ::core::fmt::Debug for IMAPIFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIFolder").field(&self.0).finish()
    }
}
impl IMAPIFolder {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContentsTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHierarchyTable)(::windows::core::Vtable::as_raw(self), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenEntry)(::windows::core::Vtable::as_raw(self), cbentryid, lpentryid, lpinterface, ulflags, lpulobjtype, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: ::core::option::Option<*const SRestriction>, lpcontainerlist: ::core::option::Option<*const SBinaryArray>, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSearchCriteria)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lprestriction.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpcontainerlist.unwrap_or(::std::ptr::null())), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSearchCriteria)(::windows::core::Vtable::as_raw(self), ulflags, lpprestriction, lppcontainerlist, ::core::mem::transmute(lpulsearchstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IMAPIProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIProgress {}
impl ::core::fmt::Debug for IMAPIProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMAPIProp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIProp {}
impl ::core::fmt::Debug for IMAPIProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIProp").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMAPIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIStatus {}
impl ::core::fmt::Debug for IMAPIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIStatus").field(&self.0).finish()
    }
}
impl IMAPIStatus {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IMAPITable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPITable {}
impl ::core::fmt::Debug for IMAPITable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPITable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMailUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMailUser {}
impl ::core::fmt::Debug for IMailUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMailUser").field(&self.0).finish()
    }
}
impl IMailUser {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessage {}
impl ::core::fmt::Debug for IMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessage").field(&self.0).finish()
    }
}
impl IMessage {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IMsgStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMsgStore {}
impl ::core::fmt::Debug for IMsgStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsgStore").field(&self.0).finish()
    }
}
impl IMsgStore {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IProfSect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProfSect {}
impl ::core::fmt::Debug for IProfSect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProfSect").field(&self.0).finish()
    }
}
impl IProfSect {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IPropData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropData {}
impl ::core::fmt::Debug for IPropData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropData").field(&self.0).finish()
    }
}
impl IPropData {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLastError)(::windows::core::Vtable::as_raw(self), hresult, ulflags, lppmapierror).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveChanges)(::windows::core::Vtable::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, ulflags, lpcvalues, lppproparray).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropList)(::windows::core::Vtable::as_raw(self), ulflags, lppproptagarray).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenProperty)(::windows::core::Vtable::as_raw(self), ulproptag, lpiid, ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProps)(::windows::core::Vtable::as_raw(self), cvalues, lpproparray, lppproblems).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteProps)(::windows::core::Vtable::as_raw(self), lpproptagarray, lppproblems).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), ciidexclude, rgiidexclude, lpexcludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn CopyProps<P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMAPIProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyProps)(::windows::core::Vtable::as_raw(self), lpincludeprops, uluiparam, lpprogress.into().abi(), lpinterface, lpdestobj, ulflags, lppproblems).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNamesFromIDs)(::windows::core::Vtable::as_raw(self), lppproptags, lppropsetguid, ulflags, lpcpropnames, lppppropnames).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIDsFromNames)(::windows::core::Vtable::as_raw(self), cpropnames, lpppropnames, ulflags, lppproptags).ok()
    }
}
impl ::core::cmp::PartialEq for IProviderAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderAdmin {}
impl ::core::fmt::Debug for IProviderAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderAdmin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITableData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableData {}
impl ::core::fmt::Debug for ITableData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWABExtInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWABExtInit {}
impl ::core::fmt::Debug for IWABExtInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWABExtInit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWABObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWABObject {}
impl ::core::fmt::Debug for IWABObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWABObject").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAPIERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPIERROR {
    fn eq(&self, other: &Self) -> bool {
        self.ulVersion == other.ulVersion && self.lpszError == other.lpszError && self.lpszComponent == other.lpszComponent && self.ulLowLevelError == other.ulLowLevelError && self.ulContext == other.ulContext
    }
}
impl ::core::cmp::Eq for MAPIERROR {}
impl ::core::fmt::Debug for MAPIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPIERROR").field("ulVersion", &self.ulVersion).field("lpszError", &self.lpszError).field("lpszComponent", &self.lpszComponent).field("ulLowLevelError", &self.ulLowLevelError).field("ulContext", &self.ulContext).finish()
    }
}
impl ::core::default::Default for MAPINAMEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MAPIUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPIUID {
    fn eq(&self, other: &Self) -> bool {
        self.ab == other.ab
    }
}
impl ::core::cmp::Eq for MAPIUID {}
impl ::core::fmt::Debug for MAPIUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPIUID").field("ab", &self.ab).finish()
    }
}
impl ::core::default::Default for MTSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MTSID {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for MTSID {}
impl ::core::fmt::Debug for MTSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MTSID").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
impl ::core::default::Default for NEWMAIL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEWMAIL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.cbParentID == other.cbParentID && self.lpParentID == other.lpParentID && self.ulFlags == other.ulFlags && self.lpszMessageClass == other.lpszMessageClass && self.ulMessageFlags == other.ulMessageFlags
    }
}
impl ::core::cmp::Eq for NEWMAIL_NOTIFICATION {}
impl ::core::fmt::Debug for NEWMAIL_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWMAIL_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("cbParentID", &self.cbParentID).field("lpParentID", &self.lpParentID).field("ulFlags", &self.ulFlags).field("lpszMessageClass", &self.lpszMessageClass).field("ulMessageFlags", &self.ulMessageFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NOTIFKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NOTIFKEY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for NOTIFKEY {}
impl ::core::fmt::Debug for NOTIFKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFKEY").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
impl ::core::default::Default for OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.ulObjType == other.ulObjType && self.cbParentID == other.cbParentID && self.lpParentID == other.lpParentID && self.cbOldID == other.cbOldID && self.lpOldID == other.lpOldID && self.cbOldParentID == other.cbOldParentID && self.lpOldParentID == other.lpOldParentID && self.lpPropTagArray == other.lpPropTagArray
    }
}
impl ::core::cmp::Eq for OBJECT_NOTIFICATION {}
impl ::core::fmt::Debug for OBJECT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("ulObjType", &self.ulObjType).field("cbParentID", &self.cbParentID).field("lpParentID", &self.lpParentID).field("cbOldID", &self.cbOldID).field("lpOldID", &self.lpOldID).field("cbOldParentID", &self.cbOldParentID).field("lpOldParentID", &self.lpOldParentID).field("lpPropTagArray", &self.lpPropTagArray).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAndRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SAndRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cRes == other.cRes && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SAndRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAndRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
impl ::core::default::Default for SAppTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SAppTimeArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpat == other.lpat
    }
}
impl ::core::cmp::Eq for SAppTimeArray {}
impl ::core::fmt::Debug for SAppTimeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAppTimeArray").field("cValues", &self.cValues).field("lpat", &self.lpat).finish()
    }
}
impl ::core::default::Default for SBinary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SBinary {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpb == other.lpb
    }
}
impl ::core::cmp::Eq for SBinary {}
impl ::core::fmt::Debug for SBinary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBinary").field("cb", &self.cb).field("lpb", &self.lpb).finish()
    }
}
impl ::core::default::Default for SBinaryArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SBinaryArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpbin == other.lpbin
    }
}
impl ::core::cmp::Eq for SBinaryArray {}
impl ::core::fmt::Debug for SBinaryArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBinaryArray").field("cValues", &self.cValues).field("lpbin", &self.lpbin).finish()
    }
}
impl ::core::default::Default for SBitMaskRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SBitMaskRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relBMR == other.relBMR && self.ulPropTag == other.ulPropTag && self.ulMask == other.ulMask
    }
}
impl ::core::cmp::Eq for SBitMaskRestriction {}
impl ::core::fmt::Debug for SBitMaskRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBitMaskRestriction").field("relBMR", &self.relBMR).field("ulPropTag", &self.ulPropTag).field("ulMask", &self.ulMask).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SCommentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SCommentRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpRes == other.lpRes && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SCommentRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCommentRestriction").field("cValues", &self.cValues).field("lpRes", &self.lpRes).field("lpProp", &self.lpProp).finish()
    }
}
impl ::core::default::Default for SComparePropsRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SComparePropsRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag1 == other.ulPropTag1 && self.ulPropTag2 == other.ulPropTag2
    }
}
impl ::core::cmp::Eq for SComparePropsRestriction {}
impl ::core::fmt::Debug for SComparePropsRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SComparePropsRestriction").field("relop", &self.relop).field("ulPropTag1", &self.ulPropTag1).field("ulPropTag2", &self.ulPropTag2).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SContentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SContentRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulFuzzyLevel == other.ulFuzzyLevel && self.ulPropTag == other.ulPropTag && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SContentRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SContentRestriction").field("ulFuzzyLevel", &self.ulFuzzyLevel).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SCurrencyArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SCurrencyArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpcur == other.lpcur
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SCurrencyArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCurrencyArray").field("cValues", &self.cValues).field("lpcur", &self.lpcur).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SDateTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SDateTimeArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpft == other.lpft
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SDateTimeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDateTimeArray").field("cValues", &self.cValues).field("lpft", &self.lpft).finish()
    }
}
impl ::core::default::Default for SDoubleArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SDoubleArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpdbl == other.lpdbl
    }
}
impl ::core::cmp::Eq for SDoubleArray {}
impl ::core::fmt::Debug for SDoubleArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDoubleArray").field("cValues", &self.cValues).field("lpdbl", &self.lpdbl).finish()
    }
}
impl ::core::default::Default for SExistRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SExistRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved1 == other.ulReserved1 && self.ulPropTag == other.ulPropTag && self.ulReserved2 == other.ulReserved2
    }
}
impl ::core::cmp::Eq for SExistRestriction {}
impl ::core::fmt::Debug for SExistRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SExistRestriction").field("ulReserved1", &self.ulReserved1).field("ulPropTag", &self.ulPropTag).field("ulReserved2", &self.ulReserved2).finish()
    }
}
impl ::core::default::Default for SGuidArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SGuidArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpguid == other.lpguid
    }
}
impl ::core::cmp::Eq for SGuidArray {}
impl ::core::fmt::Debug for SGuidArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SGuidArray").field("cValues", &self.cValues).field("lpguid", &self.lpguid).finish()
    }
}
impl ::core::default::Default for SLPSTRArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SLPSTRArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lppszA == other.lppszA
    }
}
impl ::core::cmp::Eq for SLPSTRArray {}
impl ::core::fmt::Debug for SLPSTRArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLPSTRArray").field("cValues", &self.cValues).field("lppszA", &self.lppszA).finish()
    }
}
impl ::core::default::Default for SLargeIntegerArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SLargeIntegerArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpli == other.lpli
    }
}
impl ::core::cmp::Eq for SLargeIntegerArray {}
impl ::core::fmt::Debug for SLargeIntegerArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLargeIntegerArray").field("cValues", &self.cValues).field("lpli", &self.lpli).finish()
    }
}
impl ::core::default::Default for SLongArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SLongArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpl == other.lpl
    }
}
impl ::core::cmp::Eq for SLongArray {}
impl ::core::fmt::Debug for SLongArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLongArray").field("cValues", &self.cValues).field("lpl", &self.lpl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SNotRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SNotRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SNotRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNotRestriction").field("ulReserved", &self.ulReserved).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SOrRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SOrRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cRes == other.cRes && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SOrRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOrRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
impl ::core::default::Default for SPropProblem {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPropProblem {
    fn eq(&self, other: &Self) -> bool {
        self.ulIndex == other.ulIndex && self.ulPropTag == other.ulPropTag && self.scode == other.scode
    }
}
impl ::core::cmp::Eq for SPropProblem {}
impl ::core::fmt::Debug for SPropProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropProblem").field("ulIndex", &self.ulIndex).field("ulPropTag", &self.ulPropTag).field("scode", &self.scode).finish()
    }
}
impl ::core::default::Default for SPropProblemArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPropProblemArray {
    fn eq(&self, other: &Self) -> bool {
        self.cProblem == other.cProblem && self.aProblem == other.aProblem
    }
}
impl ::core::cmp::Eq for SPropProblemArray {}
impl ::core::fmt::Debug for SPropProblemArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropProblemArray").field("cProblem", &self.cProblem).field("aProblem", &self.aProblem).finish()
    }
}
impl ::core::default::Default for SPropTagArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPropTagArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.aulPropTag == other.aulPropTag
    }
}
impl ::core::cmp::Eq for SPropTagArray {}
impl ::core::fmt::Debug for SPropTagArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropTagArray").field("cValues", &self.cValues).field("aulPropTag", &self.aulPropTag).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropertyRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SPropertyRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag == other.ulPropTag && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SPropertyRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropertyRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
impl ::core::default::Default for SRealArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SRealArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpflt == other.lpflt
    }
}
impl ::core::cmp::Eq for SRealArray {}
impl ::core::fmt::Debug for SRealArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRealArray").field("cValues", &self.cValues).field("lpflt", &self.lpflt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRow {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRow {
    fn eq(&self, other: &Self) -> bool {
        self.ulAdrEntryPad == other.ulAdrEntryPad && self.cValues == other.cValues && self.lpProps == other.lpProps
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRow").field("ulAdrEntryPad", &self.ulAdrEntryPad).field("cValues", &self.cValues).field("lpProps", &self.lpProps).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRowSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRowSet {
    fn eq(&self, other: &Self) -> bool {
        self.cRows == other.cRows && self.aRow == other.aRow
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRowSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRowSet").field("cRows", &self.cRows).field("aRow", &self.aRow).finish()
    }
}
impl ::core::default::Default for SShortArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SShortArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpi == other.lpi
    }
}
impl ::core::cmp::Eq for SShortArray {}
impl ::core::fmt::Debug for SShortArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SShortArray").field("cValues", &self.cValues).field("lpi", &self.lpi).finish()
    }
}
impl ::core::default::Default for SSizeRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSizeRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag == other.ulPropTag && self.cb == other.cb
    }
}
impl ::core::cmp::Eq for SSizeRestriction {}
impl ::core::fmt::Debug for SSizeRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSizeRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("cb", &self.cb).finish()
    }
}
impl ::core::default::Default for SSortOrder {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSortOrder {
    fn eq(&self, other: &Self) -> bool {
        self.ulPropTag == other.ulPropTag && self.ulOrder == other.ulOrder
    }
}
impl ::core::cmp::Eq for SSortOrder {}
impl ::core::fmt::Debug for SSortOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSortOrder").field("ulPropTag", &self.ulPropTag).field("ulOrder", &self.ulOrder).finish()
    }
}
impl ::core::default::Default for SSortOrderSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSortOrderSet {
    fn eq(&self, other: &Self) -> bool {
        self.cSorts == other.cSorts && self.cCategories == other.cCategories && self.cExpanded == other.cExpanded && self.aSort == other.aSort
    }
}
impl ::core::cmp::Eq for SSortOrderSet {}
impl ::core::fmt::Debug for SSortOrderSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSortOrderSet").field("cSorts", &self.cSorts).field("cCategories", &self.cCategories).field("cExpanded", &self.cExpanded).field("aSort", &self.aSort).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SSubRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SSubRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulSubObject == other.ulSubObject && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SSubRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSubRestriction").field("ulSubObject", &self.ulSubObject).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for STATUS_OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for STATUS_OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.cValues == other.cValues && self.lpPropVals == other.lpPropVals
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for STATUS_OBJECT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATUS_OBJECT_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("cValues", &self.cValues).field("lpPropVals", &self.lpPropVals).finish()
    }
}
impl ::core::default::Default for SWStringArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SWStringArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lppszW == other.lppszW
    }
}
impl ::core::cmp::Eq for SWStringArray {}
impl ::core::fmt::Debug for SWStringArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWStringArray").field("cValues", &self.cValues).field("lppszW", &self.lppszW).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for TABLE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WABEXTDISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WABEXTDISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpWABObject == other.lpWABObject && self.lpAdrBook == other.lpAdrBook && self.lpPropObj == other.lpPropObj && self.fReadOnly == other.fReadOnly && self.fDataChanged == other.fDataChanged && self.ulFlags == other.ulFlags && self.lpv == other.lpv && self.lpsz == other.lpsz
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WABEXTDISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABEXTDISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WABEXTDISPLAY").field("cbSize", &self.cbSize).field("lpWABObject", &self.lpWABObject).field("lpAdrBook", &self.lpAdrBook).field("lpPropObj", &self.lpPropObj).field("fReadOnly", &self.fReadOnly).field("fDataChanged", &self.fDataChanged).field("ulFlags", &self.ulFlags).field("lpv", &self.lpv).field("lpsz", &self.lpsz).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WABIMPORTPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WABIMPORTPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpAdrBook == other.lpAdrBook && self.hWnd == other.hWnd && self.ulFlags == other.ulFlags && self.lpszFileName == other.lpszFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WABIMPORTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABIMPORTPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WABIMPORTPARAM").field("cbSize", &self.cbSize).field("lpAdrBook", &self.lpAdrBook).field("hWnd", &self.hWnd).field("ulFlags", &self.ulFlags).field("lpszFileName", &self.lpszFileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAB_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAB_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwnd == other.hwnd && self.szFileName == other.szFileName && self.ulFlags == other.ulFlags && self.guidPSExt == other.guidPSExt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WAB_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAB_PARAM").field("cbSize", &self.cbSize).field("hwnd", &self.hwnd).field("szFileName", &self.szFileName).field("ulFlags", &self.ulFlags).field("guidPSExt", &self.guidPSExt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for __UPV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
