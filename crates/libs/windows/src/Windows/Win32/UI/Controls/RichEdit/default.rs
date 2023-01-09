impl ::core::default::Default for BIDIOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BIDIOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wMask == other.wMask && self.wEffects == other.wEffects
    }
}
impl ::core::cmp::Eq for BIDIOPTIONS {}
impl ::core::fmt::Debug for BIDIOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIDIOPTIONS").field("cbSize", &self.cbSize).field("wMask", &self.wMask).field("wEffects", &self.wEffects).finish()
    }
}
impl ::core::default::Default for CARET_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CARET_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CARET_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CARET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CFE_EFFECTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CFE_EFFECTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CFE_EFFECTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CFE_EFFECTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CFE_EFFECTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CFE_EFFECTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CFE_EFFECTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CFE_EFFECTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CFM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CFM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CFM_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CFM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CFM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CFM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CFM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CFM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGENOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwChangeType == other.dwChangeType && self.pvCookieData == other.pvCookieData
    }
}
impl ::core::cmp::Eq for CHANGENOTIFY {}
impl ::core::fmt::Debug for CHANGENOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGENOTIFY").field("dwChangeType", &self.dwChangeType).field("pvCookieData", &self.pvCookieData).finish()
    }
}
impl ::core::default::Default for CHANGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGETYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHARFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHARFORMATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHARFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATA").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHARFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHARFORMATW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHARFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATW").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
impl ::core::default::Default for CHARRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHARRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.cpMin == other.cpMin && self.cpMax == other.cpMax
    }
}
impl ::core::cmp::Eq for CHARRANGE {}
impl ::core::fmt::Debug for CHARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARRANGE").field("cpMin", &self.cpMin).field("cpMax", &self.cpMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPBOARDFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.crText == other.crText && self.crBackground == other.crBackground && self.dwEffects == other.dwEffects
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPCOLOR").field("crText", &self.crText).field("crBackground", &self.crBackground).field("dwEffects", &self.dwEffects).finish()
    }
}
impl ::core::default::Default for EDITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCORRECTTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENDCOMPOSITIONNOTIFY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENDCOMPOSITIONNOTIFY_CODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENOLEOPFAILED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENPROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENSAVECLIPBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GETTEXTEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GETTEXTEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETTEXTEX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GETTEXTLENGTHEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GETTEXTLENGTHEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for GETTEXTLENGTHEX {}
impl ::core::fmt::Debug for GETTEXTLENGTHEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GETTEXTLENGTHEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
impl ::core::default::Default for GETTEXTLENGTHEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GETTEXTLENGTHEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETTEXTLENGTHEX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUPTYPINGCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HYPHRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HYPHRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.khyph == other.khyph && self.ichHyph == other.ichHyph && self.chHyph == other.chHyph
    }
}
impl ::core::cmp::Eq for HYPHRESULT {}
impl ::core::fmt::Debug for HYPHRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPHRESULT").field("khyph", &self.khyph).field("ichHyph", &self.ichHyph).field("chHyph", &self.chHyph).finish()
    }
}
impl ::core::default::Default for IMECOMPTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMECOMPTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IMECOMPTEXT {}
impl ::core::fmt::Debug for IMECOMPTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPTEXT").field("cb", &self.cb).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IMECOMPTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMECOMPTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMECOMPTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRichEditOle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOle {}
impl ::core::fmt::Debug for IRichEditOle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRichEditOleCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOleCallback {}
impl ::core::fmt::Debug for IRichEditOleCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOleCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRicheditUiaOverrides {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRicheditUiaOverrides {}
impl ::core::fmt::Debug for IRicheditUiaOverrides {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRicheditUiaOverrides").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDisplays {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDisplays {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDisplays {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDisplays").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2 {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSaved(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSaved)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSaved)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDefaultTabStop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultTabStop)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn New(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.New)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: tomConstants, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pvar, flags, codepage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: tomConstants, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pvar, flags, codepage).ok()
    }
    pub unsafe fn Freeze(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Freeze)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unfreeze(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Unfreeze)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginEditCollection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndEditCollection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Undo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Undo)(::windows::core::Vtable::as_raw(self), count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Redo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Redo)(::windows::core::Vtable::as_raw(self), count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Range)(::windows::core::Vtable::as_raw(self), cpactive, cpanchor, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), x, y, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument2Old {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument2Old {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument2Old {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2Old").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2Old {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSaved(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSaved)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSaved)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDefaultTabStop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultTabStop)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn New(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.New)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: tomConstants, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pvar, flags, codepage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: tomConstants, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pvar, flags, codepage).ok()
    }
    pub unsafe fn Freeze(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Freeze)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unfreeze(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Unfreeze)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginEditCollection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndEditCollection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Undo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Undo)(::windows::core::Vtable::as_raw(self), count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Redo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Redo)(::windows::core::Vtable::as_raw(self), count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Range)(::windows::core::Vtable::as_raw(self), cpactive, cpanchor, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), x, y, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextFont {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextFont2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextFont2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextFont2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextFont2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDuplicate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, pfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDuplicate)(::windows::core::Vtable::as_raw(self), pfont.into().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, pfont: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqual)(::windows::core::Vtable::as_raw(self), pfont.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStyle)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetAllCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAllCaps)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAllCaps)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetAnimation(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAnimation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAnimation)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetBackColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBackColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBackColor)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetBold(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBold)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBold(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBold)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmboss(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEmboss)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEmboss)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetForeColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetForeColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetForeColor)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetHidden(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHidden)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHidden)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetEngrave(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEngrave)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEngrave)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetItalic(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItalic)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItalic)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetKerning(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetKerning)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKerning)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetLanguageID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguageID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLanguageID)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn GetOutline(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutline)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPosition)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetProtected(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProtected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProtected)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetShadow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetShadow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetShadow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSize(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSize)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSmallCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSmallCaps)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSmallCaps)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSpacing)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSpacing)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetStrikeThrough(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStrikeThrough)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStrikeThrough)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSubscript(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSubscript)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSubscript)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSuperscript(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSuperscript)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSuperscript)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetUnderline(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnderline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUnderline)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetWeight(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWeight)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWeight)(::windows::core::Vtable::as_raw(self), value).ok()
    }
}
impl ::core::cmp::PartialEq for ITextHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost {}
impl ::core::fmt::Debug for ITextHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost2 {}
impl ::core::fmt::Debug for ITextHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost2").field(&self.0).finish()
    }
}
impl ITextHost2 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        (::windows::core::Vtable::vtable(self).base__.TxGetDC)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC<P0>(&self, hdc: P0) -> i32
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxReleaseDC)(::windows::core::Vtable::as_raw(self), hdc.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowScrollBar<P0>(&self, fnbar: i32, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxShowScrollBar)(::windows::core::Vtable::as_raw(self), fnbar, fshow.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.TxEnableScrollBar)(::windows::core::Vtable::as_raw(self), fusbflags, fuarrowflags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollRange<P0>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSetScrollRange)(::windows::core::Vtable::as_raw(self), fnbar, nminpos, nmaxpos, fredraw.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollPos<P0>(&self, fnbar: i32, npos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSetScrollPos)(::windows::core::Vtable::as_raw(self), fnbar, npos, fredraw.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxInvalidateRect<P0>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: P0)
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxInvalidateRect)(::windows::core::Vtable::as_raw(self), prc, fmode.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxViewChange<P0>(&self, fupdate: P0)
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxViewChange)(::windows::core::Vtable::as_raw(self), fupdate.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxCreateCaret<P0>(&self, hbmp: P0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxCreateCaret)(::windows::core::Vtable::as_raw(self), hbmp.into(), xwidth, yheight)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowCaret<P0>(&self, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxShowCaret)(::windows::core::Vtable::as_raw(self), fshow.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.TxSetCaretPos)(::windows::core::Vtable::as_raw(self), x, y)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.TxSetTimer)(::windows::core::Vtable::as_raw(self), idtimer, utimeout)
    }
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        (::windows::core::Vtable::vtable(self).base__.TxKillTimer)(::windows::core::Vtable::as_raw(self), idtimer)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx<P0>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: P0, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD)
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HRGN>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxScrollWindowEx)(::windows::core::Vtable::as_raw(self), dx, dy, lprcscroll, lprcclip, hrgnupdate.into(), lprcupdate, fuscroll)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCapture<P0>(&self, fcapture: P0)
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSetCapture)(::windows::core::Vtable::as_raw(self), fcapture.into())
    }
    pub unsafe fn TxSetFocus(&self) {
        (::windows::core::Vtable::vtable(self).base__.TxSetFocus)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor<P0, P1>(&self, hcur: P0, ftext: P1)
    where
        P0: ::std::convert::Into<super::super::WindowsAndMessaging::HCURSOR>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSetCursor)(::windows::core::Vtable::as_raw(self), hcur.into(), ftext.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.TxScreenToClient)(::windows::core::Vtable::as_raw(self), lppt)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.TxClientToScreen)(::windows::core::Vtable::as_raw(self), lppt)
    }
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxActivate)(::windows::core::Vtable::as_raw(self), ploldstate).ok()
    }
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxDeactivate)(::windows::core::Vtable::as_raw(self), lnewstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetClientRect)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetViewInset)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetCharFormat)(::windows::core::Vtable::as_raw(self), ppcf).ok()
    }
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetParaFormat)(::windows::core::Vtable::as_raw(self), pppf).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
        (::windows::core::Vtable::vtable(self).base__.TxGetSysColor)(::windows::core::Vtable::as_raw(self), nindex)
    }
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetBackStyle)(::windows::core::Vtable::as_raw(self), pstyle).ok()
    }
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetMaxLength)(::windows::core::Vtable::as_raw(self), plength).ok()
    }
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetScrollBars)(::windows::core::Vtable::as_raw(self), pdwscrollbar).ok()
    }
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows::core::Result<i8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TxGetPasswordChar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetAcceleratorPos)(::windows::core::Vtable::as_raw(self), pcp).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetExtent)(::windows::core::Vtable::as_raw(self), lpextent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxCharFormatChange)(::windows::core::Vtable::as_raw(self), pcf).ok()
    }
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxParaFormatChange)(::windows::core::Vtable::as_raw(self), ppf).ok()
    }
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetPropertyBits)(::windows::core::Vtable::as_raw(self), dwmask, pdwbits).ok()
    }
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxNotify)(::windows::core::Vtable::as_raw(self), inotify, pv).ok()
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        (::windows::core::Vtable::vtable(self).base__.TxImmGetContext)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmReleaseContext<P0>(&self, himc: P0)
    where
        P0: ::std::convert::Into<super::super::super::Globalization::HIMC>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxImmReleaseContext)(::windows::core::Vtable::as_raw(self), himc.into())
    }
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetSelectionBarWidth)(::windows::core::Vtable::as_raw(self), lselbarwidth).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextPara {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextPara {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextPara {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextPara2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextPara2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextPara2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextPara2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDuplicate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, ppara: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDuplicate)(::windows::core::Vtable::as_raw(self), ppara.into().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, ppara: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqual)(::windows::core::Vtable::as_raw(self), ppara.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStyle)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAlignment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAlignment)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetHyphenation(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHyphenation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHyphenation)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFirstLineIndent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetKeepTogether(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetKeepTogether)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeepTogether)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetKeepWithNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeepWithNext)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetLeftIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLeftIndent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLineSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacingRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetListAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetListAlignment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListAlignment)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetListLevelIndex(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetListLevelIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListLevelIndex)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetListStart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetListStart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListStart)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetListTab(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetListTab)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListTab)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetListType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetListType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetListType(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetListType)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetNoLineNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNoLineNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNoLineNumber)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPageBreakBefore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPageBreakBefore)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetRightIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRightIndent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRightIndent)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIndents)(::windows::core::Vtable::as_raw(self), first, left, right).ok()
    }
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), rule, spacing).ok()
    }
    pub unsafe fn GetSpaceAfter(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSpaceAfter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSpaceAfter)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpaceBefore(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSpaceBefore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSpaceBefore)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetWidowControl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidowControl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWidowControl)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetTabCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTabCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddTab)(::windows::core::Vtable::as_raw(self), tbpos, tbalign, tbleader).ok()
    }
    pub unsafe fn ClearAllTabs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearAllTabs)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteTab)(::windows::core::Vtable::as_raw(self), tbpos).ok()
    }
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTab)(::windows::core::Vtable::as_raw(self), itab, ptbpos, ptbalign, ptbleader).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRange {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRange2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextRange2 {
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetText(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetChar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetChar)(::windows::core::Vtable::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDuplicate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFormattedText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFormattedText)(::windows::core::Vtable::as_raw(self), prange.into().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStart)(::windows::core::Vtable::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEnd)(::windows::core::Vtable::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFont)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFont)(::windows::core::Vtable::as_raw(self), pfont.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPara)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPara)(::windows::core::Vtable::as_raw(self), ppara.into().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStoryLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStoryType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Collapse)(::windows::core::Vtable::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Expand)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetIndex)(::windows::core::Vtable::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRange)(::windows::core::Vtable::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.InRange)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.InStory)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsEqual)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.StartOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EndOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveStart)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveEnd)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveStartWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveEndWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveStartUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveEndUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindText(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextStart(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindTextStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextEnd(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindTextEnd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Cut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Copy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Paste)(::windows::core::Vtable::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CanPaste)(::windows::core::Vtable::as_raw(self), pvar, format, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CanEdit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ChangeCase)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPoint)(::windows::core::Vtable::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPoint)(::windows::core::Vtable::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEmbeddedObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFlags)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveLeft)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveRight)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveUp)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveDown)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HomeKey)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndKey)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TypeText(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TypeText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRow {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection {
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetText(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChar)(::windows::core::Vtable::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDuplicate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFormattedText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFormattedText)(::windows::core::Vtable::as_raw(self), prange.into().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStart)(::windows::core::Vtable::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEnd)(::windows::core::Vtable::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFont)(::windows::core::Vtable::as_raw(self), pfont.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPara)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPara)(::windows::core::Vtable::as_raw(self), ppara.into().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStoryType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Collapse)(::windows::core::Vtable::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Expand)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIndex)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIndex)(::windows::core::Vtable::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRange)(::windows::core::Vtable::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.InRange)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.InStory)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqual)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StartOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveStart)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveEnd)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveStartWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveEndWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveStartUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveEndUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindText(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextStart(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindTextStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextEnd(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindTextEnd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Cut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Copy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Paste)(::windows::core::Vtable::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanPaste)(::windows::core::Vtable::as_raw(self), pvar, format, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanEdit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeCase)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPoint)(::windows::core::Vtable::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPoint)(::windows::core::Vtable::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEmbeddedObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextSelection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextSelection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextSelection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection2 {
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetText(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetChar)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetChar)(::windows::core::Vtable::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDuplicate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFormattedText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFormattedText)(::windows::core::Vtable::as_raw(self), prange.into().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetStart)(::windows::core::Vtable::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetEnd)(::windows::core::Vtable::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFont)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFont)(::windows::core::Vtable::as_raw(self), pfont.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPara)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPara)(::windows::core::Vtable::as_raw(self), ppara.into().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStoryLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStoryType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Collapse)(::windows::core::Vtable::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Expand)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self), unit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetIndex)(::windows::core::Vtable::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRange)(::windows::core::Vtable::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InRange)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InStory)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsEqual)(::windows::core::Vtable::as_raw(self), prange.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StartOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndOf)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveStart)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveEnd)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveStartWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveEndWhile)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveStartUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MoveEndUntil)(::windows::core::Vtable::as_raw(self), cset, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindText(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextStart(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindTextStart)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindTextEnd(&self, bstr: &::windows::core::BSTR, count: i32, flags: tomConstants) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindTextEnd)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Delete)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Cut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Copy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Paste)(::windows::core::Vtable::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CanPaste)(::windows::core::Vtable::as_raw(self), pvar, format, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CanEdit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ChangeCase)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPoint)(::windows::core::Vtable::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPoint)(::windows::core::Vtable::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEmbeddedObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFlags)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveLeft)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveRight)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveUp)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveDown)(::windows::core::Vtable::as_raw(self), unit, count, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HomeKey)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EndKey)(::windows::core::Vtable::as_raw(self), unit, extend, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TypeText(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TypeText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn GetCch(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCells(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCells)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetColumn)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDuplicate2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont2(&self) -> ::windows::core::Result<ITextFont2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont2<P0>(&self, pfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextFont2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFont2)(::windows::core::Vtable::as_raw(self), pfont.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFormattedText2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText2<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFormattedText2)(::windows::core::Vtable::as_raw(self), prange.into().abi()).ok()
    }
    pub unsafe fn GetGravity(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGravity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGravity)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara2(&self) -> ::windows::core::Result<ITextPara2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPara2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara2<P0>(&self, ppara: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextPara2>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPara2)(::windows::core::Vtable::as_raw(self), ppara.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRow(&self) -> ::windows::core::Result<ITextRow> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStartPara(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStartPara)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTable(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetURL(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetURL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetURL(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddSubrange)(::windows::core::Vtable::as_raw(self), cp1, cp2, activate).ok()
    }
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BuildUpMath)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteSubrange)(::windows::core::Vtable::as_raw(self), cpfirst, cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Find<P0>(&self, prange: P0, count: i32, flags: i32) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRange2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Find)(::windows::core::Vtable::as_raw(self), prange.into().abi(), count, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetChar2)(::windows::core::Vtable::as_raw(self), pchar, offset).ok()
    }
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDropCap)(::windows::core::Vtable::as_raw(self), pcline, pposition).ok()
    }
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInlineObject)(::windows::core::Vtable::as_raw(self), ptype, palign, pchar, pchar1, pchar2, pcount, ptexstyle, pccol, plevel).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProperty)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRect)(::windows::core::Vtable::as_raw(self), r#type, pleft, ptop, pright, pbottom, phit).ok()
    }
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSubrange)(::windows::core::Vtable::as_raw(self), isubrange, pcpfirst, pcplim).ok()
    }
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetText2)(::windows::core::Vtable::as_raw(self), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HexToUnicode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HexToUnicode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InsertTable)(::windows::core::Vtable::as_raw(self), ccol, crow, autofit).ok()
    }
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Linearize)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetActiveSubrange)(::windows::core::Vtable::as_raw(self), cpanchor, cpactive).ok()
    }
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDropCap)(::windows::core::Vtable::as_raw(self), cline, position).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProperty)(::windows::core::Vtable::as_raw(self), r#type, value).ok()
    }
    pub unsafe fn SetText2(&self, flags: i32, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetText2)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn UnicodeToHex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnicodeToHex)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInlineObject)(::windows::core::Vtable::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol).ok()
    }
    pub unsafe fn GetMathFunctionType(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMathFunctionType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn InsertImage<P0>(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: &::windows::core::BSTR, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InsertImage)(::windows::core::Vtable::as_raw(self), width, height, ascent, r#type, ::core::mem::transmute_copy(bstralttext), pstream.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ITextServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices {}
impl ::core::fmt::Debug for ITextServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextServices2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices2 {}
impl ::core::fmt::Debug for ITextServices2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices2").field(&self.0).finish()
    }
}
impl ITextServices2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSendMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSendMessage)(::windows::core::Vtable::as_raw(self), msg, wparam.into(), lparam.into(), plresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxDraw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into(), hictargetdev.into(), lprcbounds, lprcwbounds, lprcupdate, pfncontinue, dwcontinue, lviewid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetHScroll)(::windows::core::Vtable::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetVScroll)(::windows::core::Vtable::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnTxSetCursor)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into(), hictargetdev.into(), lprcclient, x, y).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxQueryHitPoint)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into(), hictargetdev.into(), lprcclient, x, y, phitresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxInPlaceActivate)(::windows::core::Vtable::as_raw(self), prcclient).ok()
    }
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxUIDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TxGetText(&self, pbstrtext: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    pub unsafe fn TxSetText<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxSetText)(::windows::core::Vtable::as_raw(self), psztext.into().abi()).ok()
    }
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetCurTargetX)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetBaseLinePos)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize<P0, P1>(&self, dwaspect: u32, hdcdraw: P0, hictargetdev: P1, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.TxGetNaturalSize)(::windows::core::Vtable::as_raw(self), dwaspect, hdcdraw.into(), hictargetdev.into(), ptd, dwmode, psizelextent, pwidth, pheight).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows::core::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TxGetDropTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnTxPropertyBitsChange)(::windows::core::Vtable::as_raw(self), dwmask, dwbits).ok()
    }
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TxGetCachedSize)(::windows::core::Vtable::as_raw(self), pdwwidth, pdwheight).ok()
    }
}
impl ::core::cmp::PartialEq for ITextStory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStory {}
impl ::core::fmt::Debug for ITextStory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStoryRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStoryRanges {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStoryRanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStoryRanges2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStoryRanges2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStoryRanges2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges2 {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStrings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStrings").field(&self.0).finish()
    }
}
impl ::core::default::Default for KHYPH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KHYPH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KHYPH").field(&self.0).finish()
    }
}
impl ::core::default::Default for MANCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MANCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANCODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSGFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OBJECTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PARAFORMAT_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PARAFORMAT_BORDERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_BORDERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_BORDERS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_BORDERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_BORDERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_BORDERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PARAFORMAT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PARAFORMAT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PARAFORMAT_NUMBERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_NUMBERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_NUMBERING").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_NUMBERING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_NUMBERING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PARAFORMAT_NUMBERING_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_NUMBERING_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_NUMBERING_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PARAFORMAT_SHADING_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARAFORMAT_SHADING_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_SHADING_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for REOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for REOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cp == other.cp && self.clsid == other.clsid && self.poleobj == other.poleobj && self.pstg == other.pstg && self.polesite == other.polesite && self.sizel == other.sizel && self.dvaspect == other.dvaspect && self.dwFlags == other.dwFlags && self.dwUser == other.dwUser
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for REOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REOBJECT").field("cbStruct", &self.cbStruct).field("cp", &self.cp).field("clsid", &self.clsid).field("poleobj", &self.poleobj).field("pstg", &self.pstg).field("polesite", &self.polesite).field("sizel", &self.sizel).field("dvaspect", &self.dvaspect).field("dwFlags", &self.dwFlags).field("dwUser", &self.dwUser).finish()
    }
}
impl ::core::default::Default for REOBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REOBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REOBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for REPASTESPECIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQRESIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::default::Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RICH_EDIT_GET_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RICH_EDIT_GET_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RICH_EDIT_GET_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SETTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for SETTEXTEX {}
impl ::core::fmt::Debug for SETTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SETTEXTEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TABLECELLPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TABLECELLPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dxWidth == other.dxWidth && self._bitfield == other._bitfield && self.wShading == other.wShading && self.dxBrdrLeft == other.dxBrdrLeft && self.dyBrdrTop == other.dyBrdrTop && self.dxBrdrRight == other.dxBrdrRight && self.dyBrdrBottom == other.dyBrdrBottom && self.crBrdrLeft == other.crBrdrLeft && self.crBrdrTop == other.crBrdrTop && self.crBrdrRight == other.crBrdrRight && self.crBrdrBottom == other.crBrdrBottom && self.crBackPat == other.crBackPat && self.crForePat == other.crForePat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TABLECELLPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TABLECELLPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLECELLPARMS")
            .field("dxWidth", &self.dxWidth)
            .field("_bitfield", &self._bitfield)
            .field("wShading", &self.wShading)
            .field("dxBrdrLeft", &self.dxBrdrLeft)
            .field("dyBrdrTop", &self.dyBrdrTop)
            .field("dxBrdrRight", &self.dxBrdrRight)
            .field("dyBrdrBottom", &self.dyBrdrBottom)
            .field("crBrdrLeft", &self.crBrdrLeft)
            .field("crBrdrTop", &self.crBrdrTop)
            .field("crBrdrRight", &self.crBrdrRight)
            .field("crBrdrBottom", &self.crBrdrBottom)
            .field("crBackPat", &self.crBackPat)
            .field("crForePat", &self.crForePat)
            .finish()
    }
}
impl ::core::default::Default for TABLEROWPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TABLEROWPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbRow == other.cbRow && self.cbCell == other.cbCell && self.cCell == other.cCell && self.cRow == other.cRow && self.dxCellMargin == other.dxCellMargin && self.dxIndent == other.dxIndent && self.dyHeight == other.dyHeight && self._bitfield == other._bitfield && self.cpStartRow == other.cpStartRow && self.bTableLevel == other.bTableLevel && self.iCell == other.iCell
    }
}
impl ::core::cmp::Eq for TABLEROWPARMS {}
impl ::core::fmt::Debug for TABLEROWPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLEROWPARMS").field("cbRow", &self.cbRow).field("cbCell", &self.cbCell).field("cCell", &self.cCell).field("cRow", &self.cRow).field("dxCellMargin", &self.dxCellMargin).field("dxIndent", &self.dxIndent).field("dyHeight", &self.dyHeight).field("_bitfield", &self._bitfield).field("cpStartRow", &self.cpStartRow).field("bTableLevel", &self.bTableLevel).field("iCell", &self.iCell).finish()
    }
}
impl ::core::default::Default for TEXTMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXTMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXTBACKSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXTBACKSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTBACKSTYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXTHITRESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXTHITRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTHITRESULT").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXTNATURALSIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXTNATURALSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTNATURALSIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXTVIEW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXTVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTVIEW").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNDONAMEID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNDONAMEID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNDONAMEID").field(&self.0).finish()
    }
}
impl ::core::default::Default for tomConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for tomConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("tomConstants").field(&self.0).finish()
    }
}
