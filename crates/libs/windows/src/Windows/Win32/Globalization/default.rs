impl ::core::default::Default for CHARSETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHARSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ciCharset == other.ciCharset && self.ciACP == other.ciACP && self.fs == other.fs
    }
}
impl ::core::cmp::Eq for CHARSETINFO {}
impl ::core::fmt::Debug for CHARSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARSETINFO").field("ciCharset", &self.ciCharset).field("ciACP", &self.ciACP).field("fs", &self.fs).finish()
    }
}
impl ::core::default::Default for COMPARE_STRING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPARE_STRING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPARE_STRING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMPARE_STRING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMPARE_STRING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CORRECTIVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CORRECTIVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CORRECTIVE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize && self.DefaultChar == other.DefaultChar && self.LeadByte == other.LeadByte
    }
}
impl ::core::cmp::Eq for CPINFO {}
impl ::core::fmt::Debug for CPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFO").field("MaxCharSize", &self.MaxCharSize).field("DefaultChar", &self.DefaultChar).field("LeadByte", &self.LeadByte).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize && self.DefaultChar == other.DefaultChar && self.LeadByte == other.LeadByte && self.UnicodeDefaultChar == other.UnicodeDefaultChar && self.CodePage == other.CodePage && self.CodePageName == other.CodePageName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFOEXA").field("MaxCharSize", &self.MaxCharSize).field("DefaultChar", &self.DefaultChar).field("LeadByte", &self.LeadByte).field("UnicodeDefaultChar", &self.UnicodeDefaultChar).field("CodePage", &self.CodePage).field("CodePageName", &self.CodePageName).finish()
    }
}
impl ::core::default::Default for CPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize && self.DefaultChar == other.DefaultChar && self.LeadByte == other.LeadByte && self.UnicodeDefaultChar == other.UnicodeDefaultChar && self.CodePage == other.CodePage && self.CodePageName == other.CodePageName
    }
}
impl ::core::cmp::Eq for CPINFOEXW {}
impl ::core::fmt::Debug for CPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFOEXW").field("MaxCharSize", &self.MaxCharSize).field("DefaultChar", &self.DefaultChar).field("LeadByte", &self.LeadByte).field("UnicodeDefaultChar", &self.UnicodeDefaultChar).field("CodePage", &self.CodePage).field("CodePageName", &self.CodePageName).finish()
    }
}
impl ::core::default::Default for CURRENCYFMTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CURRENCYFMTA {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits && self.LeadingZero == other.LeadingZero && self.Grouping == other.Grouping && self.lpDecimalSep == other.lpDecimalSep && self.lpThousandSep == other.lpThousandSep && self.NegativeOrder == other.NegativeOrder && self.PositiveOrder == other.PositiveOrder && self.lpCurrencySymbol == other.lpCurrencySymbol
    }
}
impl ::core::cmp::Eq for CURRENCYFMTA {}
impl ::core::fmt::Debug for CURRENCYFMTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURRENCYFMTA").field("NumDigits", &self.NumDigits).field("LeadingZero", &self.LeadingZero).field("Grouping", &self.Grouping).field("lpDecimalSep", &self.lpDecimalSep).field("lpThousandSep", &self.lpThousandSep).field("NegativeOrder", &self.NegativeOrder).field("PositiveOrder", &self.PositiveOrder).field("lpCurrencySymbol", &self.lpCurrencySymbol).finish()
    }
}
impl ::core::default::Default for CURRENCYFMTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CURRENCYFMTW {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits && self.LeadingZero == other.LeadingZero && self.Grouping == other.Grouping && self.lpDecimalSep == other.lpDecimalSep && self.lpThousandSep == other.lpThousandSep && self.NegativeOrder == other.NegativeOrder && self.PositiveOrder == other.PositiveOrder && self.lpCurrencySymbol == other.lpCurrencySymbol
    }
}
impl ::core::cmp::Eq for CURRENCYFMTW {}
impl ::core::fmt::Debug for CURRENCYFMTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURRENCYFMTW").field("NumDigits", &self.NumDigits).field("LeadingZero", &self.LeadingZero).field("Grouping", &self.Grouping).field("lpDecimalSep", &self.lpDecimalSep).field("lpThousandSep", &self.lpThousandSep).field("NegativeOrder", &self.NegativeOrder).field("PositiveOrder", &self.PositiveOrder).field("lpCurrencySymbol", &self.lpCurrencySymbol).finish()
    }
}
impl ::core::default::Default for DetectEncodingInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DetectEncodingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.nLangID == other.nLangID && self.nCodePage == other.nCodePage && self.nDocPercent == other.nDocPercent && self.nConfidence == other.nConfidence
    }
}
impl ::core::cmp::Eq for DetectEncodingInfo {}
impl ::core::fmt::Debug for DetectEncodingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DetectEncodingInfo").field("nLangID", &self.nLangID).field("nCodePage", &self.nCodePage).field("nDocPercent", &self.nDocPercent).field("nConfidence", &self.nConfidence).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ENUMTEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ENUMTEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.etmNewTextMetricEx == other.etmNewTextMetricEx && self.etmAxesList == other.etmAxesList
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ENUMTEXTMETRICA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ENUMTEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTEXTMETRICA").field("etmNewTextMetricEx", &self.etmNewTextMetricEx).field("etmAxesList", &self.etmAxesList).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ENUMTEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ENUMTEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.etmNewTextMetricEx == other.etmNewTextMetricEx && self.etmAxesList == other.etmAxesList
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ENUMTEXTMETRICW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ENUMTEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTEXTMETRICW").field("etmNewTextMetricEx", &self.etmNewTextMetricEx).field("etmAxesList", &self.etmAxesList).finish()
    }
}
impl ::core::default::Default for ENUM_DATE_FORMATS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_DATE_FORMATS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_DATE_FORMATS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_CODE_PAGES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILEMUIINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILEMUIINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVersion == other.dwVersion && self.dwFileType == other.dwFileType && self.pChecksum == other.pChecksum && self.pServiceChecksum == other.pServiceChecksum && self.dwLanguageNameOffset == other.dwLanguageNameOffset && self.dwTypeIDMainSize == other.dwTypeIDMainSize && self.dwTypeIDMainOffset == other.dwTypeIDMainOffset && self.dwTypeNameMainOffset == other.dwTypeNameMainOffset && self.dwTypeIDMUISize == other.dwTypeIDMUISize && self.dwTypeIDMUIOffset == other.dwTypeIDMUIOffset && self.dwTypeNameMUIOffset == other.dwTypeNameMUIOffset && self.abBuffer == other.abBuffer
    }
}
impl ::core::cmp::Eq for FILEMUIINFO {}
impl ::core::fmt::Debug for FILEMUIINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILEMUIINFO")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFileType", &self.dwFileType)
            .field("pChecksum", &self.pChecksum)
            .field("pServiceChecksum", &self.pServiceChecksum)
            .field("dwLanguageNameOffset", &self.dwLanguageNameOffset)
            .field("dwTypeIDMainSize", &self.dwTypeIDMainSize)
            .field("dwTypeIDMainOffset", &self.dwTypeIDMainOffset)
            .field("dwTypeNameMainOffset", &self.dwTypeNameMainOffset)
            .field("dwTypeIDMUISize", &self.dwTypeIDMUISize)
            .field("dwTypeIDMUIOffset", &self.dwTypeIDMUIOffset)
            .field("dwTypeNameMUIOffset", &self.dwTypeNameMUIOffset)
            .field("abBuffer", &self.abBuffer)
            .finish()
    }
}
impl ::core::default::Default for FOLD_STRING_MAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLD_STRING_MAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLD_STRING_MAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FOLD_STRING_MAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FOLD_STRING_MAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FONTSIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FONTSIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.fsUsb == other.fsUsb && self.fsCsb == other.fsCsb
    }
}
impl ::core::cmp::Eq for FONTSIGNATURE {}
impl ::core::fmt::Debug for FONTSIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTSIGNATURE").field("fsUsb", &self.fsUsb).field("fsCsb", &self.fsCsb).finish()
    }
}
impl ::core::default::Default for GOFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GOFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.du == other.du && self.dv == other.dv
    }
}
impl ::core::cmp::Eq for GOFFSET {}
impl ::core::fmt::Debug for GOFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOFFSET").field("du", &self.du).field("dv", &self.dv).finish()
    }
}
impl ::core::cmp::PartialEq for IComprehensiveSpellCheckProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComprehensiveSpellCheckProvider {}
impl ::core::fmt::Debug for IComprehensiveSpellCheckProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComprehensiveSpellCheckProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumCodePage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCodePage {}
impl ::core::fmt::Debug for IEnumCodePage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCodePage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumRfc1766 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRfc1766 {}
impl ::core::fmt::Debug for IEnumRfc1766 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRfc1766").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumScript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumScript {}
impl ::core::fmt::Debug for IEnumScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumScript").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSpellingError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpellingError {}
impl ::core::fmt::Debug for IEnumSpellingError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSpellingError").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangCodePages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangCodePages {}
impl ::core::fmt::Debug for IMLangCodePages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangCodePages").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangConvertCharset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangConvertCharset {}
impl ::core::fmt::Debug for IMLangConvertCharset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangConvertCharset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangFontLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangFontLink {}
impl ::core::fmt::Debug for IMLangFontLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangFontLink").field(&self.0).finish()
    }
}
impl IMLangFontLink {
    pub unsafe fn GetCharCodePages(&self, chsrc: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCharCodePages)(::windows::core::Vtable::as_raw(self), chsrc, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStrCodePages(&self, pszsrc: &[u16], dwprioritycodepages: u32, pdwcodepages: ::core::option::Option<*mut u32>, pcchcodepages: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStrCodePages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszsrc.as_ptr()), pszsrc.len() as _, dwprioritycodepages, ::core::mem::transmute(pdwcodepages.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcchcodepages.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CodePageToCodePages(&self, ucodepage: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CodePageToCodePages)(::windows::core::Vtable::as_raw(self), ucodepage, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CodePagesToCodePage)(::windows::core::Vtable::as_raw(self), dwcodepages, udefaultcodepage, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMLangFontLink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangFontLink2 {}
impl ::core::fmt::Debug for IMLangFontLink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangFontLink2").field(&self.0).finish()
    }
}
impl IMLangFontLink2 {
    pub unsafe fn GetCharCodePages(&self, chsrc: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCharCodePages)(::windows::core::Vtable::as_raw(self), chsrc, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStrCodePages(&self, pszsrc: &[u16], dwprioritycodepages: u32, pdwcodepages: ::core::option::Option<*mut u32>, pcchcodepages: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStrCodePages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszsrc.as_ptr()), pszsrc.len() as _, dwprioritycodepages, ::core::mem::transmute(pdwcodepages.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcchcodepages.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CodePageToCodePages(&self, ucodepage: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CodePageToCodePages)(::windows::core::Vtable::as_raw(self), ucodepage, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CodePagesToCodePage)(::windows::core::Vtable::as_raw(self), dwcodepages, udefaultcodepage, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMLangLineBreakConsole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangLineBreakConsole {}
impl ::core::fmt::Debug for IMLangLineBreakConsole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangLineBreakConsole").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangString {}
impl ::core::fmt::Debug for IMLangString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangString").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangStringAStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringAStr {}
impl ::core::fmt::Debug for IMLangStringAStr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringAStr").field(&self.0).finish()
    }
}
impl IMLangStringAStr {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sync<P0>(&self, fnoaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Sync)(::windows::core::Vtable::as_raw(self), fnoaccess.into()).ok()
    }
    pub unsafe fn GetLength(&self, pllen: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pllen.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetMLStr<P0>(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMLStr)(::windows::core::Vtable::as_raw(self), ldestpos, ldestlen, psrcmlstr.into().abi(), lsrcpos, lsrclen).ok()
    }
    pub unsafe fn GetMLStr<P0>(&self, lsrcpos: i32, lsrclen: i32, punkouter: P0, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut ::core::option::Option<::windows::core::IUnknown>, pldestpos: ::core::option::Option<*mut i32>, pldestlen: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMLStr)(::windows::core::Vtable::as_raw(self), lsrcpos, lsrclen, punkouter.into().abi(), dwclscontext, piid, ::core::mem::transmute(ppdestmlstr), ::core::mem::transmute(pldestpos.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pldestlen.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IMLangStringBufA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringBufA {}
impl ::core::fmt::Debug for IMLangStringBufA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringBufA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangStringBufW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringBufW {}
impl ::core::fmt::Debug for IMLangStringBufW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringBufW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLangStringWStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringWStr {}
impl ::core::fmt::Debug for IMLangStringWStr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringWStr").field(&self.0).finish()
    }
}
impl IMLangStringWStr {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sync<P0>(&self, fnoaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Sync)(::windows::core::Vtable::as_raw(self), fnoaccess.into()).ok()
    }
    pub unsafe fn GetLength(&self, pllen: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pllen.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetMLStr<P0>(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMLStr)(::windows::core::Vtable::as_raw(self), ldestpos, ldestlen, psrcmlstr.into().abi(), lsrcpos, lsrclen).ok()
    }
    pub unsafe fn GetMLStr<P0>(&self, lsrcpos: i32, lsrclen: i32, punkouter: P0, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut ::core::option::Option<::windows::core::IUnknown>, pldestpos: ::core::option::Option<*mut i32>, pldestlen: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMLStr)(::windows::core::Vtable::as_raw(self), lsrcpos, lsrclen, punkouter.into().abi(), dwclscontext, piid, ::core::mem::transmute(ppdestmlstr), ::core::mem::transmute(pldestpos.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pldestlen.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IMultiLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiLanguage {}
impl ::core::fmt::Debug for IMultiLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMultiLanguage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiLanguage2 {}
impl ::core::fmt::Debug for IMultiLanguage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMultiLanguage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiLanguage3 {}
impl ::core::fmt::Debug for IMultiLanguage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage3").field(&self.0).finish()
    }
}
impl IMultiLanguage3 {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNumberOfCodePageInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodePageInfo)(::windows::core::Vtable::as_raw(self), uicodepage, langid, pcodepageinfo).ok()
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFamilyCodePage)(::windows::core::Vtable::as_raw(self), uicodepage, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32, langid: u16) -> ::windows::core::Result<IEnumCodePage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumCodePages)(::windows::core::Vtable::as_raw(self), grfflags, langid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCharsetInfo(&self, charset: &::windows::core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCharsetInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(charset), pcharsetinfo).ok()
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsConvertible)(::windows::core::Vtable::as_raw(self), dwsrcencoding, dwdstencoding).ok()
    }
    pub unsafe fn ConvertString(&self, pdwmode: ::core::option::Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: ::core::option::Option<*const u8>, pcsrcsize: ::core::option::Option<*mut u32>, pdststr: ::core::option::Option<*mut u8>, pcdstsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ConvertString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwsrcencoding, dwdstencoding, ::core::mem::transmute(psrcstr.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pcsrcsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdststr.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcdstsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringToUnicode<P0>(&self, pdwmode: ::core::option::Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: ::core::option::Option<*mut u32>, pdststr: ::windows::core::PWSTR, pcdstsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringToUnicode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwencoding, psrcstr.into().abi(), ::core::mem::transmute(pcsrcsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdststr), ::core::mem::transmute(pcdstsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringFromUnicode<P0>(&self, pdwmode: ::core::option::Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: ::core::option::Option<*mut u32>, pdststr: ::windows::core::PSTR, pcdstsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringFromUnicode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwencoding, psrcstr.into().abi(), ::core::mem::transmute(pcsrcsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdststr), ::core::mem::transmute(pcdstsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertStringReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringReset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetRfc1766FromLcid(&self, locale: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRfc1766FromLcid)(::windows::core::Vtable::as_raw(self), locale, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLcidFromRfc1766)(::windows::core::Vtable::as_raw(self), plocale, ::core::mem::transmute_copy(bstrrfc1766)).ok()
    }
    pub unsafe fn EnumRfc1766(&self, langid: u16) -> ::windows::core::Result<IEnumRfc1766> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumRfc1766)(::windows::core::Vtable::as_raw(self), langid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRfc1766Info(&self, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRfc1766Info)(::windows::core::Vtable::as_raw(self), locale, langid, prfc1766info).ok()
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::Result<IMLangConvertCharset> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateConvertCharset)(::windows::core::Vtable::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConvertStringInIStream<P0, P1, P2>(&self, pdwmode: ::core::option::Option<*mut u32>, dwflag: u32, lpfallback: P0, dwsrcencoding: u32, dwdstencoding: u32, pstmin: P1, pstmout: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringInIStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwflag, lpfallback.into().abi(), dwsrcencoding, dwdstencoding, pstmin.into().abi(), pstmout.into().abi()).ok()
    }
    pub unsafe fn ConvertStringToUnicodeEx<P0, P1>(&self, pdwmode: ::core::option::Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: ::core::option::Option<*mut u32>, pdststr: ::windows::core::PWSTR, pcdstsize: ::core::option::Option<*mut u32>, dwflag: u32, lpfallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringToUnicodeEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwencoding, psrcstr.into().abi(), ::core::mem::transmute(pcsrcsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdststr), ::core::mem::transmute(pcdstsize.unwrap_or(::std::ptr::null_mut())), dwflag, lpfallback.into().abi()).ok()
    }
    pub unsafe fn ConvertStringFromUnicodeEx<P0, P1>(&self, pdwmode: ::core::option::Option<*mut u32>, dwencoding: u32, psrcstr: P0, pcsrcsize: ::core::option::Option<*mut u32>, pdststr: ::windows::core::PSTR, pcdstsize: ::core::option::Option<*mut u32>, dwflag: u32, lpfallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertStringFromUnicodeEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdwmode.unwrap_or(::std::ptr::null_mut())), dwencoding, psrcstr.into().abi(), ::core::mem::transmute(pcsrcsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdststr), ::core::mem::transmute(pcdstsize.unwrap_or(::std::ptr::null_mut())), dwflag, lpfallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectCodepageInIStream<P0>(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: P0, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DetectCodepageInIStream)(::windows::core::Vtable::as_raw(self), dwflag, dwprefwincodepage, pstmin.into().abi(), lpencoding, pnscores).ok()
    }
    pub unsafe fn DetectInputCodepage<P0>(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: P0, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DetectInputCodepage)(::windows::core::Vtable::as_raw(self), dwflag, dwprefwincodepage, psrcstr.into().abi(), pcsrcsize, lpencoding, pnscores).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateCodePage<P0>(&self, uicodepage: u32, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ValidateCodePage)(::windows::core::Vtable::as_raw(self), uicodepage, hwnd.into()).ok()
    }
    pub unsafe fn GetCodePageDescription(&self, uicodepage: u32, lcid: u32, lpwidecharstr: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodePageDescription)(::windows::core::Vtable::as_raw(self), uicodepage, lcid, ::core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len() as _).ok()
    }
    pub unsafe fn IsCodePageInstallable(&self, uicodepage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsCodePageInstallable)(::windows::core::Vtable::as_raw(self), uicodepage).ok()
    }
    pub unsafe fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMimeDBSource)(::windows::core::Vtable::as_raw(self), dwsource).ok()
    }
    pub unsafe fn GetNumberOfScripts(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNumberOfScripts)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumScripts(&self, dwflags: u32, langid: u16) -> ::windows::core::Result<IEnumScript> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumScripts)(::windows::core::Vtable::as_raw(self), dwflags, langid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateCodePageEx<P0>(&self, uicodepage: u32, hwnd: P0, dwfiodcontrol: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ValidateCodePageEx)(::windows::core::Vtable::as_raw(self), uicodepage, hwnd.into(), dwfiodcontrol).ok()
    }
}
impl ::core::cmp::PartialEq for IOptionDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOptionDescription {}
impl ::core::fmt::Debug for IOptionDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOptionDescription").field(&self.0).finish()
    }
}
impl ::core::default::Default for IS_TEXT_UNICODE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IS_TEXT_UNICODE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IS_TEXT_UNICODE_RESULT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IS_TEXT_UNICODE_RESULT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IS_TEXT_UNICODE_RESULT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IS_VALID_LOCALE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IS_VALID_LOCALE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IS_VALID_LOCALE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellCheckProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellCheckProvider {}
impl ::core::fmt::Debug for ISpellCheckProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellCheckProviderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellCheckProviderFactory {}
impl ::core::fmt::Debug for ISpellCheckProviderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckProviderFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellChecker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellChecker {}
impl ::core::fmt::Debug for ISpellChecker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellChecker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellChecker2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellChecker2 {}
impl ::core::fmt::Debug for ISpellChecker2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellChecker2").field(&self.0).finish()
    }
}
impl ISpellChecker2 {
    pub unsafe fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LanguageTag)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> ::windows::core::Result<IEnumSpellingError>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Check)(::windows::core::Vtable::as_raw(self), text.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> ::windows::core::Result<super::System::Com::IEnumString>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Suggest)(::windows::core::Vtable::as_raw(self), word.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, word: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Add)(::windows::core::Vtable::as_raw(self), word.into().abi()).ok()
    }
    pub unsafe fn Ignore<P0>(&self, word: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Ignore)(::windows::core::Vtable::as_raw(self), word.into().abi()).ok()
    }
    pub unsafe fn AutoCorrect<P0, P1>(&self, from: P0, to: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AutoCorrect)(::windows::core::Vtable::as_raw(self), from.into().abi(), to.into().abi()).ok()
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptionValue)(::windows::core::Vtable::as_raw(self), optionid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OptionIds(&self) -> ::windows::core::Result<super::System::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OptionIds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocalizedName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalizedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn add_SpellCheckerChanged<P0>(&self, handler: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISpellCheckerChangedEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.add_SpellCheckerChanged)(::windows::core::Vtable::as_raw(self), handler.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.remove_SpellCheckerChanged)(::windows::core::Vtable::as_raw(self), eventcookie).ok()
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> ::windows::core::Result<IOptionDescription>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOptionDescription)(::windows::core::Vtable::as_raw(self), optionid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> ::windows::core::Result<IEnumSpellingError>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComprehensiveCheck)(::windows::core::Vtable::as_raw(self), text.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpellCheckerChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellCheckerChangedEventHandler {}
impl ::core::fmt::Debug for ISpellCheckerChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckerChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellCheckerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellCheckerFactory {}
impl ::core::fmt::Debug for ISpellCheckerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckerFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpellingError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellingError {}
impl ::core::fmt::Debug for ISpellingError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellingError").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUserDictionariesRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDictionariesRegistrar {}
impl ::core::fmt::Debug for IUserDictionariesRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDictionariesRegistrar").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOCALESIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALESIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.lsUsb == other.lsUsb && self.lsCsbDefault == other.lsCsbDefault && self.lsCsbSupported == other.lsCsbSupported
    }
}
impl ::core::cmp::Eq for LOCALESIGNATURE {}
impl ::core::fmt::Debug for LOCALESIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALESIGNATURE").field("lsUsb", &self.lsUsb).field("lsCsbDefault", &self.lsCsbDefault).field("lsCsbSupported", &self.lsCsbSupported).finish()
    }
}
impl ::core::default::Default for MAPPING_DATA_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPPING_DATA_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwStartIndex == other.dwStartIndex && self.dwEndIndex == other.dwEndIndex && self.pszDescription == other.pszDescription && self.dwDescriptionLength == other.dwDescriptionLength && self.pData == other.pData && self.dwDataSize == other.dwDataSize && self.pszContentType == other.pszContentType && self.prgActionIds == other.prgActionIds && self.dwActionsCount == other.dwActionsCount && self.prgActionDisplayNames == other.prgActionDisplayNames
    }
}
impl ::core::cmp::Eq for MAPPING_DATA_RANGE {}
impl ::core::fmt::Debug for MAPPING_DATA_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_DATA_RANGE")
            .field("dwStartIndex", &self.dwStartIndex)
            .field("dwEndIndex", &self.dwEndIndex)
            .field("pszDescription", &self.pszDescription)
            .field("dwDescriptionLength", &self.dwDescriptionLength)
            .field("pData", &self.pData)
            .field("dwDataSize", &self.dwDataSize)
            .field("pszContentType", &self.pszContentType)
            .field("prgActionIds", &self.prgActionIds)
            .field("dwActionsCount", &self.dwActionsCount)
            .field("prgActionDisplayNames", &self.prgActionDisplayNames)
            .finish()
    }
}
impl ::core::default::Default for MAPPING_ENUM_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPPING_ENUM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.pszCategory == other.pszCategory && self.pszInputLanguage == other.pszInputLanguage && self.pszOutputLanguage == other.pszOutputLanguage && self.pszInputScript == other.pszInputScript && self.pszOutputScript == other.pszOutputScript && self.pszInputContentType == other.pszInputContentType && self.pszOutputContentType == other.pszOutputContentType && self.pGuid == other.pGuid && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MAPPING_ENUM_OPTIONS {}
impl ::core::fmt::Debug for MAPPING_ENUM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_ENUM_OPTIONS")
            .field("Size", &self.Size)
            .field("pszCategory", &self.pszCategory)
            .field("pszInputLanguage", &self.pszInputLanguage)
            .field("pszOutputLanguage", &self.pszOutputLanguage)
            .field("pszInputScript", &self.pszInputScript)
            .field("pszOutputScript", &self.pszOutputScript)
            .field("pszInputContentType", &self.pszInputContentType)
            .field("pszOutputContentType", &self.pszOutputContentType)
            .field("pGuid", &self.pGuid)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::default::Default for MAPPING_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MAPPING_PROPERTY_BAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPPING_PROPERTY_BAG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.prgResultRanges == other.prgResultRanges && self.dwRangesCount == other.dwRangesCount && self.pServiceData == other.pServiceData && self.dwServiceDataSize == other.dwServiceDataSize && self.pCallerData == other.pCallerData && self.dwCallerDataSize == other.dwCallerDataSize && self.pContext == other.pContext
    }
}
impl ::core::cmp::Eq for MAPPING_PROPERTY_BAG {}
impl ::core::fmt::Debug for MAPPING_PROPERTY_BAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_PROPERTY_BAG").field("Size", &self.Size).field("prgResultRanges", &self.prgResultRanges).field("dwRangesCount", &self.dwRangesCount).field("pServiceData", &self.pServiceData).field("dwServiceDataSize", &self.dwServiceDataSize).field("pCallerData", &self.pCallerData).field("dwCallerDataSize", &self.dwCallerDataSize).field("pContext", &self.pContext).finish()
    }
}
impl ::core::default::Default for MAPPING_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAPPING_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.pszCopyright == other.pszCopyright
            && self.wMajorVersion == other.wMajorVersion
            && self.wMinorVersion == other.wMinorVersion
            && self.wBuildVersion == other.wBuildVersion
            && self.wStepVersion == other.wStepVersion
            && self.dwInputContentTypesCount == other.dwInputContentTypesCount
            && self.prgInputContentTypes == other.prgInputContentTypes
            && self.dwOutputContentTypesCount == other.dwOutputContentTypesCount
            && self.prgOutputContentTypes == other.prgOutputContentTypes
            && self.dwInputLanguagesCount == other.dwInputLanguagesCount
            && self.prgInputLanguages == other.prgInputLanguages
            && self.dwOutputLanguagesCount == other.dwOutputLanguagesCount
            && self.prgOutputLanguages == other.prgOutputLanguages
            && self.dwInputScriptsCount == other.dwInputScriptsCount
            && self.prgInputScripts == other.prgInputScripts
            && self.dwOutputScriptsCount == other.dwOutputScriptsCount
            && self.prgOutputScripts == other.prgOutputScripts
            && self.guid == other.guid
            && self.pszCategory == other.pszCategory
            && self.pszDescription == other.pszDescription
            && self.dwPrivateDataSize == other.dwPrivateDataSize
            && self.pPrivateData == other.pPrivateData
            && self.pContext == other.pContext
            && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MAPPING_SERVICE_INFO {}
impl ::core::fmt::Debug for MAPPING_SERVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_SERVICE_INFO")
            .field("Size", &self.Size)
            .field("pszCopyright", &self.pszCopyright)
            .field("wMajorVersion", &self.wMajorVersion)
            .field("wMinorVersion", &self.wMinorVersion)
            .field("wBuildVersion", &self.wBuildVersion)
            .field("wStepVersion", &self.wStepVersion)
            .field("dwInputContentTypesCount", &self.dwInputContentTypesCount)
            .field("prgInputContentTypes", &self.prgInputContentTypes)
            .field("dwOutputContentTypesCount", &self.dwOutputContentTypesCount)
            .field("prgOutputContentTypes", &self.prgOutputContentTypes)
            .field("dwInputLanguagesCount", &self.dwInputLanguagesCount)
            .field("prgInputLanguages", &self.prgInputLanguages)
            .field("dwOutputLanguagesCount", &self.dwOutputLanguagesCount)
            .field("prgOutputLanguages", &self.prgOutputLanguages)
            .field("dwInputScriptsCount", &self.dwInputScriptsCount)
            .field("prgInputScripts", &self.prgInputScripts)
            .field("dwOutputScriptsCount", &self.dwOutputScriptsCount)
            .field("prgOutputScripts", &self.prgOutputScripts)
            .field("guid", &self.guid)
            .field("pszCategory", &self.pszCategory)
            .field("pszDescription", &self.pszDescription)
            .field("dwPrivateDataSize", &self.dwPrivateDataSize)
            .field("pPrivateData", &self.pPrivateData)
            .field("pContext", &self.pContext)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::default::Default for MIMECONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIMECONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIMECONTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIMECPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIMECPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.uiCodePage == other.uiCodePage && self.uiFamilyCodePage == other.uiFamilyCodePage && self.wszDescription == other.wszDescription && self.wszWebCharset == other.wszWebCharset && self.wszHeaderCharset == other.wszHeaderCharset && self.wszBodyCharset == other.wszBodyCharset && self.wszFixedWidthFont == other.wszFixedWidthFont && self.wszProportionalFont == other.wszProportionalFont && self.bGDICharset == other.bGDICharset
    }
}
impl ::core::cmp::Eq for MIMECPINFO {}
impl ::core::fmt::Debug for MIMECPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIMECPINFO")
            .field("dwFlags", &self.dwFlags)
            .field("uiCodePage", &self.uiCodePage)
            .field("uiFamilyCodePage", &self.uiFamilyCodePage)
            .field("wszDescription", &self.wszDescription)
            .field("wszWebCharset", &self.wszWebCharset)
            .field("wszHeaderCharset", &self.wszHeaderCharset)
            .field("wszBodyCharset", &self.wszBodyCharset)
            .field("wszFixedWidthFont", &self.wszFixedWidthFont)
            .field("wszProportionalFont", &self.wszProportionalFont)
            .field("bGDICharset", &self.bGDICharset)
            .finish()
    }
}
impl ::core::default::Default for MIMECSETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIMECSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uiCodePage == other.uiCodePage && self.uiInternetEncoding == other.uiInternetEncoding && self.wszCharset == other.wszCharset
    }
}
impl ::core::cmp::Eq for MIMECSETINFO {}
impl ::core::fmt::Debug for MIMECSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIMECSETINFO").field("uiCodePage", &self.uiCodePage).field("uiInternetEncoding", &self.uiInternetEncoding).field("wszCharset", &self.wszCharset).finish()
    }
}
impl ::core::default::Default for MLCONVCHAR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLCONVCHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLCONVCHAR").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLCP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLCP").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLDETECTCP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLDETECTCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLDETECTCP").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLSTR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLSTR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLSTR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTI_BYTE_TO_WIDE_CHAR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NEWTEXTMETRICEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NEWTEXTMETRICEXA {
    fn eq(&self, other: &Self) -> bool {
        self.ntmTm == other.ntmTm && self.ntmFontSig == other.ntmFontSig
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NEWTEXTMETRICEXA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NEWTEXTMETRICEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICEXA").field("ntmTm", &self.ntmTm).field("ntmFontSig", &self.ntmFontSig).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NEWTEXTMETRICEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NEWTEXTMETRICEXW {
    fn eq(&self, other: &Self) -> bool {
        self.ntmTm == other.ntmTm && self.ntmFontSig == other.ntmFontSig
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NEWTEXTMETRICEXW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NEWTEXTMETRICEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICEXW").field("ntmTm", &self.ntmTm).field("ntmFontSig", &self.ntmFontSig).finish()
    }
}
impl ::core::default::Default for NLSVERSIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NLSVERSIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNLSVersionInfoSize == other.dwNLSVersionInfoSize && self.dwNLSVersion == other.dwNLSVersion && self.dwDefinedVersion == other.dwDefinedVersion && self.dwEffectiveId == other.dwEffectiveId && self.guidCustomVersion == other.guidCustomVersion
    }
}
impl ::core::cmp::Eq for NLSVERSIONINFO {}
impl ::core::fmt::Debug for NLSVERSIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLSVERSIONINFO").field("dwNLSVersionInfoSize", &self.dwNLSVersionInfoSize).field("dwNLSVersion", &self.dwNLSVersion).field("dwDefinedVersion", &self.dwDefinedVersion).field("dwEffectiveId", &self.dwEffectiveId).field("guidCustomVersion", &self.guidCustomVersion).finish()
    }
}
impl ::core::default::Default for NLSVERSIONINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NLSVERSIONINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwNLSVersionInfoSize == other.dwNLSVersionInfoSize && self.dwNLSVersion == other.dwNLSVersion && self.dwDefinedVersion == other.dwDefinedVersion && self.dwEffectiveId == other.dwEffectiveId && self.guidCustomVersion == other.guidCustomVersion
    }
}
impl ::core::cmp::Eq for NLSVERSIONINFOEX {}
impl ::core::fmt::Debug for NLSVERSIONINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLSVERSIONINFOEX").field("dwNLSVersionInfoSize", &self.dwNLSVersionInfoSize).field("dwNLSVersion", &self.dwNLSVersion).field("dwDefinedVersion", &self.dwDefinedVersion).field("dwEffectiveId", &self.dwEffectiveId).field("guidCustomVersion", &self.guidCustomVersion).finish()
    }
}
impl ::core::default::Default for NORM_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NORM_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NORM_FORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for NUMBERFMTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NUMBERFMTA {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits && self.LeadingZero == other.LeadingZero && self.Grouping == other.Grouping && self.lpDecimalSep == other.lpDecimalSep && self.lpThousandSep == other.lpThousandSep && self.NegativeOrder == other.NegativeOrder
    }
}
impl ::core::cmp::Eq for NUMBERFMTA {}
impl ::core::fmt::Debug for NUMBERFMTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMBERFMTA").field("NumDigits", &self.NumDigits).field("LeadingZero", &self.LeadingZero).field("Grouping", &self.Grouping).field("lpDecimalSep", &self.lpDecimalSep).field("lpThousandSep", &self.lpThousandSep).field("NegativeOrder", &self.NegativeOrder).finish()
    }
}
impl ::core::default::Default for NUMBERFMTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NUMBERFMTW {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits && self.LeadingZero == other.LeadingZero && self.Grouping == other.Grouping && self.lpDecimalSep == other.lpDecimalSep && self.lpThousandSep == other.lpThousandSep && self.NegativeOrder == other.NegativeOrder
    }
}
impl ::core::cmp::Eq for NUMBERFMTW {}
impl ::core::fmt::Debug for NUMBERFMTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMBERFMTW").field("NumDigits", &self.NumDigits).field("LeadingZero", &self.LeadingZero).field("Grouping", &self.Grouping).field("lpDecimalSep", &self.lpDecimalSep).field("lpThousandSep", &self.lpThousandSep).field("NegativeOrder", &self.NegativeOrder).finish()
    }
}
impl ::core::default::Default for OPENTYPE_FEATURE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPENTYPE_FEATURE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.tagFeature == other.tagFeature && self.lParameter == other.lParameter
    }
}
impl ::core::cmp::Eq for OPENTYPE_FEATURE_RECORD {}
impl ::core::fmt::Debug for OPENTYPE_FEATURE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENTYPE_FEATURE_RECORD").field("tagFeature", &self.tagFeature).field("lParameter", &self.lParameter).finish()
    }
}
impl ::core::default::Default for RFC1766INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFC1766INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lcid == other.lcid && self.wszRfc1766 == other.wszRfc1766 && self.wszLocaleName == other.wszLocaleName
    }
}
impl ::core::cmp::Eq for RFC1766INFO {}
impl ::core::fmt::Debug for RFC1766INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RFC1766INFO").field("lcid", &self.lcid).field("wszRfc1766", &self.wszRfc1766).field("wszLocaleName", &self.wszLocaleName).finish()
    }
}
impl ::core::default::Default for SCRIPTCONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTCONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTCONTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTFONTCONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTFONTCONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTFONTCONTF").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPTFONTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPTFONTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.scripts == other.scripts && self.wszFont == other.wszFont
    }
}
impl ::core::cmp::Eq for SCRIPTFONTINFO {}
impl ::core::fmt::Debug for SCRIPTFONTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPTFONTINFO").field("scripts", &self.scripts).field("wszFont", &self.wszFont).finish()
    }
}
impl ::core::default::Default for SCRIPTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ScriptId == other.ScriptId && self.uiCodePage == other.uiCodePage && self.wszDescription == other.wszDescription && self.wszFixedWidthFont == other.wszFixedWidthFont && self.wszProportionalFont == other.wszProportionalFont
    }
}
impl ::core::cmp::Eq for SCRIPTINFO {}
impl ::core::fmt::Debug for SCRIPTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPTINFO").field("ScriptId", &self.ScriptId).field("uiCodePage", &self.uiCodePage).field("wszDescription", &self.wszDescription).field("wszFixedWidthFont", &self.wszFixedWidthFont).field("wszProportionalFont", &self.wszProportionalFont).finish()
    }
}
impl ::core::default::Default for SCRIPT_ANALYSIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_ANALYSIS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.s == other.s
    }
}
impl ::core::cmp::Eq for SCRIPT_ANALYSIS {}
impl ::core::fmt::Debug for SCRIPT_ANALYSIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_ANALYSIS").field("_bitfield", &self._bitfield).field("s", &self.s).finish()
    }
}
impl ::core::default::Default for SCRIPT_CHARPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_CHARPROP {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_CHARPROP {}
impl ::core::fmt::Debug for SCRIPT_CHARPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_CHARPROP").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCRIPT_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_CONTROL {}
impl ::core::fmt::Debug for SCRIPT_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_CONTROL").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCRIPT_DIGITSUBSTITUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_DIGITSUBSTITUTE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2 && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for SCRIPT_DIGITSUBSTITUTE {}
impl ::core::fmt::Debug for SCRIPT_DIGITSUBSTITUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_DIGITSUBSTITUTE").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for SCRIPT_FONTPROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_FONTPROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.wgBlank == other.wgBlank && self.wgDefault == other.wgDefault && self.wgInvalid == other.wgInvalid && self.wgKashida == other.wgKashida && self.iKashidaWidth == other.iKashidaWidth
    }
}
impl ::core::cmp::Eq for SCRIPT_FONTPROPERTIES {}
impl ::core::fmt::Debug for SCRIPT_FONTPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_FONTPROPERTIES").field("cBytes", &self.cBytes).field("wgBlank", &self.wgBlank).field("wgDefault", &self.wgDefault).field("wgInvalid", &self.wgInvalid).field("wgKashida", &self.wgKashida).field("iKashidaWidth", &self.iKashidaWidth).finish()
    }
}
impl ::core::default::Default for SCRIPT_GLYPHPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_GLYPHPROP {
    fn eq(&self, other: &Self) -> bool {
        self.sva == other.sva && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for SCRIPT_GLYPHPROP {}
impl ::core::fmt::Debug for SCRIPT_GLYPHPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_GLYPHPROP").field("sva", &self.sva).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for SCRIPT_IS_COMPLEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_IS_COMPLEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_IS_COMPLEX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.iCharPos == other.iCharPos && self.a == other.a
    }
}
impl ::core::cmp::Eq for SCRIPT_ITEM {}
impl ::core::fmt::Debug for SCRIPT_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_ITEM").field("iCharPos", &self.iCharPos).field("a", &self.a).finish()
    }
}
impl ::core::default::Default for SCRIPT_JUSTIFY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_JUSTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_JUSTIFY").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCRIPT_LOGATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_LOGATTR {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_LOGATTR {}
impl ::core::fmt::Debug for SCRIPT_LOGATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_LOGATTR").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCRIPT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
impl ::core::cmp::Eq for SCRIPT_PROPERTIES {}
impl ::core::fmt::Debug for SCRIPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_PROPERTIES").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).finish()
    }
}
impl ::core::default::Default for SCRIPT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_STATE {}
impl ::core::fmt::Debug for SCRIPT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_STATE").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCRIPT_TABDEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_TABDEF {
    fn eq(&self, other: &Self) -> bool {
        self.cTabStops == other.cTabStops && self.iScale == other.iScale && self.pTabStops == other.pTabStops && self.iTabOrigin == other.iTabOrigin
    }
}
impl ::core::cmp::Eq for SCRIPT_TABDEF {}
impl ::core::fmt::Debug for SCRIPT_TABDEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_TABDEF").field("cTabStops", &self.cTabStops).field("iScale", &self.iScale).field("pTabStops", &self.pTabStops).field("iTabOrigin", &self.iTabOrigin).finish()
    }
}
impl ::core::default::Default for SCRIPT_VISATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCRIPT_VISATTR {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_VISATTR {}
impl ::core::fmt::Debug for SCRIPT_VISATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_VISATTR").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SYSGEOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSGEOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSGEOCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSGEOTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSGEOTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSGEOTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSNLS_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSNLS_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSNLS_FUNCTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXTRANGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TEXTRANGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.potfRecords == other.potfRecords && self.cotfRecords == other.cotfRecords
    }
}
impl ::core::cmp::Eq for TEXTRANGE_PROPERTIES {}
impl ::core::fmt::Debug for TEXTRANGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTRANGE_PROPERTIES").field("potfRecords", &self.potfRecords).field("cotfRecords", &self.cotfRecords).finish()
    }
}
impl ::core::default::Default for TIME_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TIME_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TIME_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TRANSLATE_CHARSET_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSLATE_CHARSET_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSLATE_CHARSET_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for UAcceptResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UAcceptResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UAcceptResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for UAlphabeticIndexLabelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UAlphabeticIndexLabelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UAlphabeticIndexLabelType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBiDiDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBiDiMirroring {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiMirroring {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiMirroring").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBiDiOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiOrder").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBiDiReorderingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiReorderingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiReorderingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBiDiReorderingOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiReorderingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiReorderingOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBidiPairedBracketType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBidiPairedBracketType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBidiPairedBracketType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBlockCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBlockCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBlockCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UBreakIteratorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBreakIteratorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBreakIteratorType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCPMapRangeOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPMapRangeOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPMapRangeOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCPTrie {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UCPTrieData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UCPTrieType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPTrieType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPTrieType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCPTrieValueWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPTrieValueWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPTrieValueWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarAMPMs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarAMPMs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarAMPMs").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarDateFields {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDateFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDateFields").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarDisplayNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDisplayNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDisplayNameType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarLimitType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarLimitType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarLimitType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarMonths {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarMonths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarMonths").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarWallTimeOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarWallTimeOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarWallTimeOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCalendarWeekdayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarWeekdayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarWeekdayType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCharCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCharDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCharIterator {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UCharIteratorOrigin {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharIteratorOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharIteratorOrigin").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCharNameChoice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharNameChoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharNameChoice").field(&self.0).finish()
    }
}
impl ::core::default::Default for UColAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for UColAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColAttributeValue").field(&self.0).finish()
    }
}
impl ::core::default::Default for UColBoundMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColBoundMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColBoundMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UColReorderCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColReorderCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColReorderCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UColRuleOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColRuleOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColRuleOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCollationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCollationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCollationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for UConverterCallbackReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterCallbackReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterCallbackReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for UConverterFromUnicodeArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UConverterFromUnicodeArgs {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.flush == other.flush && self.converter == other.converter && self.source == other.source && self.sourceLimit == other.sourceLimit && self.target == other.target && self.targetLimit == other.targetLimit && self.offsets == other.offsets
    }
}
impl ::core::cmp::Eq for UConverterFromUnicodeArgs {}
impl ::core::fmt::Debug for UConverterFromUnicodeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UConverterFromUnicodeArgs").field("size", &self.size).field("flush", &self.flush).field("converter", &self.converter).field("source", &self.source).field("sourceLimit", &self.sourceLimit).field("target", &self.target).field("targetLimit", &self.targetLimit).field("offsets", &self.offsets).finish()
    }
}
impl ::core::default::Default for UConverterPlatform {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterPlatform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterPlatform").field(&self.0).finish()
    }
}
impl ::core::default::Default for UConverterToUnicodeArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UConverterToUnicodeArgs {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.flush == other.flush && self.converter == other.converter && self.source == other.source && self.sourceLimit == other.sourceLimit && self.target == other.target && self.targetLimit == other.targetLimit && self.offsets == other.offsets
    }
}
impl ::core::cmp::Eq for UConverterToUnicodeArgs {}
impl ::core::fmt::Debug for UConverterToUnicodeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UConverterToUnicodeArgs").field("size", &self.size).field("flush", &self.flush).field("converter", &self.converter).field("source", &self.source).field("sourceLimit", &self.sourceLimit).field("target", &self.target).field("targetLimit", &self.targetLimit).field("offsets", &self.offsets).finish()
    }
}
impl ::core::default::Default for UConverterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UConverterUnicodeSet {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterUnicodeSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterUnicodeSet").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCurrCurrencyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrCurrencyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrCurrencyType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCurrNameStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrNameStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrNameStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCurrencySpacing {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrencySpacing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrencySpacing").field(&self.0).finish()
    }
}
impl ::core::default::Default for UCurrencyUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrencyUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrencyUsage").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateAbsoluteUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateAbsoluteUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateAbsoluteUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateFormatBooleanAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatBooleanAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatBooleanAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateFormatField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatField").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateFormatSymbolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatSymbolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatSymbolType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateRelativeDateTimeFormatterStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateRelativeDateTimeFormatterStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateRelativeDateTimeFormatterStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateRelativeUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateRelativeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateRelativeUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateTimePGDisplayWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePGDisplayWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePGDisplayWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateTimePatternConflict {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternConflict").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateTimePatternField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternField").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateTimePatternMatchOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternMatchOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternMatchOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDateTimeScale {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimeScale {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimeScale").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDecompositionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDecompositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDecompositionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDialectHandling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDialectHandling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDialectHandling").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDisplayContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDisplayContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDisplayContext").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDisplayContextType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDisplayContextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDisplayContextType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UEastAsianWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UEastAsianWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UEastAsianWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for UErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UErrorCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UFieldCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UFieldCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UFieldCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for UFieldPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UFieldPosition {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.beginIndex == other.beginIndex && self.endIndex == other.endIndex
    }
}
impl ::core::cmp::Eq for UFieldPosition {}
impl ::core::fmt::Debug for UFieldPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UFieldPosition").field("field", &self.field).field("beginIndex", &self.beginIndex).field("endIndex", &self.endIndex).finish()
    }
}
impl ::core::default::Default for UFormattableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UFormattableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UFormattableType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UGender {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UGender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UGender").field(&self.0).finish()
    }
}
impl ::core::default::Default for UGraphemeClusterBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UGraphemeClusterBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UGraphemeClusterBreak").field(&self.0).finish()
    }
}
impl ::core::default::Default for UHangulSyllableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UHangulSyllableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UHangulSyllableType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIDNAInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UIDNAInfo {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.isTransitionalDifferent == other.isTransitionalDifferent && self.reservedB3 == other.reservedB3 && self.errors == other.errors && self.reservedI2 == other.reservedI2 && self.reservedI3 == other.reservedI3
    }
}
impl ::core::cmp::Eq for UIDNAInfo {}
impl ::core::fmt::Debug for UIDNAInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIDNAInfo").field("size", &self.size).field("isTransitionalDifferent", &self.isTransitionalDifferent).field("reservedB3", &self.reservedB3).field("errors", &self.errors).field("reservedI2", &self.reservedI2).field("reservedI3", &self.reservedI3).finish()
    }
}
impl ::core::default::Default for UIndicPositionalCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIndicPositionalCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIndicPositionalCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIndicSyllabicCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIndicSyllabicCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIndicSyllabicCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for UJoiningGroup {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UJoiningGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UJoiningGroup").field(&self.0).finish()
    }
}
impl ::core::default::Default for UJoiningType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UJoiningType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UJoiningType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULayoutType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULayoutType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULayoutType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULineBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULineBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULineBreak").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULineBreakTag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULineBreakTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULineBreakTag").field(&self.0).finish()
    }
}
impl ::core::default::Default for UListFormatterField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterField").field(&self.0).finish()
    }
}
impl ::core::default::Default for UListFormatterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UListFormatterWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULocAvailableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocAvailableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocAvailableType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULocDataLocaleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocDataLocaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocDataLocaleType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULocaleDataDelimiterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocaleDataDelimiterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocaleDataDelimiterType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ULocaleDataExemplarSetType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocaleDataExemplarSetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocaleDataExemplarSetType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UMeasureFormatWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMeasureFormatWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMeasureFormatWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for UMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMeasurementSystem").field(&self.0).finish()
    }
}
impl ::core::default::Default for UMessagePatternApostropheMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternApostropheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternApostropheMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UMessagePatternArgType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternArgType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternArgType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UMessagePatternPartType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternPartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternPartType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNICODERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UNICODERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcFrom == other.wcFrom && self.wcTo == other.wcTo
    }
}
impl ::core::cmp::Eq for UNICODERANGE {}
impl ::core::fmt::Debug for UNICODERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNICODERANGE").field("wcFrom", &self.wcFrom).field("wcTo", &self.wcTo).finish()
    }
}
impl ::core::default::Default for UNormalization2Mode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalization2Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalization2Mode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNormalizationCheckResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalizationCheckResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalizationCheckResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNormalizationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalizationMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberCompactStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberCompactStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberCompactStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberDecimalSeparatorDisplay {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberDecimalSeparatorDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberDecimalSeparatorDisplay").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatAttributeValue").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatFields {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatFields").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatPadPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatPadPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatPadPosition").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatRoundingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatRoundingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatRoundingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatSymbol {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatSymbol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatSymbol").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberFormatTextAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatTextAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatTextAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberGroupingStrategy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberGroupingStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberGroupingStrategy").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberRangeCollapse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeCollapse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeCollapse").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberRangeIdentityFallback {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeIdentityFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityFallback").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberRangeIdentityResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeIdentityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberSignDisplay {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberSignDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberSignDisplay").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumberUnitWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberUnitWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberUnitWidth").field(&self.0).finish()
    }
}
impl ::core::default::Default for UNumericType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumericType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumericType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UParseError {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UParseError {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.offset == other.offset && self.preContext == other.preContext && self.postContext == other.postContext
    }
}
impl ::core::cmp::Eq for UParseError {}
impl ::core::fmt::Debug for UParseError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UParseError").field("line", &self.line).field("offset", &self.offset).field("preContext", &self.preContext).field("postContext", &self.postContext).finish()
    }
}
impl ::core::default::Default for UPluralType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPluralType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPluralType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for UPropertyNameChoice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPropertyNameChoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPropertyNameChoice").field(&self.0).finish()
    }
}
impl ::core::default::Default for URegexpFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URegexpFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URegexpFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for URegionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URegionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URegionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for URelativeDateTimeFormatterField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URelativeDateTimeFormatterField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URelativeDateTimeFormatterField").field(&self.0).finish()
    }
}
impl ::core::default::Default for URelativeDateTimeUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URelativeDateTimeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URelativeDateTimeUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for UReplaceableCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UReplaceableCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.charAt == other.charAt && self.char32At == other.char32At && self.replace == other.replace && self.extract == other.extract && self.copy == other.copy
    }
}
impl ::core::cmp::Eq for UReplaceableCallbacks {}
impl ::core::fmt::Debug for UReplaceableCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UReplaceableCallbacks").field("length", &self.length).field("charAt", &self.charAt).field("char32At", &self.char32At).field("replace", &self.replace).field("extract", &self.extract).field("copy", &self.copy).finish()
    }
}
impl ::core::default::Default for UResType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UResType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UResType").field(&self.0).finish()
    }
}
impl ::core::default::Default for URestrictionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URestrictionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URestrictionLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for UScriptCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UScriptCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UScriptCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for UScriptUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UScriptUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UScriptUsage").field(&self.0).finish()
    }
}
impl ::core::default::Default for USearchAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USearchAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USearchAttribute").field(&self.0).finish()
    }
}
impl ::core::default::Default for USearchAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USearchAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USearchAttributeValue").field(&self.0).finish()
    }
}
impl ::core::default::Default for USentenceBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USentenceBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USentenceBreak").field(&self.0).finish()
    }
}
impl ::core::default::Default for USentenceBreakTag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USentenceBreakTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USentenceBreakTag").field(&self.0).finish()
    }
}
impl ::core::default::Default for USerializedSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USerializedSet {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.bmpLength == other.bmpLength && self.length == other.length && self.staticArray == other.staticArray
    }
}
impl ::core::cmp::Eq for USerializedSet {}
impl ::core::fmt::Debug for USerializedSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USerializedSet").field("array", &self.array).field("bmpLength", &self.bmpLength).field("length", &self.length).field("staticArray", &self.staticArray).finish()
    }
}
impl ::core::default::Default for USetSpanCondition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USetSpanCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USetSpanCondition").field(&self.0).finish()
    }
}
impl ::core::default::Default for USpoofChecks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USpoofChecks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USpoofChecks").field(&self.0).finish()
    }
}
impl ::core::default::Default for UStringPrepProfileType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringPrepProfileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringPrepProfileType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UStringTrieBuildOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringTrieBuildOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringTrieBuildOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UStringTrieResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringTrieResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringTrieResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for USystemTimeZoneType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USystemTimeZoneType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USystemTimeZoneType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UText {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UText {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic
            && self.flags == other.flags
            && self.providerProperties == other.providerProperties
            && self.sizeOfStruct == other.sizeOfStruct
            && self.chunkNativeLimit == other.chunkNativeLimit
            && self.extraSize == other.extraSize
            && self.nativeIndexingLimit == other.nativeIndexingLimit
            && self.chunkNativeStart == other.chunkNativeStart
            && self.chunkOffset == other.chunkOffset
            && self.chunkLength == other.chunkLength
            && self.chunkContents == other.chunkContents
            && self.pFuncs == other.pFuncs
            && self.pExtra == other.pExtra
            && self.context == other.context
            && self.p == other.p
            && self.q == other.q
            && self.r == other.r
            && self.privP == other.privP
            && self.a == other.a
            && self.b == other.b
            && self.c == other.c
            && self.privA == other.privA
            && self.privB == other.privB
            && self.privC == other.privC
    }
}
impl ::core::cmp::Eq for UText {}
impl ::core::fmt::Debug for UText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UText")
            .field("magic", &self.magic)
            .field("flags", &self.flags)
            .field("providerProperties", &self.providerProperties)
            .field("sizeOfStruct", &self.sizeOfStruct)
            .field("chunkNativeLimit", &self.chunkNativeLimit)
            .field("extraSize", &self.extraSize)
            .field("nativeIndexingLimit", &self.nativeIndexingLimit)
            .field("chunkNativeStart", &self.chunkNativeStart)
            .field("chunkOffset", &self.chunkOffset)
            .field("chunkLength", &self.chunkLength)
            .field("chunkContents", &self.chunkContents)
            .field("pFuncs", &self.pFuncs)
            .field("pExtra", &self.pExtra)
            .field("context", &self.context)
            .field("p", &self.p)
            .field("q", &self.q)
            .field("r", &self.r)
            .field("privP", &self.privP)
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("privA", &self.privA)
            .field("privB", &self.privB)
            .field("privC", &self.privC)
            .finish()
    }
}
impl ::core::default::Default for UTextFuncs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UTimeScaleValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeScaleValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeScaleValue").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneFormatGMTOffsetPatternType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatGMTOffsetPatternType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatGMTOffsetPatternType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneFormatParseOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatParseOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatParseOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneFormatTimeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatTimeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatTimeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneNameType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTimeZoneTransitionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneTransitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneTransitionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTraceFunctionNumber {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTraceFunctionNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTraceFunctionNumber").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTraceLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTraceLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTransDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTransDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTransDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for UTransPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UTransPosition {
    fn eq(&self, other: &Self) -> bool {
        self.contextStart == other.contextStart && self.contextLimit == other.contextLimit && self.start == other.start && self.limit == other.limit
    }
}
impl ::core::cmp::Eq for UTransPosition {}
impl ::core::fmt::Debug for UTransPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UTransPosition").field("contextStart", &self.contextStart).field("contextLimit", &self.contextLimit).field("start", &self.start).field("limit", &self.limit).finish()
    }
}
impl ::core::default::Default for UVerticalOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UVerticalOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UVerticalOrientation").field(&self.0).finish()
    }
}
impl ::core::default::Default for UWordBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UWordBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UWordBreak").field(&self.0).finish()
    }
}
impl ::core::default::Default for UWordBreakValues {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UWordBreakValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UWordBreakValues").field(&self.0).finish()
    }
}
impl ::core::default::Default for WORDLIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORDLIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORDLIST_TYPE").field(&self.0).finish()
    }
}
