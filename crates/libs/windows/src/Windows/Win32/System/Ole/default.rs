impl ::core::default::Default for ACTIVATEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVATEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ACTIVEOBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVEOBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVEOBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACTIVEOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACTIVEOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BINDSPEED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDSPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSPEED").field(&self.0).finish()
    }
}
impl ::core::default::Default for BUSY_DIALOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BUSY_DIALOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUSY_DIALOG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BUSY_DIALOG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BUSY_DIALOG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CADWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CADWORD {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CADWORD {}
impl ::core::fmt::Debug for CADWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADWORD").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::core::default::Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CALPOLESTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CALPOLESTR {}
impl ::core::fmt::Debug for CALPOLESTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPOLESTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::core::default::Default for CAUUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CAUUID {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUUID {}
impl ::core::fmt::Debug for CAUUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUUID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::core::default::Default for CHANGEKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGEKIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHANGE_ICON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGE_ICON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_ICON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGE_ICON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGE_ICON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CHANGE_SOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGE_SOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_SOURCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGE_SOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGE_SOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLEANLOCALSTORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pInterface == other.pInterface && self.pStorage == other.pStorage && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for CLEANLOCALSTORAGE {}
impl ::core::fmt::Debug for CLEANLOCALSTORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLEANLOCALSTORAGE").field("pInterface", &self.pInterface).field("pStorage", &self.pStorage).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for CLIPBOARD_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLIPBOARD_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIPBOARD_FORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for CONTROLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for CONTROLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hAccel == other.hAccel && self.cAccel == other.cAccel && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for CONTROLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTROLINFO").field("cb", &self.cb).field("hAccel", &self.hAccel).field("cAccel", &self.cAccel).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CTRLINFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CTRLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CTRLINFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISCARDCACHE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISCARDCACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISCARDCACHE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOCMISC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOCMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOCMISC").field(&self.0).finish()
    }
}
impl ::core::default::Default for DROPEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DROPEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPEFFECT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DROPEFFECT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DROPEFFECT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DROPEFFECT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DROPEFFECT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DROPEFFECT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DVASPECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DVASPECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DVASPECTINFO {}
impl ::core::fmt::Debug for DVASPECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVASPECTINFO").field("cb", &self.cb).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DVASPECTINFOFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DVASPECTINFOFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECTINFOFLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DVEXTENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DVEXTENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwExtentMode == other.dwExtentMode && self.sizelProposed == other.sizelProposed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DVEXTENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DVEXTENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVEXTENTINFO").field("cb", &self.cb).field("dwExtentMode", &self.dwExtentMode).field("sizelProposed", &self.sizelProposed).finish()
    }
}
impl ::core::default::Default for DVEXTENTMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DVEXTENTMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVEXTENTMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDIT_LINKS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDIT_LINKS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDIT_LINKS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EDIT_LINKS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EDIT_LINKS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for EMBDHLP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMBDHLP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBDHLP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EMBDHLP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EMBDHLP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EMBDHLP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ENUM_CONTROLS_WHICH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_CONTROLS_WHICH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_CONTROLS_WHICH_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FDEX_PROP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDEX_PROP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDEX_PROP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FDEX_PROP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FDEX_PROP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FDEX_PROP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for FONTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GUIDKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GUIDKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUIDKIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for HITRESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HITRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HITRESULT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAdviseSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAdviseSinkEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAdviseSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSinkEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IAdviseSinkEx {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const super::Com::FORMATETC, pstgmed: *const super::Com::STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).base__.OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).base__.OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnSave)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnClose)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ICanHandleException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanHandleException {}
impl ::core::fmt::Debug for ICanHandleException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanHandleException").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IClassFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IClassFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IClassFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IClassFactory2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstance)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LockServer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LockServer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IContinue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinue {}
impl ::core::fmt::Debug for IContinue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContinueCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinueCallback {}
impl ::core::fmt::Debug for IContinueCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinueCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateErrorInfo {}
impl ::core::fmt::Debug for ICreateErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateErrorInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeInfo {}
impl ::core::fmt::Debug for ICreateTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateTypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeInfo2 {}
impl ::core::fmt::Debug for ICreateTypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeInfo2").field(&self.0).finish()
    }
}
impl ICreateTypeInfo2 {
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeFlags)(::windows::core::Vtable::as_raw(self), utypeflags).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, pstrdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDocString)(::windows::core::Vtable::as_raw(self), pstrdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddRefTypeInfo)(::windows::core::Vtable::as_raw(self), ptinfo.into().abi(), phreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddFuncDesc)(::windows::core::Vtable::as_raw(self), index, pfuncdesc).ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddImplType)(::windows::core::Vtable::as_raw(self), index, hreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetImplTypeFlags)(::windows::core::Vtable::as_raw(self), index, impltypeflags).ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAlignment)(::windows::core::Vtable::as_raw(self), cbalignment).ok()
    }
    pub unsafe fn SetSchema<P0>(&self, pstrschema: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSchema)(::windows::core::Vtable::as_raw(self), pstrschema.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddVarDesc)(::windows::core::Vtable::as_raw(self), index, pvardesc).ok()
    }
    pub unsafe fn SetFuncAndParamNames(&self, index: u32, rgsznames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFuncAndParamNames)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(rgsznames.as_ptr()), rgsznames.len() as _).ok()
    }
    pub unsafe fn SetVarName<P0>(&self, index: u32, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVarName)(::windows::core::Vtable::as_raw(self), index, szname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeDescAlias)(::windows::core::Vtable::as_raw(self), ptdescalias).ok()
    }
    pub unsafe fn DefineFuncAsDllEntry<P0, P1>(&self, index: u32, szdllname: P0, szprocname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DefineFuncAsDllEntry)(::windows::core::Vtable::as_raw(self), index, szdllname.into().abi(), szprocname.into().abi()).ok()
    }
    pub unsafe fn SetFuncDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFuncDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetVarDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVarDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFuncHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVarHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetMops(&self, index: u32, bstrmops: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMops)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute_copy(bstrmops)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeIdldesc)(::windows::core::Vtable::as_raw(self), pidldesc).ok()
    }
    pub unsafe fn LayOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LayOut)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ICreateTypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeLib {}
impl ::core::fmt::Debug for ICreateTypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeLib").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateTypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeLib2 {}
impl ::core::fmt::Debug for ICreateTypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeLib2").field(&self.0).finish()
    }
}
impl ICreateTypeLib2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTypeInfo<P0>(&self, szname: P0, tkind: super::Com::TYPEKIND) -> ::windows::core::Result<ICreateTypeInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypeInfo)(::windows::core::Vtable::as_raw(self), szname.into().abi(), tkind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), szname.into().abi()).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, szdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDocString)(::windows::core::Vtable::as_raw(self), szdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpFileName<P0>(&self, szhelpfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHelpFileName)(::windows::core::Vtable::as_raw(self), szhelpfilename.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLcid)(::windows::core::Vtable::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLibFlags)(::windows::core::Vtable::as_raw(self), ulibflags).ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveAllChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IDispError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispError {}
impl ::core::fmt::Debug for IDispError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDispatchEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDispatchEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDispatchEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatchEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDropSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropSource {}
impl ::core::fmt::Debug for IDropSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDropSourceNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropSourceNotify {}
impl ::core::fmt::Debug for IDropSourceNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropSourceNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDropTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropTarget {}
impl ::core::fmt::Debug for IDropTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnterpriseDropTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnterpriseDropTarget {}
impl ::core::fmt::Debug for IEnterpriseDropTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnterpriseDropTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumOLEVERB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOLEVERB {}
impl ::core::fmt::Debug for IEnumOLEVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOLEVERB").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumOleDocumentViews {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOleDocumentViews {}
impl ::core::fmt::Debug for IEnumOleDocumentViews {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOleDocumentViews").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumOleUndoUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOleUndoUnits {}
impl ::core::fmt::Debug for IEnumOleUndoUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOleUndoUnits").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumVARIANT {}
impl ::core::fmt::Debug for IEnumVARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumVARIANT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFont {}
impl ::core::fmt::Debug for IFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFont").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFontDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFontDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFontDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFontDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFontEventsDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFontEventsDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFontEventsDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFontEventsDisp").field(&self.0).finish()
    }
}
impl ::core::default::Default for IGNOREMIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IGNOREMIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGNOREMIME").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetOleObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetOleObject {}
impl ::core::fmt::Debug for IGetOleObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetOleObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetVBAObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetVBAObject {}
impl ::core::fmt::Debug for IGetVBAObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetVBAObject").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSERT_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INSERT_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSERT_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INSERT_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INSERT_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INTERFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pmethdata == other.pmethdata && self.cMembers == other.cMembers
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INTERFACEDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INTERFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEDATA").field("pmethdata", &self.pmethdata).field("cMembers", &self.cMembers).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectIdentity {}
impl ::core::fmt::Debug for IObjectIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectIdentity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectWithSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithSite {}
impl ::core::fmt::Debug for IObjectWithSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleAdviseHolder {}
impl ::core::fmt::Debug for IOleAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleAdviseHolder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCache {}
impl ::core::fmt::Debug for IOleCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCache2 {}
impl ::core::fmt::Debug for IOleCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCache2").field(&self.0).finish()
    }
}
impl IOleCache2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Cache)(::windows::core::Vtable::as_raw(self), pformatetc, advf, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Uncache(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Uncache)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCache(&self) -> ::windows::core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumCache)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitCache<P0>(&self, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitCache)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), pformatetc, pmedium, frelease.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleCacheControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCacheControl {}
impl ::core::fmt::Debug for IOleCacheControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCacheControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleClientSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleClientSite {}
impl ::core::fmt::Debug for IOleClientSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleClientSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleCommandTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCommandTarget {}
impl ::core::fmt::Debug for IOleCommandTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCommandTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleContainer {}
impl ::core::fmt::Debug for IOleContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleContainer").field(&self.0).finish()
    }
}
impl IOleContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
}
impl ::core::cmp::PartialEq for IOleControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleControl {}
impl ::core::fmt::Debug for IOleControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleControlSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleControlSite {}
impl ::core::fmt::Debug for IOleControlSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleControlSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocument {}
impl ::core::fmt::Debug for IOleDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleDocumentSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocumentSite {}
impl ::core::fmt::Debug for IOleDocumentSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocumentSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleDocumentView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocumentView {}
impl ::core::fmt::Debug for IOleDocumentView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocumentView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceActiveObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceActiveObject {}
impl ::core::fmt::Debug for IOleInPlaceActiveObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceActiveObject").field(&self.0).finish()
    }
}
impl IOleInPlaceActiveObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceFrame {}
impl ::core::fmt::Debug for IOleInPlaceFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceFrame").field(&self.0).finish()
    }
}
impl IOleInPlaceFrame {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBorder(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBorder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    pub unsafe fn SetActiveObject<P0, P1>(&self, pactiveobject: P0, pszobjname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceActiveObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetActiveObject)(::windows::core::Vtable::as_raw(self), pactiveobject.into().abi(), pszobjname.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceObject {}
impl ::core::fmt::Debug for IOleInPlaceObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceObject").field(&self.0).finish()
    }
}
impl IOleInPlaceObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceObjectWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceObjectWindowless {}
impl ::core::fmt::Debug for IOleInPlaceObjectWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceObjectWindowless").field(&self.0).finish()
    }
}
impl IOleInPlaceObjectWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UIDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectRects)(::windows::core::Vtable::as_raw(self), lprcposrect, lprccliprect).ok()
    }
    pub unsafe fn ReactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSite {}
impl ::core::fmt::Debug for IOleInPlaceSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSite").field(&self.0).finish()
    }
}
impl IOleInPlaceSite {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSiteEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSiteEx {}
impl ::core::fmt::Debug for IOleInPlaceSiteEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSiteEx").field(&self.0).finish()
    }
}
impl IOleInPlaceSiteEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CanInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetWindowContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppframe), ::core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Scroll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scrollextant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnUIDeactivate)(::windows::core::Vtable::as_raw(self), fundoable.into()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DiscardUndoState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnPosRectChange)(::windows::core::Vtable::as_raw(self), lprcposrect).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSiteWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSiteWindowless {}
impl ::core::fmt::Debug for IOleInPlaceSiteWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSiteWindowless").field(&self.0).finish()
    }
}
impl IOleInPlaceSiteWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CanInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindowContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppframe), ::core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Scroll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scrollextant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnUIDeactivate)(::windows::core::Vtable::as_raw(self), fundoable.into()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardUndoState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnPosRectChange)(::windows::core::Vtable::as_raw(self), lprcposrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceActivateEx)(::windows::core::Vtable::as_raw(self), pfnoredraw, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceDeactivateEx<P0>(&self, fnoredraw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceDeactivateEx)(::windows::core::Vtable::as_raw(self), fnoredraw.into()).ok()
    }
    pub unsafe fn RequestUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceUIWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceUIWindow {}
impl ::core::fmt::Debug for IOleInPlaceUIWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceUIWindow").field(&self.0).finish()
    }
}
impl IOleInPlaceUIWindow {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleItemContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleItemContainer {}
impl ::core::fmt::Debug for IOleItemContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleItemContainer").field(&self.0).finish()
    }
}
impl IOleItemContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumObjects(&self, grfflags: OLECONTF) -> ::windows::core::Result<super::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), grfflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockContainer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LockContainer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IOleLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleLink {}
impl ::core::fmt::Debug for IOleLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleLink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleObject {}
impl ::core::fmt::Debug for IOleObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleParentUndoUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleParentUndoUnit {}
impl ::core::fmt::Debug for IOleParentUndoUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleParentUndoUnit").field(&self.0).finish()
    }
}
impl IOleParentUndoUnit {
    pub unsafe fn Do<P0>(&self, pundomanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoManager>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Do)(::windows::core::Vtable::as_raw(self), pundomanager.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUnitType(&self, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnitType)(::windows::core::Vtable::as_raw(self), pclsid, plid).ok()
    }
    pub unsafe fn OnNextAdd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnNextAdd)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IOleUILinkContainerA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkContainerA {}
impl ::core::fmt::Debug for IOleUILinkContainerA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkContainerA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleUILinkContainerW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkContainerW {}
impl ::core::fmt::Debug for IOleUILinkContainerW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkContainerW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleUILinkInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkInfoA {}
impl ::core::fmt::Debug for IOleUILinkInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkInfoA").field(&self.0).finish()
    }
}
impl IOleUILinkInfoA {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
}
impl ::core::cmp::PartialEq for IOleUILinkInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkInfoW {}
impl ::core::fmt::Debug for IOleUILinkInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkInfoW").field(&self.0).finish()
    }
}
impl IOleUILinkInfoW {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PWSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
}
impl ::core::cmp::PartialEq for IOleUIObjInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUIObjInfoA {}
impl ::core::fmt::Debug for IOleUIObjInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUIObjInfoA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleUIObjInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUIObjInfoW {}
impl ::core::fmt::Debug for IOleUIObjInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUIObjInfoW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleUndoManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUndoManager {}
impl ::core::fmt::Debug for IOleUndoManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUndoManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleUndoUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUndoUnit {}
impl ::core::fmt::Debug for IOleUndoUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUndoUnit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOleWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleWindow {}
impl ::core::fmt::Debug for IOleWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IParseDisplayName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IParseDisplayName {}
impl ::core::fmt::Debug for IParseDisplayName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IParseDisplayName").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPerPropertyBrowsing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerPropertyBrowsing {}
impl ::core::fmt::Debug for IPerPropertyBrowsing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerPropertyBrowsing").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistPropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistPropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPicture {}
impl ::core::fmt::Debug for IPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPicture").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPicture2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPicture2 {}
impl ::core::fmt::Debug for IPicture2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPicture2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPictureDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPictureDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPictureDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPictureDisp").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPointerInactive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPointerInactive {}
impl ::core::fmt::Debug for IPointerInactive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPointerInactive").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint {}
impl ::core::fmt::Debug for IPrint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyNotifySink {}
impl ::core::fmt::Debug for IPropertyNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPage {}
impl ::core::fmt::Debug for IPropertyPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyPage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPage2 {}
impl ::core::fmt::Debug for IPropertyPage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPage2").field(&self.0).finish()
    }
}
impl IPropertyPage2 {
    pub unsafe fn SetPageSite<P0>(&self, ppagesite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPropertyPageSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPageSite)(::windows::core::Vtable::as_raw(self), ppagesite.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0, P1>(&self, hwndparent: P0, prect: *const super::super::Foundation::RECT, bmodal: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), hwndparent.into(), prect, bmodal.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPageInfo)(::windows::core::Vtable::as_raw(self), ppageinfo).ok()
    }
    pub unsafe fn SetObjects(&self, ppunk: &[::windows::core::IUnknown]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjects)(::windows::core::Vtable::as_raw(self), ppunk.len() as _, ::core::mem::transmute(ppunk.as_ptr())).ok()
    }
    pub unsafe fn Show(&self, ncmdshow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), ncmdshow).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    pub unsafe fn IsPageDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPageDirty)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Apply)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help<P0>(&self, pszhelpdir: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Help)(::windows::core::Vtable::as_raw(self), pszhelpdir.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
}
impl ::core::cmp::PartialEq for IPropertyPageSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPageSite {}
impl ::core::fmt::Debug for IPropertyPageSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPageSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProtectFocus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectFocus {}
impl ::core::fmt::Debug for IProtectFocus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectFocus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProtectedModeMenuServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectedModeMenuServices {}
impl ::core::fmt::Debug for IProtectedModeMenuServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectedModeMenuServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvideClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideClassInfo {}
impl ::core::fmt::Debug for IProvideClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideClassInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvideClassInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideClassInfo2 {}
impl ::core::fmt::Debug for IProvideClassInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideClassInfo2").field(&self.0).finish()
    }
}
impl IProvideClassInfo2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IProvideMultipleClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideMultipleClassInfo {}
impl ::core::fmt::Debug for IProvideMultipleClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideMultipleClassInfo").field(&self.0).finish()
    }
}
impl IProvideMultipleClassInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), dwguidkind, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IProvideRuntimeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideRuntimeContext {}
impl ::core::fmt::Debug for IProvideRuntimeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideRuntimeContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQuickActivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQuickActivate {}
impl ::core::fmt::Debug for IQuickActivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQuickActivate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRecordInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecordInfo {}
impl ::core::fmt::Debug for IRecordInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecordInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimpleFrameSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleFrameSite {}
impl ::core::fmt::Debug for ISimpleFrameSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleFrameSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpecifyPropertyPages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpecifyPropertyPages {}
impl ::core::fmt::Debug for ISpecifyPropertyPages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpecifyPropertyPages").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITypeChangeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeChangeEvents {}
impl ::core::fmt::Debug for ITypeChangeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeChangeEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITypeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeFactory {}
impl ::core::fmt::Debug for ITypeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITypeMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeMarshal {}
impl ::core::fmt::Debug for ITypeMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeMarshal").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVBFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVBFormat {}
impl ::core::fmt::Debug for IVBFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVBGetControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVBGetControl {}
impl ::core::fmt::Debug for IVBGetControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBGetControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVariantChangeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVariantChangeType {}
impl ::core::fmt::Debug for IVariantChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVariantChangeType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObject {}
impl ::core::fmt::Debug for IViewObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewObject2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObject2 {}
impl ::core::fmt::Debug for IViewObject2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObject2").field(&self.0).finish()
    }
}
impl IViewObject2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, lprcwbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hdctargetdev.into(), hdcdraw.into(), ::core::mem::transmute(lprcbounds.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprcwbounds.unwrap_or(::std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetColorSet)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hictargetdev.into(), ppcolorset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Freeze)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unfreeze)(::windows::core::Vtable::as_raw(self), dwfreeze).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAdvise)(::windows::core::Vtable::as_raw(self), aspects, advf, padvsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: ::core::option::Option<*mut u32>, padvf: ::core::option::Option<*mut u32>, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAdvise)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paspects.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(padvf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppadvsink)).ok()
    }
}
impl ::core::cmp::PartialEq for IViewObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectEx {}
impl ::core::fmt::Debug for IViewObjectEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectEx").field(&self.0).finish()
    }
}
impl IViewObjectEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, lprcwbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Draw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hdctargetdev.into(), hdcdraw.into(), ::core::mem::transmute(lprcbounds.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprcwbounds.unwrap_or(::std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetColorSet)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hictargetdev.into(), ppcolorset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Freeze)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unfreeze)(::windows::core::Vtable::as_raw(self), dwfreeze).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAdvise)(::windows::core::Vtable::as_raw(self), aspects, advf, padvsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: ::core::option::Option<*mut u32>, padvf: ::core::option::Option<*mut u32>, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAdvise)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paspects.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(padvf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppadvsink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetExtent)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, ptd, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IZoomEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoomEvents {}
impl ::core::fmt::Debug for IZoomEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoomEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for KEYMODIFIERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEYMODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYMODIFIERS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KEYMODIFIERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KEYMODIFIERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KEYMODIFIERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KEYMODIFIERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KEYMODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIBFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIBFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbLicInfo == other.cbLicInfo && self.fRuntimeKeyAvail == other.fRuntimeKeyAvail && self.fLicVerified == other.fLicVerified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LICINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LICINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LICINFO").field("cbLicInfo", &self.cbLicInfo).field("fRuntimeKeyAvail", &self.fRuntimeKeyAvail).field("fLicVerified", &self.fLicVerified).finish()
    }
}
impl ::core::default::Default for LOAD_PICTURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOAD_PICTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_PICTURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOAD_PICTURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOAD_PICTURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MEDIAPLAYBACK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEDIAPLAYBACK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIAPLAYBACK_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for METHODDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for METHODDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.ppdata == other.ppdata && self.dispid == other.dispid && self.iMeth == other.iMeth && self.cc == other.cc && self.cArgs == other.cArgs && self.wFlags == other.wFlags && self.vtReturn == other.vtReturn
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for METHODDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for METHODDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METHODDATA").field("szName", &self.szName).field("ppdata", &self.ppdata).field("dispid", &self.dispid).field("iMeth", &self.iMeth).field("cc", &self.cc).field("cArgs", &self.cArgs).field("wFlags", &self.wFlags).field("vtReturn", &self.vtReturn).finish()
    }
}
impl ::core::default::Default for MULTICLASSINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MULTICLASSINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTICLASSINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NUMPARSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NUMPARSE {
    fn eq(&self, other: &Self) -> bool {
        self.cDig == other.cDig && self.dwInFlags == other.dwInFlags && self.dwOutFlags == other.dwOutFlags && self.cchUsed == other.cchUsed && self.nBaseShift == other.nBaseShift && self.nPwr10 == other.nPwr10
    }
}
impl ::core::cmp::Eq for NUMPARSE {}
impl ::core::fmt::Debug for NUMPARSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMPARSE").field("cDig", &self.cDig).field("dwInFlags", &self.dwInFlags).field("dwOutFlags", &self.dwOutFlags).field("cchUsed", &self.cchUsed).field("nBaseShift", &self.nBaseShift).field("nPwr10", &self.nPwr10).finish()
    }
}
impl ::core::default::Default for NUMPARSE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NUMPARSE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NUMPARSE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NUMPARSE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NUMPARSE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NUMPARSE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECTDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid && self.dwDrawAspect == other.dwDrawAspect && self.sizel == other.sizel && self.pointl == other.pointl && self.dwStatus == other.dwStatus && self.dwFullUserTypeName == other.dwFullUserTypeName && self.dwSrcOfCopy == other.dwSrcOfCopy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBJECTDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTDESCRIPTOR").field("cbSize", &self.cbSize).field("clsid", &self.clsid).field("dwDrawAspect", &self.dwDrawAspect).field("sizel", &self.sizel).field("pointl", &self.pointl).field("dwStatus", &self.dwStatus).field("dwFullUserTypeName", &self.dwFullUserTypeName).field("dwSrcOfCopy", &self.dwSrcOfCopy).finish()
    }
}
impl ::core::default::Default for OBJECT_PROPERTIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OCPFIPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructSize == other.cbStructSize && self.hWndOwner == other.hWndOwner && self.x == other.x && self.y == other.y && self.lpszCaption == other.lpszCaption && self.cObjects == other.cObjects && self.lplpUnk == other.lplpUnk && self.cPages == other.cPages && self.lpPages == other.lpPages && self.lcid == other.lcid && self.dispidInitialProperty == other.dispidInitialProperty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OCPFIPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCPFIPARAMS").field("cbStructSize", &self.cbStructSize).field("hWndOwner", &self.hWndOwner).field("x", &self.x).field("y", &self.y).field("lpszCaption", &self.lpszCaption).field("cObjects", &self.cObjects).field("lplpUnk", &self.lplpUnk).field("cPages", &self.cPages).field("lpPages", &self.lpPages).field("lcid", &self.lcid).field("dispidInitialProperty", &self.dispidInitialProperty).finish()
    }
}
impl ::core::default::Default for OLECLOSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECLOSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECLOSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLECMD {
    fn eq(&self, other: &Self) -> bool {
        self.cmdID == other.cmdID && self.cmdf == other.cmdf
    }
}
impl ::core::cmp::Eq for OLECMD {}
impl ::core::fmt::Debug for OLECMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLECMD").field("cmdID", &self.cmdID).field("cmdf", &self.cmdf).finish()
    }
}
impl ::core::default::Default for OLECMDEXECOPT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDEXECOPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDEXECOPT").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDF").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_BROWSERSTATEFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_BROWSERSTATEFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_BROWSERSTATEFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_OPTICAL_ZOOMFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_OPTICAL_ZOOMFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_OPTICAL_ZOOMFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_PAGEACTIONFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_PAGEACTIONFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_PAGEACTIONFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_REFRESHFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_REFRESHFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_REFRESHFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_VIEWPORT_MODE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_VIEWPORT_MODE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_VIEWPORT_MODE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDID_WINDOWSTATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDID_WINDOWSTATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_WINDOWSTATE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLECMDTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cmdtextf == other.cmdtextf && self.cwActual == other.cwActual && self.cwBuf == other.cwBuf && self.rgwz == other.rgwz
    }
}
impl ::core::cmp::Eq for OLECMDTEXT {}
impl ::core::fmt::Debug for OLECMDTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLECMDTEXT").field("cmdtextf", &self.cmdtextf).field("cwActual", &self.cwActual).field("cwBuf", &self.cwBuf).field("rgwz", &self.rgwz).finish()
    }
}
impl ::core::default::Default for OLECMDTEXTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECMDTEXTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDTEXTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECONTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLECREATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLECREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECREATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLEDCFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEDCFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEDCFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLEGETMONIKER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEGETMONIKER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEGETMONIKER").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEINPLACEFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEINPLACEFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fMDIApp == other.fMDIApp && self.hwndFrame == other.hwndFrame && self.haccel == other.haccel && self.cAccelEntries == other.cAccelEntries
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEINPLACEFRAMEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEINPLACEFRAMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEINPLACEFRAMEINFO").field("cb", &self.cb).field("fMDIApp", &self.fMDIApp).field("hwndFrame", &self.hwndFrame).field("haccel", &self.haccel).field("cAccelEntries", &self.cAccelEntries).finish()
    }
}
impl ::core::default::Default for OLEIVERB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEIVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEIVERB").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLELINKBIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLELINKBIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLELINKBIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLEMENUGROUPWIDTHS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}
impl ::core::cmp::Eq for OLEMENUGROUPWIDTHS {}
impl ::core::fmt::Debug for OLEMENUGROUPWIDTHS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEMENUGROUPWIDTHS").field("width", &self.width).finish()
    }
}
impl ::core::default::Default for OLEMISC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEMISC").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLERENDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLERENDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLERENDER").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::default::Default for OLEUIBUSYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::default::Default for OLEUIBUSYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICHANGEICONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICHANGEICONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::default::Default for OLEUICHANGESOURCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::default::Default for OLEUICHANGESOURCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICONVERTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICONVERTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUIEDITLINKSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUIEDITLINKSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIGNRLPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIGNRLPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for OLEUIINSERTOBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for OLEUIINSERTOBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUILINKPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUILINKPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIOBJECTPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEUIOBJECTPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIOBJECTPROPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIOBJECTPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("lpPS", &self.lpPS).field("dwObject", &self.dwObject).field("lpObjInfo", &self.lpObjInfo).field("dwLink", &self.dwLink).field("lpLinkInfo", &self.lpLinkInfo).field("lpGP", &self.lpGP).field("lpVP", &self.lpVP).field("lpLP", &self.lpLP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIOBJECTPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEUIOBJECTPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIOBJECTPROPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIOBJECTPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("lpPS", &self.lpPS).field("dwObject", &self.dwObject).field("lpObjInfo", &self.lpObjInfo).field("dwLink", &self.dwLink).field("lpLinkInfo", &self.lpLinkInfo).field("lpGP", &self.lpGP).field("lpVP", &self.lpVP).field("lpLP", &self.lpLP).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for OLEUIPASTEENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for OLEUIPASTEENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for OLEUIPASTEENTRYA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for OLEUIPASTEENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTEENTRYA").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for OLEUIPASTEENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for OLEUIPASTEENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for OLEUIPASTEENTRYW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for OLEUIPASTEENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTEENTRYW").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
impl ::core::default::Default for OLEUIPASTEFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEUIPASTEFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEUIPASTEFLAG").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for OLEUIPASTESPECIALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for OLEUIPASTESPECIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIVIEWPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIVIEWPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OLEUPDATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEUPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEUPDATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for OLEVERB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for OLEVERB {
    fn eq(&self, other: &Self) -> bool {
        self.lVerb == other.lVerb && self.lpszVerbName == other.lpszVerbName && self.fuFlags == other.fuFlags && self.grfAttribs == other.grfAttribs
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for OLEVERB {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for OLEVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEVERB").field("lVerb", &self.lVerb).field("lpszVerbName", &self.lpszVerbName).field("fuFlags", &self.fuFlags).field("grfAttribs", &self.grfAttribs).finish()
    }
}
impl ::core::default::Default for OLEVERBATTRIB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEVERBATTRIB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEVERBATTRIB").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLEWHICHMK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLEWHICHMK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEWHICHMK").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLE_TRISTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OLE_TRISTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLE_TRISTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAGEACTION_UI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGEACTION_UI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGEACTION_UI").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nFromPage == other.nFromPage && self.nToPage == other.nToPage
    }
}
impl ::core::cmp::Eq for PAGERANGE {}
impl ::core::fmt::Debug for PAGERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGERANGE").field("nFromPage", &self.nFromPage).field("nToPage", &self.nToPage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESET {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fOddPages == other.fOddPages && self.fEvenPages == other.fEvenPages && self.cPageRange == other.cPageRange && self.rgPages == other.rgPages
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAGESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGESET").field("cbStruct", &self.cbStruct).field("fOddPages", &self.fOddPages).field("fEvenPages", &self.fEvenPages).field("cPageRange", &self.cPageRange).field("rgPages", &self.rgPages).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for PARAMDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for PARAMDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.vt == other.vt
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for PARAMDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for PARAMDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAMDATA").field("szName", &self.szName).field("vt", &self.vt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for PARAMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for PARAMDESC {
    fn eq(&self, other: &Self) -> bool {
        self.pparamdescex == other.pparamdescex && self.wParamFlags == other.wParamFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for PARAMDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAMDESC").field("pparamdescex", &self.pparamdescex).field("wParamFlags", &self.wParamFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PARAMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAMFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PARAMFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAMFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAMFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAMFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAMFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PASTE_SPECIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PASTE_SPECIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PASTE_SPECIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PASTE_SPECIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PASTE_SPECIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PICTUREATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PICTUREATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PICTUREATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for PICTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PICTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PICTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POINTERINACTIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTERINACTIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTERINACTIVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POINTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTF {}
impl ::core::fmt::Debug for POINTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTF").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for PRINTFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRINTFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTFLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRINTFLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRINTFLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRINTFLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRINTFLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRINTFLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROPBAG2_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROPBAG2_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPBAG2_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPPAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pszTitle == other.pszTitle && self.size == other.size && self.pszDocString == other.pszDocString && self.pszHelpFile == other.pszHelpFile && self.dwHelpContext == other.dwHelpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROPPAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPPAGEINFO").field("cb", &self.cb).field("pszTitle", &self.pszTitle).field("size", &self.size).field("pszDocString", &self.pszDocString).field("pszHelpFile", &self.pszHelpFile).field("dwHelpContext", &self.dwHelpContext).finish()
    }
}
impl ::core::default::Default for PROPPAGESTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROPPAGESTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPPAGESTATUS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::default::Default for QACONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for QACONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pClientSite == other.pClientSite && self.pAdviseSink == other.pAdviseSink && self.pPropertyNotifySink == other.pPropertyNotifySink && self.pUnkEventSink == other.pUnkEventSink && self.dwAmbientFlags == other.dwAmbientFlags && self.colorFore == other.colorFore && self.colorBack == other.colorBack && self.pFont == other.pFont && self.pUndoMgr == other.pUndoMgr && self.dwAppearance == other.dwAppearance && self.lcid == other.lcid && self.hpal == other.hpal && self.pBindHost == other.pBindHost && self.pOleControlSite == other.pOleControlSite && self.pServiceProvider == other.pServiceProvider
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for QACONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QACONTAINER")
            .field("cbSize", &self.cbSize)
            .field("pClientSite", &self.pClientSite)
            .field("pAdviseSink", &self.pAdviseSink)
            .field("pPropertyNotifySink", &self.pPropertyNotifySink)
            .field("pUnkEventSink", &self.pUnkEventSink)
            .field("dwAmbientFlags", &self.dwAmbientFlags)
            .field("colorFore", &self.colorFore)
            .field("colorBack", &self.colorBack)
            .field("pFont", &self.pFont)
            .field("pUndoMgr", &self.pUndoMgr)
            .field("dwAppearance", &self.dwAppearance)
            .field("lcid", &self.lcid)
            .field("hpal", &self.hpal)
            .field("pBindHost", &self.pBindHost)
            .field("pOleControlSite", &self.pOleControlSite)
            .field("pServiceProvider", &self.pServiceProvider)
            .finish()
    }
}
impl ::core::default::Default for QACONTAINERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QACONTAINERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QACONTAINERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for QACONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QACONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMiscStatus == other.dwMiscStatus && self.dwViewStatus == other.dwViewStatus && self.dwEventCookie == other.dwEventCookie && self.dwPropNotifyCookie == other.dwPropNotifyCookie && self.dwPointerActivationPolicy == other.dwPointerActivationPolicy
    }
}
impl ::core::cmp::Eq for QACONTROL {}
impl ::core::fmt::Debug for QACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QACONTROL").field("cbSize", &self.cbSize).field("dwMiscStatus", &self.dwMiscStatus).field("dwViewStatus", &self.dwViewStatus).field("dwEventCookie", &self.dwEventCookie).field("dwPropNotifyCookie", &self.dwPropNotifyCookie).field("dwPointerActivationPolicy", &self.dwPointerActivationPolicy).finish()
    }
}
impl ::core::default::Default for READYSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READYSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for REGKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REGKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGKIND").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SAFEARR_BRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aRecord == other.aRecord
    }
}
impl ::core::cmp::Eq for SAFEARR_BRECORD {}
impl ::core::fmt::Debug for SAFEARR_BRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_BRECORD").field("Size", &self.Size).field("aRecord", &self.aRecord).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SAFEARR_BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aBstr == other.aBstr
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SAFEARR_BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_BSTR").field("Size", &self.Size).field("aBstr", &self.aBstr).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SAFEARR_DISPATCH {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apDispatch == other.apDispatch
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SAFEARR_DISPATCH {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SAFEARR_DISPATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_DISPATCH").field("Size", &self.Size).field("apDispatch", &self.apDispatch).finish()
    }
}
impl ::core::default::Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SAFEARR_HAVEIID {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown && self.iid == other.iid
    }
}
impl ::core::cmp::Eq for SAFEARR_HAVEIID {}
impl ::core::fmt::Debug for SAFEARR_HAVEIID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_HAVEIID").field("Size", &self.Size).field("apUnknown", &self.apUnknown).field("iid", &self.iid).finish()
    }
}
impl ::core::default::Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SAFEARR_UNKNOWN {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown
    }
}
impl ::core::cmp::Eq for SAFEARR_UNKNOWN {}
impl ::core::fmt::Debug for SAFEARR_UNKNOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_UNKNOWN").field("Size", &self.Size).field("apUnknown", &self.apUnknown).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SAFEARR_VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aVariant == other.aVariant
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SAFEARR_VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SAFEARR_VARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_VARIANT").field("Size", &self.Size).field("aVariant", &self.aVariant).finish()
    }
}
impl ::core::default::Default for SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TYPEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TYPEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for UASFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UASFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UASFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UDATE {
    fn eq(&self, other: &Self) -> bool {
        self.st == other.st && self.wDayOfYear == other.wDayOfYear
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for UDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UDATE").field("st", &self.st).field("wDayOfYear", &self.wDayOfYear).finish()
    }
}
impl ::core::default::Default for UI_CONVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UI_CONVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_CONVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UI_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UI_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UI_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for UPDFCACHE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPDFCACHE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDFCACHE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UPDFCACHE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UPDFCACHE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UPDFCACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for USERCLASSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USERCLASSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERCLASSTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARCMP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARCMP").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_FIRST_DAY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_FIRST_DAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_DAY").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_FIRST_WEEK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_FIRST_WEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_WEEK").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_GROUP").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_LEADING_DIGIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_LEADING_DIGIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_LEADING_DIGIT").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_NAMED_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_NAMED_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_NAMED_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for VARFORMAT_PARENTHESES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARFORMAT_PARENTHESES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_PARENTHESES").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIEWSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIEWSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIEWSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIEW_OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WPCSETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCSETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCSETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for XFORMCOORDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XFORMCOORDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XFORMCOORDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _wireBRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.pRecInfo == other.pRecInfo && self.pRecord == other.pRecord
    }
}
impl ::core::cmp::Eq for _wireBRECORD {}
impl ::core::fmt::Debug for _wireBRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_wireBRECORD").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("pRecInfo", &self.pRecInfo).field("pRecord", &self.pRecord).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
