impl ::core::default::Default for AEROWIZARDPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AEROWIZARDPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AEROWIZARDPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ARROWBTNSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARROWBTNSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARROWBTNSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BACKGROUNDWITHBORDERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BACKGROUNDWITHBORDERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKGROUNDWITHBORDERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BALLOONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BALLOONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BALLOONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BALLOONSTEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BALLOONSTEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BALLOONSTEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BARBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BARBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BARBACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BARITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BARITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BARITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BODYSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BODYSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BODYSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDER_HSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDER_HSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDER_HSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDER_HVSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDER_HVSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDER_HVSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDER_NOSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDER_NOSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDER_NOSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BORDER_VSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BORDER_VSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDER_VSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BP_ANIMATIONPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BP_ANIMATIONPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.style == other.style && self.dwDuration == other.dwDuration
    }
}
impl ::core::cmp::Eq for BP_ANIMATIONPARAMS {}
impl ::core::fmt::Debug for BP_ANIMATIONPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BP_ANIMATIONPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("style", &self.style).field("dwDuration", &self.dwDuration).finish()
    }
}
impl ::core::default::Default for BP_ANIMATIONSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BP_ANIMATIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_ANIMATIONSTYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BP_BUFFERFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BP_BUFFERFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_BUFFERFORMAT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for BP_PAINTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for BP_PAINTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.prcExclude == other.prcExclude && self.pBlendFunction == other.pBlendFunction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for BP_PAINTPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for BP_PAINTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BP_PAINTPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("prcExclude", &self.prcExclude).field("pBlendFunction", &self.pBlendFunction).finish()
    }
}
impl ::core::default::Default for BP_PAINTPARAMS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BP_PAINTPARAMS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_PAINTPARAMS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BP_PAINTPARAMS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BP_PAINTPARAMS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for BUTTONPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BUTTONPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUTTONPARTS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BUTTON_IMAGELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BUTTON_IMAGELIST {
    fn eq(&self, other: &Self) -> bool {
        self.himl == other.himl && self.margin == other.margin && self.uAlign == other.uAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BUTTON_IMAGELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BUTTON_IMAGELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUTTON_IMAGELIST").field("himl", &self.himl).field("margin", &self.margin).field("uAlign", &self.uAlign).finish()
    }
}
impl ::core::default::Default for BUTTON_IMAGELIST_ALIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BUTTON_IMAGELIST_ALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUTTON_IMAGELIST_ALIGN").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BUTTON_SPLITINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BUTTON_SPLITINFO {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.himlGlyph == other.himlGlyph && self.uSplitStyle == other.uSplitStyle && self.size == other.size
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BUTTON_SPLITINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BUTTON_SPLITINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUTTON_SPLITINFO").field("mask", &self.mask).field("himlGlyph", &self.himlGlyph).field("uSplitStyle", &self.uSplitStyle).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for CAPTIONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CAPTIONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CAPTIONSTATES").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CCSTYLEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CCSTYLEA {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flExtStyle == other.flExtStyle && self.szText == other.szText && self.lgid == other.lgid && self.wReserved1 == other.wReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CCSTYLEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CCSTYLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEA").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
impl ::core::default::Default for CCSTYLEFLAGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CCSTYLEFLAGA {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flStyleMask == other.flStyleMask && self.pszStyle == other.pszStyle
    }
}
impl ::core::cmp::Eq for CCSTYLEFLAGA {}
impl ::core::fmt::Debug for CCSTYLEFLAGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEFLAGA").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
impl ::core::default::Default for CCSTYLEFLAGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CCSTYLEFLAGW {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flStyleMask == other.flStyleMask && self.pszStyle == other.pszStyle
    }
}
impl ::core::cmp::Eq for CCSTYLEFLAGW {}
impl ::core::fmt::Debug for CCSTYLEFLAGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEFLAGW").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
impl ::core::default::Default for CCSTYLEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CCSTYLEW {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flExtStyle == other.flExtStyle && self.szText == other.szText && self.lgid == other.lgid && self.wReserved1 == other.wReserved1
    }
}
impl ::core::cmp::Eq for CCSTYLEW {}
impl ::core::fmt::Debug for CCSTYLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEW").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
impl ::core::default::Default for CHECKBOXSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHECKBOXSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHECKBOXSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHEVRONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHEVRONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHEVRONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHEVRONVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHEVRONVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHEVRONVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLOCKPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLOCKPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOCKPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLOCKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLOCKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOCKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLOSEBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLOSEBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOSEBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLOSESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLOSESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOSESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLLAPSEBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLLAPSEBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLLAPSEBUTTONSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COLORMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COLORMAP {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COLORMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COLORMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMAP").field("from", &self.from).field("to", &self.to).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COLORSCHEME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COLORSCHEME {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.clrBtnHighlight == other.clrBtnHighlight && self.clrBtnShadow == other.clrBtnShadow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COLORSCHEME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COLORSCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORSCHEME").field("dwSize", &self.dwSize).field("clrBtnHighlight", &self.clrBtnHighlight).field("clrBtnShadow", &self.clrBtnShadow).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMBOBOXEXITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMBOBOXEXITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.iOverlay == other.iOverlay && self.iIndent == other.iIndent && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMBOBOXEXITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMBOBOXEXITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXEXITEMA").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("iOverlay", &self.iOverlay).field("iIndent", &self.iIndent).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMBOBOXEXITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMBOBOXEXITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.iOverlay == other.iOverlay && self.iIndent == other.iIndent && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMBOBOXEXITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMBOBOXEXITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXEXITEMW").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("iOverlay", &self.iOverlay).field("iIndent", &self.iIndent).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMBOBOXINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMBOBOXINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcItem == other.rcItem && self.rcButton == other.rcButton && self.stateButton == other.stateButton && self.hwndCombo == other.hwndCombo && self.hwndItem == other.hwndItem && self.hwndList == other.hwndList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMBOBOXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMBOBOXINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXINFO").field("cbSize", &self.cbSize).field("rcItem", &self.rcItem).field("rcButton", &self.rcButton).field("stateButton", &self.stateButton).field("hwndCombo", &self.hwndCombo).field("hwndItem", &self.hwndItem).field("hwndList", &self.hwndList).finish()
    }
}
impl ::core::default::Default for COMBOBOXINFO_BUTTON_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMBOBOXINFO_BUTTON_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOXINFO_BUTTON_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMBOBOXPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMBOBOXPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOXPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMBOBOXSTYLESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMBOBOXSTYLESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOXSTYLESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMBOBOX_EX_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMBOBOX_EX_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOX_EX_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for COMMANDLINKGLYPHSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMMANDLINKGLYPHSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMANDLINKGLYPHSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMMANDLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMMANDLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMANDLINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMMUNICATIONSPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMMUNICATIONSPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMUNICATIONSPARTS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPAREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPAREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.hwndItem == other.hwndItem && self.itemID1 == other.itemID1 && self.itemData1 == other.itemData1 && self.itemID2 == other.itemID2 && self.itemData2 == other.itemData2 && self.dwLocaleId == other.dwLocaleId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPAREITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPAREITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPAREITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("hwndItem", &self.hwndItem).field("itemID1", &self.itemID1).field("itemData1", &self.itemData1).field("itemID2", &self.itemID2).field("itemData2", &self.itemData2).field("dwLocaleId", &self.dwLocaleId).finish()
    }
}
impl ::core::default::Default for CONTENTALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTENTALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTENTALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTENTAREASTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTENTAREASTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTENTAREASTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTENTLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTENTLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTENTLINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTENTPANESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTENTPANESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTENTPANESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTROLLABELSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTROLLABELSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTROLLABELSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTROLPANELPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTROLPANELPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTROLPANELPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for COPYSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREATELINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATELINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATELINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for CUEBANNERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CUEBANNERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CUEBANNERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATEBORDERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DATEBORDERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATEBORDERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATEPICKERPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DATEPICKERPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATEPICKERPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATETEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DATETEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATETEXTSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DATETIMEPICKERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DATETIMEPICKERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcCheck == other.rcCheck && self.stateCheck == other.stateCheck && self.rcButton == other.rcButton && self.stateButton == other.stateButton && self.hwndEdit == other.hwndEdit && self.hwndUD == other.hwndUD && self.hwndDropDown == other.hwndDropDown
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DATETIMEPICKERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DATETIMEPICKERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIMEPICKERINFO").field("cbSize", &self.cbSize).field("rcCheck", &self.rcCheck).field("stateCheck", &self.stateCheck).field("rcButton", &self.rcButton).field("stateButton", &self.stateButton).field("hwndEdit", &self.hwndEdit).field("hwndUD", &self.hwndUD).field("hwndDropDown", &self.hwndDropDown).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELETEITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DELETEITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.hwndItem == other.hwndItem && self.itemData == other.itemData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DELETEITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DELETEITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELETEITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("hwndItem", &self.hwndItem).field("itemData", &self.itemData).finish()
    }
}
impl ::core::default::Default for DLG_BUTTON_CHECK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DLG_BUTTON_CHECK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DLG_BUTTON_CHECK_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DLG_DIR_LIST_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DLG_DIR_LIST_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DLG_DIR_LIST_FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DOWNHORZSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOWNHORZSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOWNHORZSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOWNSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOWNSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOWNSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DPAMM_MESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DPAMM_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPAMM_MESSAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DPASTREAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DPASTREAMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.iPos == other.iPos && self.pvItem == other.pvItem
    }
}
impl ::core::cmp::Eq for DPASTREAMINFO {}
impl ::core::fmt::Debug for DPASTREAMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DPASTREAMINFO").field("iPos", &self.iPos).field("pvItem", &self.pvItem).finish()
    }
}
impl ::core::default::Default for DRAGDROPPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAGDROPPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAGDROPPARTS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAGLISTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRAGLISTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uNotification == other.uNotification && self.hWnd == other.hWnd && self.ptCursor == other.ptCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRAGLISTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRAGLISTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAGLISTINFO").field("uNotification", &self.uNotification).field("hWnd", &self.hWnd).field("ptCursor", &self.ptCursor).finish()
    }
}
impl ::core::default::Default for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAGLISTINFO_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DRAWITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DRAWITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.itemAction == other.itemAction && self.itemState == other.itemState && self.hwndItem == other.hwndItem && self.hDC == other.hDC && self.rcItem == other.rcItem && self.itemData == other.itemData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DRAWITEMSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DRAWITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("itemAction", &self.itemAction).field("itemState", &self.itemState).field("hwndItem", &self.hwndItem).field("hDC", &self.hDC).field("rcItem", &self.rcItem).field("itemData", &self.itemData).finish()
    }
}
impl ::core::default::Default for DRAWITEMSTRUCT_CTL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAWITEMSTRUCT_CTL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWITEMSTRUCT_CTL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_THEME_PARENT_BACKGROUND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DROPDOWNBUTTONLEFTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DROPDOWNBUTTONLEFTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPDOWNBUTTONLEFTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DROPDOWNBUTTONRIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DROPDOWNBUTTONRIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPDOWNBUTTONRIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DROPDOWNITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DROPDOWNITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPDOWNITEMSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DTBGOPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DTBGOPTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.rcClip == other.rcClip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DTBGOPTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DTBGOPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBGOPTS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("rcClip", &self.rcClip).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DTTOPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DTTOPTS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTTOPTS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTTOPTS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DTTOPTS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DTTOPTS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DTTOPTS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DTTOPTS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DTTOPTS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for EC_ENDOFLINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EC_ENDOFLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_ENDOFLINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EC_SEARCHWEB_ENTRYPOINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EC_SEARCHWEB_ENTRYPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SEARCHWEB_ENTRYPOINT").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITBALLOONTIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EDITBALLOONTIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszTitle == other.pszTitle && self.pszText == other.pszText && self.ttiIcon == other.ttiIcon
    }
}
impl ::core::cmp::Eq for EDITBALLOONTIP {}
impl ::core::fmt::Debug for EDITBALLOONTIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EDITBALLOONTIP").field("cbStruct", &self.cbStruct).field("pszTitle", &self.pszTitle).field("pszText", &self.pszText).field("ttiIcon", &self.ttiIcon).finish()
    }
}
impl ::core::default::Default for EDITBALLOONTIP_ICON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITBALLOONTIP_ICON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBALLOONTIP_ICON").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITBORDER_HSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITBORDER_HSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBORDER_HSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITBORDER_HVSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITBORDER_HVSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBORDER_HVSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITBORDER_NOSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITBORDER_NOSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBORDER_NOSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITBORDER_VSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITBORDER_VSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBORDER_VSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EDITTEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDITTEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITTEXTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EMPTYMARKUPPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMPTYMARKUPPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMPTYMARKUPPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENABLE_SCROLL_BAR_ARROWS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENABLE_SCROLL_BAR_ARROWS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENABLE_SCROLL_BAR_ARROWS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXPANDBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPANDBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPANDBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXPANDOBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPANDOBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPANDOBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for EXPLORERBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPLORERBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPLORERBARPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FEEDBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEEDBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILLTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILLVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILLVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILLVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for FLYOUTPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLYOUTPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLYOUTPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMEBOTTOMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMEBOTTOMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMEBOTTOMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMELEFTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMELEFTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMELEFTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMERIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMERIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMERIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_THEME_BITMAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_THEME_BITMAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_THEME_BITMAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GLYPHFONTSIZINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLYPHFONTSIZINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLYPHFONTSIZINGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GLYPHSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLYPHSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLYPHSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GLYPHTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLYPHTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLYPHTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GRIDCELLBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRIDCELLBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLBACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GRIDCELLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRIDCELLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GRIDCELLUPPERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRIDCELLUPPERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLUPPERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GRIPPERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRIPPERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIPPERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GROUPBOXSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GROUPBOXSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUPBOXSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GROUPHEADERLINESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GROUPHEADERLINESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUPHEADERLINESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for GROUPHEADERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GROUPHEADERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUPHEADERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HALIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HALIGN").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HDHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HDHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iItem == other.iItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HDHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HDHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for HDITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for HDITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.cxy == other.cxy && self.pszText == other.pszText && self.hbm == other.hbm && self.cchTextMax == other.cchTextMax && self.fmt == other.fmt && self.lParam == other.lParam && self.iImage == other.iImage && self.iOrder == other.iOrder && self.r#type == other.r#type && self.pvFilter == other.pvFilter && self.state == other.state
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for HDITEMA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for HDITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDITEMA").field("mask", &self.mask).field("cxy", &self.cxy).field("pszText", &self.pszText).field("hbm", &self.hbm).field("cchTextMax", &self.cchTextMax).field("fmt", &self.fmt).field("lParam", &self.lParam).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("type", &self.r#type).field("pvFilter", &self.pvFilter).field("state", &self.state).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for HDITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for HDITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.cxy == other.cxy && self.pszText == other.pszText && self.hbm == other.hbm && self.cchTextMax == other.cchTextMax && self.fmt == other.fmt && self.lParam == other.lParam && self.iImage == other.iImage && self.iOrder == other.iOrder && self.r#type == other.r#type && self.pvFilter == other.pvFilter && self.state == other.state
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for HDITEMW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for HDITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDITEMW").field("mask", &self.mask).field("cxy", &self.cxy).field("pszText", &self.pszText).field("hbm", &self.hbm).field("cchTextMax", &self.cchTextMax).field("fmt", &self.fmt).field("lParam", &self.lParam).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("type", &self.r#type).field("pvFilter", &self.pvFilter).field("state", &self.state).finish()
    }
}
impl ::core::default::Default for HDI_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDI_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDI_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HDI_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HDI_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HDI_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HDI_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HDI_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for HDLAYOUT {
    fn eq(&self, other: &Self) -> bool {
        self.prc == other.prc && self.pwpos == other.pwpos
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for HDLAYOUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for HDLAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDLAYOUT").field("prc", &self.prc).field("pwpos", &self.pwpos).finish()
    }
}
impl ::core::default::Default for HD_TEXTFILTERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HD_TEXTFILTERA {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText && self.cchTextMax == other.cchTextMax
    }
}
impl ::core::cmp::Eq for HD_TEXTFILTERA {}
impl ::core::fmt::Debug for HD_TEXTFILTERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HD_TEXTFILTERA").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
impl ::core::default::Default for HD_TEXTFILTERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HD_TEXTFILTERW {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText && self.cchTextMax == other.cchTextMax
    }
}
impl ::core::cmp::Eq for HD_TEXTFILTERW {}
impl ::core::fmt::Debug for HD_TEXTFILTERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HD_TEXTFILTERW").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
impl ::core::default::Default for HEADERAREASTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERAREASTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERAREASTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERCLOSESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERCLOSESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERCLOSESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERDROPDOWNFILTERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERDROPDOWNFILTERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERDROPDOWNFILTERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERDROPDOWNSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERDROPDOWNSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERDROPDOWNSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERITEMLEFTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERITEMLEFTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERITEMLEFTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERITEMRIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERITEMRIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERITEMRIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADEROVERFLOWSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADEROVERFLOWSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADEROVERFLOWSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERPINSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERPINSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERPINSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERSORTARROWSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERSORTARROWSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERSORTARROWSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADERSTYLESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADERSTYLESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADERSTYLESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADER_CONTROL_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADER_CONTROL_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_CONTROL_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADER_CONTROL_FORMAT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADER_CONTROL_FORMAT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_CONTROL_FORMAT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADER_CONTROL_FORMAT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADER_CONTROL_FORMAT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_CONTROL_FORMAT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_CONTROL_NOTIFICATION_BUTTON").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEADER_HITTEST_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEADER_HITTEST_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_HITTEST_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HEADER_HITTEST_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HEADER_HITTEST_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HEADER_HITTEST_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HELPBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HELPBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HELPBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HELPLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HELPLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HELPLINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIT_TEST_BACKGROUND_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HIT_TEST_BACKGROUND_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIT_TEST_BACKGROUND_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HORZSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HORZSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HORZSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HORZTHUMBSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HORZTHUMBSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HORZTHUMBSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HOTGLYPHSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOTGLYPHSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOTGLYPHSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HOVERBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOVERBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOVERBACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HYPERLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HYPERLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HYPERLINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for HYPERLINKTEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HYPERLINKTEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HYPERLINKTEXTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICONEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICONEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICONEFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for IEBARMENUSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IEBARMENUSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEBARMENUSTATES").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IImageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList {}
impl ::core::fmt::Debug for IImageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IImageList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList2 {}
impl ::core::fmt::Debug for IImageList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList2").field(&self.0).finish()
    }
}
impl IImageList2 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Add<P0, P1>(&self, hbmimage: P0, hbmmask: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Add)(::windows::core::Vtable::as_raw(self), hbmimage.into(), hbmmask.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ReplaceIcon<P0>(&self, i: i32, hicon: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReplaceIcon)(::windows::core::Vtable::as_raw(self), i, hicon.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOverlayImage)(::windows::core::Vtable::as_raw(self), iimage, ioverlay).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace<P0, P1>(&self, i: i32, hbmimage: P0, hbmmask: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Vtable::vtable(self).base__.Replace)(::windows::core::Vtable::as_raw(self), i, hbmimage.into(), hbmmask.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AddMasked<P0, P1>(&self, hbmimage: P0, crmask: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddMasked)(::windows::core::Vtable::as_raw(self), hbmimage.into(), crmask.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), pimldp).ok()
    }
    pub unsafe fn Remove(&self, i: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Remove)(::windows::core::Vtable::as_raw(self), i).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows::core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIcon)(::windows::core::Vtable::as_raw(self), i, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImageInfo(&self, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetImageInfo)(::windows::core::Vtable::as_raw(self), i, pimageinfo).ok()
    }
    pub unsafe fn Copy<P0>(&self, idst: i32, punksrc: P0, isrc: i32, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Copy)(::windows::core::Vtable::as_raw(self), idst, punksrc.into().abi(), isrc, uflags).ok()
    }
    pub unsafe fn Merge<P0>(&self, i1: i32, punk2: P0, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Merge)(::windows::core::Vtable::as_raw(self), i1, punk2.into().abi(), i2, dx, dy, riid, ppv).ok()
    }
    pub unsafe fn Clone(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImageRect)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIconSize)(::windows::core::Vtable::as_raw(self), cx, cy).ok()
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIconSize)(::windows::core::Vtable::as_raw(self), cx, cy).ok()
    }
    pub unsafe fn GetImageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImageCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetImageCount)(::windows::core::Vtable::as_raw(self), unewcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBkColor<P0>(&self, clrbk: P0) -> ::windows::core::Result<super::super::Foundation::COLORREF>
    where
        P0: ::std::convert::Into<super::super::Foundation::COLORREF>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SetBkColor)(::windows::core::Vtable::as_raw(self), clrbk.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBkColor(&self) -> ::windows::core::Result<super::super::Foundation::COLORREF> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBkColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginDrag)(::windows::core::Vtable::as_raw(self), itrack, dxhotspot, dyhotspot).ok()
    }
    pub unsafe fn EndDrag(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDrag)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragEnter<P0>(&self, hwndlock: P0, x: i32, y: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DragEnter)(::windows::core::Vtable::as_raw(self), hwndlock.into(), x, y).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragLeave<P0>(&self, hwndlock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DragLeave)(::windows::core::Vtable::as_raw(self), hwndlock.into()).ok()
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DragMove)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn SetDragCursorImage<P0>(&self, punk: P0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDragCursorImage)(::windows::core::Vtable::as_raw(self), punk.into().abi(), idrag, dxhotspot, dyhotspot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragShowNolock<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.DragShowNolock)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDragImage(&self, ppt: ::core::option::Option<*mut super::super::Foundation::POINT>, ppthotspot: ::core::option::Option<*mut super::super::Foundation::POINT>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDragImage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppt.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppthotspot.unwrap_or(::std::ptr::null_mut())), riid, ppv).ok()
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows::core::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemFlags)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOverlayImage)(::windows::core::Vtable::as_raw(self), ioverlay, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IMAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IMAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hbmImage == other.hbmImage && self.hbmMask == other.hbmMask && self.Unused1 == other.Unused1 && self.Unused2 == other.Unused2 && self.rcImage == other.rcImage
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IMAGEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IMAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEINFO").field("hbmImage", &self.hbmImage).field("hbmMask", &self.hbmMask).field("Unused1", &self.Unused1).field("Unused2", &self.Unused2).field("rcImage", &self.rcImage).finish()
    }
}
impl ::core::default::Default for IMAGELAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGELAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGELAYOUT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IMAGELISTDRAWPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IMAGELISTDRAWPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.himl == other.himl && self.i == other.i && self.hdcDst == other.hdcDst && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.xBitmap == other.xBitmap && self.yBitmap == other.yBitmap && self.rgbBk == other.rgbBk && self.rgbFg == other.rgbFg && self.fStyle == other.fStyle && self.dwRop == other.dwRop && self.fState == other.fState && self.Frame == other.Frame && self.crEffect == other.crEffect
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IMAGELISTDRAWPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IMAGELISTDRAWPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGELISTDRAWPARAMS")
            .field("cbSize", &self.cbSize)
            .field("himl", &self.himl)
            .field("i", &self.i)
            .field("hdcDst", &self.hdcDst)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("xBitmap", &self.xBitmap)
            .field("yBitmap", &self.yBitmap)
            .field("rgbBk", &self.rgbBk)
            .field("rgbFg", &self.rgbFg)
            .field("fStyle", &self.fStyle)
            .field("dwRop", &self.dwRop)
            .field("fState", &self.fState)
            .field("Frame", &self.Frame)
            .field("crEffect", &self.crEffect)
            .finish()
    }
}
impl ::core::default::Default for IMAGELISTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IMAGELISTSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cAlloc == other.cAlloc && self.cUsed == other.cUsed && self.cStandby == other.cStandby
    }
}
impl ::core::cmp::Eq for IMAGELISTSTATS {}
impl ::core::fmt::Debug for IMAGELISTSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGELISTSTATS").field("cbSize", &self.cbSize).field("cAlloc", &self.cAlloc).field("cUsed", &self.cUsed).field("cStandby", &self.cStandby).finish()
    }
}
impl ::core::default::Default for IMAGELIST_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGELIST_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGELIST_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGELIST_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGELIST_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IMAGESELECTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGESELECTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGESELECTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_LIST_COPY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_LIST_COPY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_COPY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_LIST_DRAW_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_LIST_DRAW_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_DRAW_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IMAGE_LIST_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_LIST_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_WRITE_STREAM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for INITCOMMONCONTROLSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INITCOMMONCONTROLSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwICC == other.dwICC
    }
}
impl ::core::cmp::Eq for INITCOMMONCONTROLSEX {}
impl ::core::fmt::Debug for INITCOMMONCONTROLSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INITCOMMONCONTROLSEX").field("dwSize", &self.dwSize).field("dwICC", &self.dwICC).finish()
    }
}
impl ::core::default::Default for INITCOMMONCONTROLSEX_ICC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INITCOMMONCONTROLSEX_ICC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INITCOMMONCONTROLSEX_ICC").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for INTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.iValueCount == other.iValueCount && self.iValues == other.iValues
    }
}
impl ::core::cmp::Eq for INTLIST {}
impl ::core::fmt::Debug for INTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTLIST").field("iValueCount", &self.iValueCount).field("iValues", &self.iValues).finish()
    }
}
impl ::core::default::Default for ITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for LABELSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LABELSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LABELSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LHITTESTINFO").field("pt", &self.pt).field("item", &self.item).finish()
    }
}
impl ::core::default::Default for LINKHEADERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LINKHEADERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINKHEADERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for LINKPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LINKPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINKPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for LISTBOXPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LISTBOXPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LISTBOXPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LISTITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LISTITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LISTITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for LISTVIEWPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LISTVIEWPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LISTVIEWPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIST_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_ITEM_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_ITEM_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_ITEM_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_ITEM_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_ITEM_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_ITEM_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_BACKGROUND_IMAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_GROUP_ALIGN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_GROUP_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_GROUP_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_GROUP_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_GROUP_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_GROUP_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_GROUP_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_INSERT_MARK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_INSERT_MARK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_INSERT_MARK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_INSERT_MARK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_INSERT_MARK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_INSERT_MARK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_INSERT_MARK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_INSERT_MARK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LIST_VIEW_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LIST_VIEW_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LIST_VIEW_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LIST_VIEW_ITEM_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LIST_VIEW_ITEM_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIST_VIEW_ITEM_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iLink == other.iLink && self.state == other.state && self.stateMask == other.stateMask && self.szID == other.szID && self.szUrl == other.szUrl
    }
}
impl ::core::cmp::Eq for LITEM {}
impl ::core::fmt::Debug for LITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LITEM").field("mask", &self.mask).field("iLink", &self.iLink).field("state", &self.state).field("stateMask", &self.stateMask).field("szID", &self.szID).field("szUrl", &self.szUrl).finish()
    }
}
impl ::core::default::Default for LOGOFFBUTTONSSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOGOFFBUTTONSSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGOFFBUTTONSSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LVBKIMAGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LVBKIMAGEA {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.hbm == other.hbm && self.pszImage == other.pszImage && self.cchImageMax == other.cchImageMax && self.xOffsetPercent == other.xOffsetPercent && self.yOffsetPercent == other.yOffsetPercent
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LVBKIMAGEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LVBKIMAGEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVBKIMAGEA").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LVBKIMAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LVBKIMAGEW {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.hbm == other.hbm && self.pszImage == other.pszImage && self.cchImageMax == other.cchImageMax && self.xOffsetPercent == other.xOffsetPercent && self.yOffsetPercent == other.yOffsetPercent
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LVBKIMAGEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LVBKIMAGEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVBKIMAGEW").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
impl ::core::default::Default for LVCOLUMNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVCOLUMNA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.fmt == other.fmt && self.cx == other.cx && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iSubItem == other.iSubItem && self.iImage == other.iImage && self.iOrder == other.iOrder && self.cxMin == other.cxMin && self.cxDefault == other.cxDefault && self.cxIdeal == other.cxIdeal
    }
}
impl ::core::cmp::Eq for LVCOLUMNA {}
impl ::core::fmt::Debug for LVCOLUMNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVCOLUMNA").field("mask", &self.mask).field("fmt", &self.fmt).field("cx", &self.cx).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iSubItem", &self.iSubItem).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("cxMin", &self.cxMin).field("cxDefault", &self.cxDefault).field("cxIdeal", &self.cxIdeal).finish()
    }
}
impl ::core::default::Default for LVCOLUMNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVCOLUMNW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.fmt == other.fmt && self.cx == other.cx && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iSubItem == other.iSubItem && self.iImage == other.iImage && self.iOrder == other.iOrder && self.cxMin == other.cxMin && self.cxDefault == other.cxDefault && self.cxIdeal == other.cxIdeal
    }
}
impl ::core::cmp::Eq for LVCOLUMNW {}
impl ::core::fmt::Debug for LVCOLUMNW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVCOLUMNW").field("mask", &self.mask).field("fmt", &self.fmt).field("cx", &self.cx).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iSubItem", &self.iSubItem).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("cxMin", &self.cxMin).field("cxDefault", &self.cxDefault).field("cxIdeal", &self.cxIdeal).finish()
    }
}
impl ::core::default::Default for LVCOLUMNW_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVCOLUMNW_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVCOLUMNW_FORMAT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVCOLUMNW_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVCOLUMNW_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LVCOLUMNW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVCOLUMNW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVCOLUMNW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVCOLUMNW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVCOLUMNW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVCOLUMNW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVFINDINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVFINDINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.psz == other.psz && self.lParam == other.lParam && self.pt == other.pt && self.vkDirection == other.vkDirection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVFINDINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVFINDINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFINDINFOA").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVFINDINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVFINDINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.psz == other.psz && self.lParam == other.lParam && self.pt == other.pt && self.vkDirection == other.vkDirection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVFINDINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVFINDINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFINDINFOW").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
impl ::core::default::Default for LVFINDINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVFINDINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVFINDINFOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVFINDINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVFINDINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LVFOOTERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVFOOTERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.cItems == other.cItems
    }
}
impl ::core::cmp::Eq for LVFOOTERINFO {}
impl ::core::fmt::Debug for LVFOOTERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFOOTERINFO").field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("cItems", &self.cItems).finish()
    }
}
impl ::core::default::Default for LVFOOTERITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVFOOTERITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.state == other.state && self.stateMask == other.stateMask
    }
}
impl ::core::cmp::Eq for LVFOOTERITEM {}
impl ::core::fmt::Debug for LVFOOTERITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFOOTERITEM").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("state", &self.state).field("stateMask", &self.stateMask).finish()
    }
}
impl ::core::default::Default for LVFOOTERITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVFOOTERITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVFOOTERITEM_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for LVGROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVGROUP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.mask == other.mask
            && self.pszHeader == other.pszHeader
            && self.cchHeader == other.cchHeader
            && self.pszFooter == other.pszFooter
            && self.cchFooter == other.cchFooter
            && self.iGroupId == other.iGroupId
            && self.stateMask == other.stateMask
            && self.state == other.state
            && self.uAlign == other.uAlign
            && self.pszSubtitle == other.pszSubtitle
            && self.cchSubtitle == other.cchSubtitle
            && self.pszTask == other.pszTask
            && self.cchTask == other.cchTask
            && self.pszDescriptionTop == other.pszDescriptionTop
            && self.cchDescriptionTop == other.cchDescriptionTop
            && self.pszDescriptionBottom == other.pszDescriptionBottom
            && self.cchDescriptionBottom == other.cchDescriptionBottom
            && self.iTitleImage == other.iTitleImage
            && self.iExtendedImage == other.iExtendedImage
            && self.iFirstItem == other.iFirstItem
            && self.cItems == other.cItems
            && self.pszSubsetTitle == other.pszSubsetTitle
            && self.cchSubsetTitle == other.cchSubsetTitle
    }
}
impl ::core::cmp::Eq for LVGROUP {}
impl ::core::fmt::Debug for LVGROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVGROUP")
            .field("cbSize", &self.cbSize)
            .field("mask", &self.mask)
            .field("pszHeader", &self.pszHeader)
            .field("cchHeader", &self.cchHeader)
            .field("pszFooter", &self.pszFooter)
            .field("cchFooter", &self.cchFooter)
            .field("iGroupId", &self.iGroupId)
            .field("stateMask", &self.stateMask)
            .field("state", &self.state)
            .field("uAlign", &self.uAlign)
            .field("pszSubtitle", &self.pszSubtitle)
            .field("cchSubtitle", &self.cchSubtitle)
            .field("pszTask", &self.pszTask)
            .field("cchTask", &self.cchTask)
            .field("pszDescriptionTop", &self.pszDescriptionTop)
            .field("cchDescriptionTop", &self.cchDescriptionTop)
            .field("pszDescriptionBottom", &self.pszDescriptionBottom)
            .field("cchDescriptionBottom", &self.cchDescriptionBottom)
            .field("iTitleImage", &self.iTitleImage)
            .field("iExtendedImage", &self.iExtendedImage)
            .field("iFirstItem", &self.iFirstItem)
            .field("cItems", &self.cItems)
            .field("pszSubsetTitle", &self.pszSubsetTitle)
            .field("cchSubsetTitle", &self.cchSubsetTitle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVGROUPMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVGROUPMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.mask == other.mask && self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom && self.crLeft == other.crLeft && self.crTop == other.crTop && self.crRight == other.crRight && self.crBottom == other.crBottom && self.crHeader == other.crHeader && self.crFooter == other.crFooter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVGROUPMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVGROUPMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVGROUPMETRICS").field("cbSize", &self.cbSize).field("mask", &self.mask).field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).field("crLeft", &self.crLeft).field("crTop", &self.crTop).field("crRight", &self.crRight).field("crBottom", &self.crBottom).field("crHeader", &self.crHeader).field("crFooter", &self.crFooter).finish()
    }
}
impl ::core::default::Default for LVGROUP_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVGROUP_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVGROUP_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVGROUP_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVGROUP_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVGROUP_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVGROUP_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVGROUP_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("iGroup", &self.iGroup).finish()
    }
}
impl ::core::default::Default for LVHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LVINSERTMARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iItem == other.iItem && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for LVINSERTMARK {}
impl ::core::fmt::Debug for LVINSERTMARK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVINSERTMARK").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iItem", &self.iItem).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam && self.iIndent == other.iIndent && self.iGroupId == other.iGroupId && self.cColumns == other.cColumns && self.puColumns == other.puColumns && self.piColFmt == other.piColFmt && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMA")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
impl ::core::default::Default for LVITEMA_GROUP_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVITEMA_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVITEMA_GROUP_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for LVITEMINDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVITEMINDEX {
    fn eq(&self, other: &Self) -> bool {
        self.iItem == other.iItem && self.iGroup == other.iGroup
    }
}
impl ::core::cmp::Eq for LVITEMINDEX {}
impl ::core::fmt::Debug for LVITEMINDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMINDEX").field("iItem", &self.iItem).field("iGroup", &self.iGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam && self.iIndent == other.iIndent && self.iGroupId == other.iGroupId && self.cColumns == other.cColumns && self.puColumns == other.puColumns && self.piColFmt == other.piColFmt && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMW")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
impl ::core::default::Default for LVSETINFOTIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVSETINFOTIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.iItem == other.iItem && self.iSubItem == other.iSubItem
    }
}
impl ::core::cmp::Eq for LVSETINFOTIP {}
impl ::core::fmt::Debug for LVSETINFOTIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVSETINFOTIP").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
impl ::core::default::Default for LVTILEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LVTILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iItem == other.iItem && self.cColumns == other.cColumns && self.puColumns == other.puColumns && self.piColFmt == other.piColFmt
    }
}
impl ::core::cmp::Eq for LVTILEINFO {}
impl ::core::fmt::Debug for LVTILEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVTILEINFO").field("cbSize", &self.cbSize).field("iItem", &self.iItem).field("cColumns", &self.cColumns).field("puColumns", &self.puColumns).field("piColFmt", &self.piColFmt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LVTILEVIEWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LVTILEVIEWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwFlags == other.dwFlags && self.sizeTile == other.sizeTile && self.cLines == other.cLines && self.rcLabelMargin == other.rcLabelMargin
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LVTILEVIEWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LVTILEVIEWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVTILEVIEWINFO").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwFlags", &self.dwFlags).field("sizeTile", &self.sizeTile).field("cLines", &self.cLines).field("rcLabelMargin", &self.rcLabelMargin).finish()
    }
}
impl ::core::default::Default for LVTILEVIEWINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVTILEVIEWINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVTILEVIEWINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVTILEVIEWINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVTILEVIEWINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LVTILEVIEWINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LVTILEVIEWINFO_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVTILEVIEWINFO_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVTILEVIEWINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVTILEVIEWINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVTILEVIEWINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MARGINS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MARGINS {
    fn eq(&self, other: &Self) -> bool {
        self.cxLeftWidth == other.cxLeftWidth && self.cxRightWidth == other.cxRightWidth && self.cyTopHeight == other.cyTopHeight && self.cyBottomHeight == other.cyBottomHeight
    }
}
impl ::core::cmp::Eq for MARGINS {}
impl ::core::fmt::Debug for MARGINS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MARGINS").field("cxLeftWidth", &self.cxLeftWidth).field("cxRightWidth", &self.cxRightWidth).field("cyTopHeight", &self.cyTopHeight).field("cyBottomHeight", &self.cyBottomHeight).finish()
    }
}
impl ::core::default::Default for MARKUPTEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MARKUPTEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MARKUPTEXTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAXBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MAXBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAXBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MAXCAPTIONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MAXCAPTIONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAXCAPTIONSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCGRIDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCGRIDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPart == other.dwPart && self.dwFlags == other.dwFlags && self.iCalendar == other.iCalendar && self.iRow == other.iRow && self.iCol == other.iCol && self.bSelected == other.bSelected && self.stStart == other.stStart && self.stEnd == other.stEnd && self.rc == other.rc && self.pszName == other.pszName && self.cchName == other.cchName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCGRIDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MCGRIDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCGRIDINFO").field("cbSize", &self.cbSize).field("dwPart", &self.dwPart).field("dwFlags", &self.dwFlags).field("iCalendar", &self.iCalendar).field("iRow", &self.iRow).field("iCol", &self.iCol).field("bSelected", &self.bSelected).field("stStart", &self.stStart).field("stEnd", &self.stEnd).field("rc", &self.rc).field("pszName", &self.pszName).field("cchName", &self.cchName).finish()
    }
}
impl ::core::default::Default for MCGRIDINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MCGRIDINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MCGRIDINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MCGRIDINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MCGRIDINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MCGRIDINFO_PART {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MCGRIDINFO_PART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MCGRIDINFO_PART").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pt == other.pt && self.uHit == other.uHit && self.st == other.st && self.rc == other.rc && self.iOffset == other.iOffset && self.iRow == other.iRow && self.iCol == other.iCol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MCHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCHITTESTINFO").field("cbSize", &self.cbSize).field("pt", &self.pt).field("uHit", &self.uHit).field("st", &self.st).field("rc", &self.rc).field("iOffset", &self.iOffset).field("iRow", &self.iRow).field("iCol", &self.iCol).finish()
    }
}
impl ::core::default::Default for MCHITTESTINFO_HIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MCHITTESTINFO_HIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MCHITTESTINFO_HIT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MCHITTESTINFO_HIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MCHITTESTINFO_HIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MCHITTESTINFO_HIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MDICLOSEBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MDICLOSEBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MDICLOSEBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MDIMINBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MDIMINBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MDIMINBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MDIRESTOREBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MDIRESTOREBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MDIRESTOREBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MEASUREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEASUREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.itemWidth == other.itemWidth && self.itemHeight == other.itemHeight && self.itemData == other.itemData
    }
}
impl ::core::cmp::Eq for MEASUREITEMSTRUCT {}
impl ::core::fmt::Debug for MEASUREITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEASUREITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("itemWidth", &self.itemWidth).field("itemHeight", &self.itemHeight).field("itemData", &self.itemData).finish()
    }
}
impl ::core::default::Default for MENUBANDPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUBANDPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUBANDPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MENUBANDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUBANDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUBANDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MENUPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MINCAPTIONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINCAPTIONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINCAPTIONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MONTHCALPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONTHCALPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONTHCALPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MONTH_CALDENDAR_MESSAGES_VIEW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONTH_CALDENDAR_MESSAGES_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONTH_CALDENDAR_MESSAGES_VIEW").field(&self.0).finish()
    }
}
impl ::core::default::Default for MOREPROGRAMSARROWBACKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOREPROGRAMSARROWBACKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSARROWBACKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MOREPROGRAMSARROWSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOREPROGRAMSARROWSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSARROWSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MOREPROGRAMSTABSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOREPROGRAMSTABSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSTABSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for MOVESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOVESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOVESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAVIGATIONPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAVIGATIONPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVIGATIONPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAVNEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAVNEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVNEXTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAVPREVSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAVPREVSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVPREVSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAV_BACKBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAV_BACKBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAV_BACKBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAV_FORWARDBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAV_FORWARDBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAV_FORWARDBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAV_MENUBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAV_MENUBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAV_MENUBUTTONSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMBCDROPDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMBCDROPDOWN {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMBCDROPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMBCDROPDOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMBCDROPDOWN").field("hdr", &self.hdr).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMBCHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMBCHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMBCHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMBCHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMBCHOTITEM").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCBEDRAGBEGINA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItemid == other.iItemid && self.szText == other.szText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCBEDRAGBEGINA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCBEDRAGBEGINA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEDRAGBEGINA").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCBEDRAGBEGINW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItemid == other.iItemid && self.szText == other.szText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCBEDRAGBEGINW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCBEDRAGBEGINW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEDRAGBEGINW").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCBEENDEDITA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCBEENDEDITA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.iNewSelection == other.iNewSelection && self.szText == other.szText && self.iWhy == other.iWhy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCBEENDEDITA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCBEENDEDITA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEENDEDITA").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCBEENDEDITW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCBEENDEDITW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.iNewSelection == other.iNewSelection && self.szText == other.szText && self.iWhy == other.iWhy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCBEENDEDITW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCBEENDEDITW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEENDEDITW").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCHAR {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ch == other.ch && self.dwItemPrev == other.dwItemPrev && self.dwItemNext == other.dwItemNext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCHAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCHAR").field("hdr", &self.hdr).field("ch", &self.ch).field("dwItemPrev", &self.dwItemPrev).field("dwItemNext", &self.dwItemNext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCOMBOBOXEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCOMBOBOXEXA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ceItem == other.ceItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCOMBOBOXEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCOMBOBOXEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCOMBOBOXEXA").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCOMBOBOXEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCOMBOBOXEXW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ceItem == other.ceItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCOMBOBOXEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCOMBOBOXEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCOMBOBOXEXW").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwDrawStage == other.dwDrawStage && self.hdc == other.hdc && self.rc == other.rc && self.dwItemSpec == other.dwItemSpec && self.uItemState == other.uItemState && self.lItemlParam == other.lItemlParam
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMDRAW").field("hdr", &self.hdr).field("dwDrawStage", &self.dwDrawStage).field("hdc", &self.hdc).field("rc", &self.rc).field("dwItemSpec", &self.dwItemSpec).field("uItemState", &self.uItemState).field("lItemlParam", &self.lItemlParam).finish()
    }
}
impl ::core::default::Default for NMCUSTOMDRAW_DRAW_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMCUSTOMDRAW_DRAW_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMCUSTOMDRAW_DRAW_STAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMCUSTOMDRAW_DRAW_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMCUSTOMSPLITRECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMCUSTOMSPLITRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcClient == other.rcClient && self.rcButton == other.rcButton && self.rcSplit == other.rcSplit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMCUSTOMSPLITRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMCUSTOMSPLITRECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMSPLITRECTINFO").field("hdr", &self.hdr).field("rcClient", &self.rcClient).field("rcButton", &self.rcButton).field("rcSplit", &self.rcSplit).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMCUSTOMTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMCUSTOMTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hDC == other.hDC && self.lpString == other.lpString && self.nCount == other.nCount && self.lpRect == other.lpRect && self.uFormat == other.uFormat && self.fLink == other.fLink
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMCUSTOMTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMCUSTOMTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMTEXT").field("hdr", &self.hdr).field("hDC", &self.hDC).field("lpString", &self.lpString).field("nCount", &self.nCount).field("lpRect", &self.lpRect).field("uFormat", &self.uFormat).field("fLink", &self.fLink).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMECHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMECHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.dwFlags == other.dwFlags && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMECHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMECHANGE").field("nmhdr", &self.nmhdr).field("dwFlags", &self.dwFlags).field("st", &self.st).finish()
    }
}
impl ::core::default::Default for NMDATETIMECHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMDATETIMECHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMDATETIMECHANGE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.st == other.st && self.pszDisplay == other.pszDisplay && self.szDisplay == other.szDisplay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEFORMATQUERYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEFORMATQUERYA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.szMax == other.szMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEFORMATQUERYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEFORMATQUERYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATQUERYA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEFORMATQUERYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEFORMATQUERYW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.szMax == other.szMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEFORMATQUERYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEFORMATQUERYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATQUERYW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.st == other.st && self.pszDisplay == other.pszDisplay && self.szDisplay == other.szDisplay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMESTRINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMESTRINGA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszUserString == other.pszUserString && self.st == other.st && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMESTRINGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMESTRINGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMESTRINGA").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMESTRINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMESTRINGW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszUserString == other.pszUserString && self.st == other.st && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMESTRINGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMESTRINGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMESTRINGW").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEWMKEYDOWNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEWMKEYDOWNA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.nVirtKey == other.nVirtKey && self.pszFormat == other.pszFormat && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEWMKEYDOWNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEWMKEYDOWNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEWMKEYDOWNA").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDATETIMEWMKEYDOWNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDATETIMEWMKEYDOWNW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.nVirtKey == other.nVirtKey && self.pszFormat == other.pszFormat && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDATETIMEWMKEYDOWNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDATETIMEWMKEYDOWNW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEWMKEYDOWNW").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMDAYSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMDAYSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.stStart == other.stStart && self.cDayState == other.cDayState && self.prgDayState == other.prgDayState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMDAYSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMDAYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDAYSTATE").field("nmhdr", &self.nmhdr).field("stStart", &self.stStart).field("cDayState", &self.cDayState).field("prgDayState", &self.prgDayState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMHDDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMHDDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMHDDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMHDDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDDISPINFOA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMHDDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMHDDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMHDDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMHDDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDDISPINFOW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMHDFILTERBTNCLICK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMHDFILTERBTNCLICK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.rc == other.rc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMHDFILTERBTNCLICK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMHDFILTERBTNCLICK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDFILTERBTNCLICK").field("hdr", &self.hdr).field("iItem", &self.iItem).field("rc", &self.rc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMHDR {
    fn eq(&self, other: &Self) -> bool {
        self.hwndFrom == other.hwndFrom && self.idFrom == other.idFrom && self.code == other.code
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDR").field("hwndFrom", &self.hwndFrom).field("idFrom", &self.idFrom).field("code", &self.code).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iButton == other.iButton && self.pitem == other.pitem
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMHEADERA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHEADERA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iButton == other.iButton && self.pitem == other.pitem
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMHEADERW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHEADERW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMIPADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMIPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iField == other.iField && self.iValue == other.iValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMIPADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMIPADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMIPADDRESS").field("hdr", &self.hdr).field("iField", &self.iField).field("iValue", &self.iValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMITEMACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMITEMACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.uNewState == other.uNewState && self.uOldState == other.uOldState && self.uChanged == other.uChanged && self.ptAction == other.ptAction && self.lParam == other.lParam && self.uKeyFlags == other.uKeyFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMITEMACTIVATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMITEMACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMITEMACTIVATE").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).field("uChanged", &self.uChanged).field("ptAction", &self.ptAction).field("lParam", &self.lParam).field("uKeyFlags", &self.uKeyFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMKEY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.nVKey == other.nVKey && self.uFlags == other.uFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMKEY").field("hdr", &self.hdr).field("nVKey", &self.nVKey).field("uFlags", &self.uFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLINK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLINK").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLISTVIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLISTVIEW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.uNewState == other.uNewState && self.uOldState == other.uOldState && self.uChanged == other.uChanged && self.ptAction == other.ptAction && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLISTVIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLISTVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLISTVIEW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).field("uChanged", &self.uChanged).field("ptAction", &self.ptAction).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVCACHEHINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVCACHEHINT {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iFrom == other.iFrom && self.iTo == other.iTo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVCACHEHINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVCACHEHINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVCACHEHINT").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMLVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMLVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.clrText == other.clrText && self.clrTextBk == other.clrTextBk && self.iSubItem == other.iSubItem && self.dwItemType == other.dwItemType && self.clrFace == other.clrFace && self.iIconEffect == other.iIconEffect && self.iIconPhase == other.iIconPhase && self.iPartId == other.iPartId && self.iStateId == other.iStateId && self.rcText == other.rcText && self.uAlign == other.uAlign
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMLVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMLVCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVCUSTOMDRAW").field("nmcd", &self.nmcd).field("clrText", &self.clrText).field("clrTextBk", &self.clrTextBk).field("iSubItem", &self.iSubItem).field("dwItemType", &self.dwItemType).field("clrFace", &self.clrFace).field("iIconEffect", &self.iIconEffect).field("iIconPhase", &self.iIconPhase).field("iPartId", &self.iPartId).field("iStateId", &self.iStateId).field("rcText", &self.rcText).field("uAlign", &self.uAlign).finish()
    }
}
impl ::core::default::Default for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVCUSTOMDRAW_ITEM_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVEMPTYMARKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVEMPTYMARKUP {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.szMarkup == other.szMarkup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVEMPTYMARKUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVEMPTYMARKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVEMPTYMARKUP").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("szMarkup", &self.szMarkup).finish()
    }
}
impl ::core::default::Default for NMLVEMPTYMARKUP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMLVEMPTYMARKUP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVEMPTYMARKUP_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVFINDITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVFINDITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iStart == other.iStart && self.lvfi == other.lvfi
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVFINDITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVFINDITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVFINDITEMA").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVFINDITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVFINDITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iStart == other.iStart && self.lvfi == other.lvfi
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVFINDITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVFINDITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVFINDITEMW").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVGETINFOTIPA").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVGETINFOTIPW").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
impl ::core::default::Default for NMLVGETINFOTIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMLVGETINFOTIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVGETINFOTIP_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVLINK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.link == other.link && self.iItem == other.iItem && self.iSubItem == other.iSubItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVLINK").field("hdr", &self.hdr).field("link", &self.link).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVODSTATECHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVODSTATECHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iFrom == other.iFrom && self.iTo == other.iTo && self.uNewState == other.uNewState && self.uOldState == other.uOldState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVODSTATECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVODSTATECHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVODSTATECHANGE").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMLVSCROLL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMLVSCROLL {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dx == other.dx && self.dy == other.dy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMLVSCROLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMLVSCROLL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVSCROLL").field("hdr", &self.hdr).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMMOUSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMMOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwItemSpec == other.dwItemSpec && self.dwItemData == other.dwItemData && self.pt == other.pt && self.dwHitInfo == other.dwHitInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMMOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMMOUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMMOUSE").field("hdr", &self.hdr).field("dwItemSpec", &self.dwItemSpec).field("dwItemData", &self.dwItemData).field("pt", &self.pt).field("dwHitInfo", &self.dwHitInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMOBJECTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.piid == other.piid && self.pObject == other.pObject && self.hResult == other.hResult && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMOBJECTNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMOBJECTNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMOBJECTNOTIFY").field("hdr", &self.hdr).field("iItem", &self.iItem).field("piid", &self.piid).field("pObject", &self.pObject).field("hResult", &self.hResult).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMPGCALCSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMPGCALCSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlag == other.dwFlag && self.iWidth == other.iWidth && self.iHeight == other.iHeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMPGCALCSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMPGCALCSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMPGCALCSIZE").field("hdr", &self.hdr).field("dwFlag", &self.dwFlag).field("iWidth", &self.iWidth).field("iHeight", &self.iHeight).finish()
    }
}
impl ::core::default::Default for NMPGCALCSIZE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMPGCALCSIZE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGCALCSIZE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMPGHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMPGHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.idOld == other.idOld && self.idNew == other.idNew && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMPGHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMPGHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMPGHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMPGSCROLL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NMPGSCROLL_DIR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMPGSCROLL_DIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGSCROLL_DIR").field(&self.0).finish()
    }
}
impl ::core::default::Default for NMPGSCROLL_KEYS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMPGSCROLL_KEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGSCROLL_KEYS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMPGSCROLL_KEYS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMPGSCROLL_KEYS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMPGSCROLL_KEYS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMRBAUTOSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMRBAUTOSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.rcTarget == other.rcTarget && self.rcActual == other.rcActual
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMRBAUTOSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMRBAUTOSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMRBAUTOSIZE").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("rcTarget", &self.rcTarget).field("rcActual", &self.rcActual).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMREBAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMREBAR {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.uBand == other.uBand && self.fStyle == other.fStyle && self.wID == other.wID && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMREBAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMREBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBAR").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("uBand", &self.uBand).field("fStyle", &self.fStyle).field("wID", &self.wID).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMREBARAUTOBREAK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMREBARAUTOBREAK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.lParam == other.lParam && self.uMsg == other.uMsg && self.fStyleCurrent == other.fStyleCurrent && self.fAutoBreak == other.fAutoBreak
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMREBARAUTOBREAK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMREBARAUTOBREAK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARAUTOBREAK").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("uMsg", &self.uMsg).field("fStyleCurrent", &self.fStyleCurrent).field("fAutoBreak", &self.fAutoBreak).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMREBARCHEVRON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMREBARCHEVRON {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.lParam == other.lParam && self.rc == other.rc && self.lParamNM == other.lParamNM
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMREBARCHEVRON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMREBARCHEVRON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARCHEVRON").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("rc", &self.rc).field("lParamNM", &self.lParamNM).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMREBARCHILDSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMREBARCHILDSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.rcChild == other.rcChild && self.rcBand == other.rcBand
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMREBARCHILDSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMREBARCHILDSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARCHILDSIZE").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("rcChild", &self.rcChild).field("rcBand", &self.rcBand).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMREBARSPLITTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMREBARSPLITTER {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcSizing == other.rcSizing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMREBARSPLITTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMREBARSPLITTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARSPLITTER").field("hdr", &self.hdr).field("rcSizing", &self.rcSizing).finish()
    }
}
impl ::core::default::Default for NMREBAR_MASK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMREBAR_MASK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMREBAR_MASK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMREBAR_MASK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMREBAR_MASK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMSEARCHWEB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMSEARCHWEB {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.entrypoint == other.entrypoint && self.hasQueryText == other.hasQueryText && self.invokeSucceeded == other.invokeSucceeded
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMSEARCHWEB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMSEARCHWEB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMSEARCHWEB").field("hdr", &self.hdr).field("entrypoint", &self.entrypoint).field("hasQueryText", &self.hasQueryText).field("invokeSucceeded", &self.invokeSucceeded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMSELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMSELCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.stSelStart == other.stSelStart && self.stSelEnd == other.stSelEnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMSELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMSELCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMSELCHANGE").field("nmhdr", &self.nmhdr).field("stSelStart", &self.stSelStart).field("stSelEnd", &self.stSelEnd).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMTBCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMTBCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.hbrMonoDither == other.hbrMonoDither && self.hbrLines == other.hbrLines && self.hpenLines == other.hpenLines && self.clrText == other.clrText && self.clrMark == other.clrMark && self.clrTextHighlight == other.clrTextHighlight && self.clrBtnFace == other.clrBtnFace && self.clrBtnHighlight == other.clrBtnHighlight && self.clrHighlightHotTrack == other.clrHighlightHotTrack && self.rcText == other.rcText && self.nStringBkMode == other.nStringBkMode && self.nHLStringBkMode == other.nHLStringBkMode && self.iListGap == other.iListGap
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMTBCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMTBCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBCUSTOMDRAW")
            .field("nmcd", &self.nmcd)
            .field("hbrMonoDither", &self.hbrMonoDither)
            .field("hbrLines", &self.hbrLines)
            .field("hpenLines", &self.hpenLines)
            .field("clrText", &self.clrText)
            .field("clrMark", &self.clrMark)
            .field("clrTextHighlight", &self.clrTextHighlight)
            .field("clrBtnFace", &self.clrBtnFace)
            .field("clrBtnHighlight", &self.clrBtnHighlight)
            .field("clrHighlightHotTrack", &self.clrHighlightHotTrack)
            .field("rcText", &self.rcText)
            .field("nStringBkMode", &self.nStringBkMode)
            .field("nHLStringBkMode", &self.nHLStringBkMode)
            .field("iListGap", &self.iListGap)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.lParam == other.lParam && self.iImage == other.iImage && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBDISPINFOA").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.lParam == other.lParam && self.iImage == other.iImage && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBDISPINFOW").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
impl ::core::default::Default for NMTBDISPINFOW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMTBDISPINFOW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMTBDISPINFOW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMTBDISPINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMTBDISPINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.idOld == other.idOld && self.idNew == other.idNew && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for NMTBHOTITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NMTBHOTITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMTBHOTITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMTBHOTITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMTBHOTITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBRESTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBRESTORE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pData == other.pData && self.pCurrent == other.pCurrent && self.cbData == other.cbData && self.iItem == other.iItem && self.cButtons == other.cButtons && self.cbBytesPerRecord == other.cbBytesPerRecord && self.tbButton == other.tbButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBRESTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBRESTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBRESTORE").field("hdr", &self.hdr).field("pData", &self.pData).field("pCurrent", &self.pCurrent).field("cbData", &self.cbData).field("iItem", &self.iItem).field("cButtons", &self.cButtons).field("cbBytesPerRecord", &self.cbBytesPerRecord).field("tbButton", &self.tbButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTBSAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTBSAVE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pData == other.pData && self.pCurrent == other.pCurrent && self.cbData == other.cbData && self.iItem == other.iItem && self.cButtons == other.cButtons && self.tbButton == other.tbButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTBSAVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTBSAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBSAVE").field("hdr", &self.hdr).field("pData", &self.pData).field("pCurrent", &self.pCurrent).field("cbData", &self.cbData).field("iItem", &self.iItem).field("cButtons", &self.cButtons).field("tbButton", &self.tbButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTCKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTOOLBARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTOOLBARA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.tbButton == other.tbButton && self.cchText == other.cchText && self.pszText == other.pszText && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTOOLBARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTOOLBARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLBARA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTOOLBARW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTOOLBARW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.tbButton == other.tbButton && self.cchText == other.cchText && self.pszText == other.pszText && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTOOLBARW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTOOLBARW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLBARW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTOOLTIPSCREATED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTOOLTIPSCREATED {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hwndToolTips == other.hwndToolTips
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTOOLTIPSCREATED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTOOLTIPSCREATED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLTIPSCREATED").field("hdr", &self.hdr).field("hwndToolTips", &self.hwndToolTips).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTRBTHUMBPOSCHANGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTRBTHUMBPOSCHANGING {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwPos == other.dwPos && self.nReason == other.nReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTRBTHUMBPOSCHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTRBTHUMBPOSCHANGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTRBTHUMBPOSCHANGING").field("hdr", &self.hdr).field("dwPos", &self.dwPos).field("nReason", &self.nReason).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTREEVIEWA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTREEVIEWA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.action == other.action && self.itemOld == other.itemOld && self.itemNew == other.itemNew && self.ptDrag == other.ptDrag
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTREEVIEWA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTREEVIEWA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTREEVIEWA").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTREEVIEWW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTREEVIEWW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.action == other.action && self.itemOld == other.itemOld && self.itemNew == other.itemNew && self.ptDrag == other.ptDrag
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTREEVIEWW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTREEVIEWW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTREEVIEWW").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMTTCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMTTCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.uDrawFlags == other.uDrawFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMTTCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMTTCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTCUSTOMDRAW").field("nmcd", &self.nmcd).field("uDrawFlags", &self.uDrawFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTTDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTTDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpszText == other.lpszText && self.szText == other.szText && self.hinst == other.hinst && self.uFlags == other.uFlags && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTTDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTTDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTDISPINFOA").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTTDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTTDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpszText == other.lpszText && self.szText == other.szText && self.hinst == other.hinst && self.uFlags == other.uFlags && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTTDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTTDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTDISPINFOW").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMTVASYNCDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMTVASYNCDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pimldp == other.pimldp && self.hr == other.hr && self.hItem == other.hItem && self.lParam == other.lParam && self.dwRetFlags == other.dwRetFlags && self.iRetImageIndex == other.iRetImageIndex
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMTVASYNCDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMTVASYNCDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVASYNCDRAW").field("hdr", &self.hdr).field("pimldp", &self.pimldp).field("hr", &self.hr).field("hItem", &self.hItem).field("lParam", &self.lParam).field("dwRetFlags", &self.dwRetFlags).field("iRetImageIndex", &self.iRetImageIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NMTVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NMTVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.clrText == other.clrText && self.clrTextBk == other.clrTextBk && self.iLevel == other.iLevel
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NMTVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for NMTVCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVCUSTOMDRAW").field("nmcd", &self.nmcd).field("clrText", &self.clrText).field("clrTextBk", &self.clrTextBk).field("iLevel", &self.iLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVDISPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVDISPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVDISPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVDISPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOEXA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVDISPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVDISPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVDISPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVDISPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOEXW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.hItem == other.hItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.hItem == other.hItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVITEMCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVITEMCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uChanged == other.uChanged && self.hItem == other.hItem && self.uStateNew == other.uStateNew && self.uStateOld == other.uStateOld && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVITEMCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVITEMCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVITEMCHANGE").field("hdr", &self.hdr).field("uChanged", &self.uChanged).field("hItem", &self.hItem).field("uStateNew", &self.uStateNew).field("uStateOld", &self.uStateOld).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMTVSTATEIMAGECHANGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMTVSTATEIMAGECHANGING {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hti == other.hti && self.iOldStateImageIndex == other.iOldStateImageIndex && self.iNewStateImageIndex == other.iNewStateImageIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMTVSTATEIMAGECHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMTVSTATEIMAGECHANGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVSTATEIMAGECHANGING").field("hdr", &self.hdr).field("hti", &self.hti).field("iOldStateImageIndex", &self.iOldStateImageIndex).field("iNewStateImageIndex", &self.iNewStateImageIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMUPDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMUPDOWN {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iPos == other.iPos && self.iDelta == other.iDelta
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMUPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMUPDOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMUPDOWN").field("hdr", &self.hdr).field("iPos", &self.iPos).field("iDelta", &self.iDelta).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NMVIEWCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NMVIEWCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.dwOldView == other.dwOldView && self.dwNewView == other.dwNewView
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NMVIEWCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NMVIEWCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMVIEWCHANGE").field("nmhdr", &self.nmhdr).field("dwOldView", &self.dwOldView).field("dwNewView", &self.dwNewView).finish()
    }
}
impl ::core::default::Default for NM_TREEVIEW_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NM_TREEVIEW_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NM_TREEVIEW_ACTION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NM_TREEVIEW_ACTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NM_TREEVIEW_ACTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NM_TREEVIEW_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NONESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NONESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NONESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NORMALGROUPCOLLAPSESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NORMALGROUPCOLLAPSESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NORMALGROUPCOLLAPSESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NORMALGROUPEXPANDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NORMALGROUPEXPANDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NORMALGROUPEXPANDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for ODA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ODA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ODA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ODS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ODS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ODS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFSETTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFSETTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFSETTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPENBOXSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPENBOXSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPENBOXSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPEN_THEME_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPEN_THEME_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_THEME_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPEN_THEME_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPEN_THEME_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PAGEPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGEPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGEPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PBRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PBRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iLow == other.iLow && self.iHigh == other.iHigh
    }
}
impl ::core::cmp::Eq for PBRANGE {}
impl ::core::fmt::Debug for PBRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PBRANGE").field("iLow", &self.iLow).field("iHigh", &self.iHigh).finish()
    }
}
impl ::core::default::Default for POINTER_DEVICE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTER_DEVICE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cursorId == other.cursorId && self.cursor == other.cursor
    }
}
impl ::core::cmp::Eq for POINTER_DEVICE_CURSOR_INFO {}
impl ::core::fmt::Debug for POINTER_DEVICE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_CURSOR_INFO").field("cursorId", &self.cursorId).field("cursor", &self.cursor).finish()
    }
}
impl ::core::default::Default for POINTER_DEVICE_CURSOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_DEVICE_CURSOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_DEVICE_CURSOR_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for POINTER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for POINTER_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.displayOrientation == other.displayOrientation && self.device == other.device && self.pointerDeviceType == other.pointerDeviceType && self.monitor == other.monitor && self.startingCursorId == other.startingCursorId && self.maxActiveContacts == other.maxActiveContacts && self.productString == other.productString
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for POINTER_DEVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for POINTER_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_INFO").field("displayOrientation", &self.displayOrientation).field("device", &self.device).field("pointerDeviceType", &self.pointerDeviceType).field("monitor", &self.monitor).field("startingCursorId", &self.startingCursorId).field("maxActiveContacts", &self.maxActiveContacts).field("productString", &self.productString).finish()
    }
}
impl ::core::default::Default for POINTER_DEVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTER_DEVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.logicalMin == other.logicalMin && self.logicalMax == other.logicalMax && self.physicalMin == other.physicalMin && self.physicalMax == other.physicalMax && self.unit == other.unit && self.unitExponent == other.unitExponent && self.usagePageId == other.usagePageId && self.usageId == other.usageId
    }
}
impl ::core::cmp::Eq for POINTER_DEVICE_PROPERTY {}
impl ::core::fmt::Debug for POINTER_DEVICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_PROPERTY").field("logicalMin", &self.logicalMin).field("logicalMax", &self.logicalMax).field("physicalMin", &self.physicalMin).field("physicalMax", &self.physicalMax).field("unit", &self.unit).field("unitExponent", &self.unitExponent).field("usagePageId", &self.usagePageId).field("usageId", &self.usageId).finish()
    }
}
impl ::core::default::Default for POINTER_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POINTER_FEEDBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_FEEDBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_FEEDBACK_MODE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for POPUPCHECKBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POPUPCHECKBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POPUPCHECKBACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for POPUPCHECKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POPUPCHECKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POPUPCHECKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for POPUPITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POPUPITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POPUPITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for POPUPSUBMENUSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POPUPSUBMENUSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POPUPSUBMENUSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROGRESSPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROGRESSPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROGRESSPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROPERTYORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROPERTYORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYORIGIN").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSHNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSHNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSHNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSHNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSHNOTIFY").field("hdr", &self.hdr).field("lParam", &self.lParam).finish()
    }
}
impl ::core::default::Default for PSPCB_MESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSPCB_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSPCB_MESSAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PUSHBUTTONDROPDOWNSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PUSHBUTTONDROPDOWNSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUSHBUTTONDROPDOWNSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for PUSHBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PUSHBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUSHBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for RADIOBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RADIOBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIOBUTTONSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RBHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RBHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iBand == other.iBand
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RBHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RBHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RBHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iBand", &self.iBand).finish()
    }
}
impl ::core::default::Default for READONLYSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READONLYSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READONLYSTATES").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for REBARBANDINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for REBARBANDINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.fStyle == other.fStyle && self.clrFore == other.clrFore && self.clrBack == other.clrBack && self.lpText == other.lpText && self.cch == other.cch && self.iImage == other.iImage && self.hwndChild == other.hwndChild && self.cxMinChild == other.cxMinChild && self.cyMinChild == other.cyMinChild && self.cx == other.cx && self.hbmBack == other.hbmBack && self.wID == other.wID && self.cyChild == other.cyChild && self.cyMaxChild == other.cyMaxChild && self.cyIntegral == other.cyIntegral && self.cxIdeal == other.cxIdeal && self.lParam == other.lParam && self.cxHeader == other.cxHeader && self.rcChevronLocation == other.rcChevronLocation && self.uChevronState == other.uChevronState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for REBARBANDINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for REBARBANDINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARBANDINFOA")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for REBARBANDINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for REBARBANDINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.fStyle == other.fStyle && self.clrFore == other.clrFore && self.clrBack == other.clrBack && self.lpText == other.lpText && self.cch == other.cch && self.iImage == other.iImage && self.hwndChild == other.hwndChild && self.cxMinChild == other.cxMinChild && self.cyMinChild == other.cyMinChild && self.cx == other.cx && self.hbmBack == other.hbmBack && self.wID == other.wID && self.cyChild == other.cyChild && self.cyMaxChild == other.cyMaxChild && self.cyIntegral == other.cyIntegral && self.cxIdeal == other.cxIdeal && self.lParam == other.lParam && self.cxHeader == other.cxHeader && self.rcChevronLocation == other.rcChevronLocation && self.uChevronState == other.uChevronState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for REBARBANDINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for REBARBANDINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARBANDINFOW")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
impl ::core::default::Default for REBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.himl == other.himl
    }
}
impl ::core::cmp::Eq for REBARINFO {}
impl ::core::fmt::Debug for REBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("himl", &self.himl).finish()
    }
}
impl ::core::default::Default for REBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REBARPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RESTOREBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESTOREBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCROLLBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLBARPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCROLLBARSTYLESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLBARSTYLESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLBARSTYLESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECTIONTITLELINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECTIONTITLELINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECTIONTITLELINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SET_THEME_APP_PROPERTIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_THEME_APP_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_THEME_APP_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SET_THEME_APP_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SET_THEME_APP_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SET_THEME_APP_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SHOWCALENDARBUTTONRIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHOWCALENDARBUTTONRIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHOWCALENDARBUTTONRIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SIZEBOXSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIZEBOXSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIZEBOXSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SIZINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIZINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIZINGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMALLCAPTIONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMALLCAPTIONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMALLCAPTIONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMALLCLOSEBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMALLCLOSEBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMALLCLOSEBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMALLFRAMEBOTTOMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMALLFRAMEBOTTOMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMALLFRAMEBOTTOMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMALLFRAMELEFTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMALLFRAMELEFTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMALLFRAMELEFTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMALLFRAMERIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMALLFRAMERIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMALLFRAMERIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOFTWAREEXPLORERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOFTWAREEXPLORERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOFTWAREEXPLORERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPECIALGROUPCOLLAPSESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPECIALGROUPCOLLAPSESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPECIALGROUPCOLLAPSESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPECIALGROUPEXPANDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPECIALGROUPEXPANDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPECIALGROUPEXPANDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPINPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPINPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPINPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPLITTERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPLITTERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPLITTERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SPLITTERVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPLITTERVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPLITTERVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for STANDARDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STANDARDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STANDARDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for STARTPANELPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STARTPANELPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTPANELPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STATICPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STATICPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATICPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STATUSPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STATUSPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATUSPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSBUTTONSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSBUTTONSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSBUTTONSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEMCLOSESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEMCLOSESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEMCLOSESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEMMAXIMIZESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEMMAXIMIZESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEMMAXIMIZESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEMMINIMIZESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEMMINIMIZESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEMMINIMIZESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEMRESTORESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEMRESTORESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEMRESTORESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABITEMBOTHEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABITEMBOTHEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABITEMBOTHEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABITEMLEFTEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABITEMLEFTEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABITEMLEFTEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABITEMRIGHTEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABITEMRIGHTEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABITEMRIGHTEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TABSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TABSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TABSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAB_CONTROL_ITEM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAB_CONTROL_ITEM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAB_CONTROL_ITEM_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKBANDPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKBANDPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKBANDPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKBARPARTS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for TASKDIALOGCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TASKDIALOGPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOGPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOGPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_BUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_COMMON_BUTTON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_ELEMENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_ELEMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_ELEMENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_ICON_ELEMENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_ICON_ELEMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_ICON_ELEMENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_MESSAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_MESSAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_MESSAGES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKDIALOG_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKDIALOG_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_NOTIFICATIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TASKLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TASKLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKLINKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TA_CUBIC_BEZIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_CUBIC_BEZIER {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rX0 == other.rX0 && self.rY0 == other.rY0 && self.rX1 == other.rX1 && self.rY1 == other.rY1
    }
}
impl ::core::cmp::Eq for TA_CUBIC_BEZIER {}
impl ::core::fmt::Debug for TA_CUBIC_BEZIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_CUBIC_BEZIER").field("header", &self.header).field("rX0", &self.rX0).field("rY0", &self.rY0).field("rX1", &self.rX1).field("rY1", &self.rY1).finish()
    }
}
impl ::core::default::Default for TA_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for TA_PROPERTY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TA_PROPERTY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_PROPERTY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TA_PROPERTY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TA_PROPERTY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TA_PROPERTY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TA_TIMINGFUNCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_TIMINGFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        self.eTimingFunctionType == other.eTimingFunctionType
    }
}
impl ::core::cmp::Eq for TA_TIMINGFUNCTION {}
impl ::core::fmt::Debug for TA_TIMINGFUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TIMINGFUNCTION").field("eTimingFunctionType", &self.eTimingFunctionType).finish()
    }
}
impl ::core::default::Default for TA_TIMINGFUNCTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TA_TIMINGFUNCTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TIMINGFUNCTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eTransformType == other.eTransformType && self.dwTimingFunctionId == other.dwTimingFunctionId && self.dwStartTime == other.dwStartTime && self.dwDurationTime == other.dwDurationTime && self.eFlags == other.eFlags
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM {}
impl ::core::fmt::Debug for TA_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM").field("eTransformType", &self.eTransformType).field("dwTimingFunctionId", &self.dwTimingFunctionId).field("dwStartTime", &self.dwStartTime).field("dwDurationTime", &self.dwDurationTime).field("eFlags", &self.eFlags).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_2D {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rX == other.rX && self.rY == other.rY && self.rInitialX == other.rInitialX && self.rInitialY == other.rInitialY && self.rOriginX == other.rOriginX && self.rOriginY == other.rOriginY
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_2D {}
impl ::core::fmt::Debug for TA_TRANSFORM_2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_2D").field("header", &self.header).field("rX", &self.rX).field("rY", &self.rY).field("rInitialX", &self.rInitialX).field("rInitialY", &self.rInitialY).field("rOriginX", &self.rOriginX).field("rOriginY", &self.rOriginY).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM_CLIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_CLIP {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rLeft == other.rLeft && self.rTop == other.rTop && self.rRight == other.rRight && self.rBottom == other.rBottom && self.rInitialLeft == other.rInitialLeft && self.rInitialTop == other.rInitialTop && self.rInitialRight == other.rInitialRight && self.rInitialBottom == other.rInitialBottom
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_CLIP {}
impl ::core::fmt::Debug for TA_TRANSFORM_CLIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_CLIP").field("header", &self.header).field("rLeft", &self.rLeft).field("rTop", &self.rTop).field("rRight", &self.rRight).field("rBottom", &self.rBottom).field("rInitialLeft", &self.rInitialLeft).field("rInitialTop", &self.rInitialTop).field("rInitialRight", &self.rInitialRight).field("rInitialBottom", &self.rInitialBottom).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TRANSFORM_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM_OPACITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_OPACITY {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rOpacity == other.rOpacity && self.rInitialOpacity == other.rInitialOpacity
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_OPACITY {}
impl ::core::fmt::Debug for TA_TRANSFORM_OPACITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_OPACITY").field("header", &self.header).field("rOpacity", &self.rOpacity).field("rInitialOpacity", &self.rInitialOpacity).finish()
    }
}
impl ::core::default::Default for TA_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TBADDBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TBADDBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.hInst == other.hInst && self.nID == other.nID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TBADDBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TBADDBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBADDBITMAP").field("hInst", &self.hInst).field("nID", &self.nID).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.iBitmap == other.iBitmap && self.idCommand == other.idCommand && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.bReserved == other.bReserved && self.dwData == other.dwData && self.iString == other.iString
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for TBBUTTON {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for TBBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.iBitmap == other.iBitmap && self.idCommand == other.idCommand && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.bReserved == other.bReserved && self.dwData == other.dwData && self.iString == other.iString
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for TBBUTTON {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for TBBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
impl ::core::default::Default for TBBUTTONINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBBUTTONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.iImage == other.iImage && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.cx == other.cx && self.lParam == other.lParam && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
impl ::core::cmp::Eq for TBBUTTONINFOA {}
impl ::core::fmt::Debug for TBBUTTONINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTONINFOA").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("iImage", &self.iImage).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("cx", &self.cx).field("lParam", &self.lParam).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
impl ::core::default::Default for TBBUTTONINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBBUTTONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.iImage == other.iImage && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.cx == other.cx && self.lParam == other.lParam && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
impl ::core::cmp::Eq for TBBUTTONINFOW {}
impl ::core::fmt::Debug for TBBUTTONINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTONINFOW").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("iImage", &self.iImage).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("cx", &self.cx).field("lParam", &self.lParam).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
impl ::core::default::Default for TBBUTTONINFOW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TBBUTTONINFOW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBBUTTONINFOW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TBBUTTONINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TBBUTTONINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TBINSERTMARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        self.iButton == other.iButton && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TBINSERTMARK {}
impl ::core::fmt::Debug for TBINSERTMARK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBINSERTMARK").field("iButton", &self.iButton).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for TBINSERTMARK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TBINSERTMARK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBINSERTMARK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TBMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TBMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.cxPad == other.cxPad && self.cyPad == other.cyPad && self.cxBarPad == other.cxBarPad && self.cyBarPad == other.cyBarPad && self.cxButtonSpacing == other.cxButtonSpacing && self.cyButtonSpacing == other.cyButtonSpacing
    }
}
impl ::core::cmp::Eq for TBMETRICS {}
impl ::core::fmt::Debug for TBMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBMETRICS").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("cxPad", &self.cxPad).field("cyPad", &self.cyPad).field("cxBarPad", &self.cxBarPad).field("cyBarPad", &self.cyBarPad).field("cxButtonSpacing", &self.cxButtonSpacing).field("cyButtonSpacing", &self.cyButtonSpacing).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TBREPLACEBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TBREPLACEBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.hInstOld == other.hInstOld && self.nIDOld == other.nIDOld && self.hInstNew == other.hInstNew && self.nIDNew == other.nIDNew && self.nButtons == other.nButtons
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TBREPLACEBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TBREPLACEBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBREPLACEBITMAP").field("hInstOld", &self.hInstOld).field("nIDOld", &self.nIDOld).field("hInstNew", &self.hInstNew).field("nIDNew", &self.nIDNew).field("nButtons", &self.nButtons).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for TBSAVEPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for TBSAVEPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.hkr == other.hkr && self.pszSubKey == other.pszSubKey && self.pszValueName == other.pszValueName
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for TBSAVEPARAMSA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for TBSAVEPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBSAVEPARAMSA").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for TBSAVEPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for TBSAVEPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.hkr == other.hkr && self.pszSubKey == other.pszSubKey && self.pszValueName == other.pszValueName
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for TBSAVEPARAMSW {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for TBSAVEPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBSAVEPARAMSW").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for TCHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMA").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
impl ::core::default::Default for TCITEMHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCITEMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.lpReserved1 == other.lpReserved1 && self.lpReserved2 == other.lpReserved2 && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage
    }
}
impl ::core::cmp::Eq for TCITEMHEADERA {}
impl ::core::fmt::Debug for TCITEMHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMHEADERA").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
impl ::core::default::Default for TCITEMHEADERA_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCITEMHEADERA_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCITEMHEADERA_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TCITEMHEADERA_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TCITEMHEADERA_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TCITEMHEADERA_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TCITEMHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCITEMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.lpReserved1 == other.lpReserved1 && self.lpReserved2 == other.lpReserved2 && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage
    }
}
impl ::core::cmp::Eq for TCITEMHEADERW {}
impl ::core::fmt::Debug for TCITEMHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMHEADERW").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMW").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
impl ::core::default::Default for TEXTSELECTIONGRIPPERPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXTSELECTIONGRIPPERPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTSELECTIONGRIPPERPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXTSHADOWTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXTSHADOWTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTSHADOWTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXTSTYLEPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXTSTYLEPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTSTYLEPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for THEMESIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THEMESIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THEMESIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for THEME_PROPERTY_SYMBOL_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THEME_PROPERTY_SYMBOL_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THEME_PROPERTY_SYMBOL_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBBOTTOMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBBOTTOMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBBOTTOMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBLEFTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBLEFTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBLEFTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBRIGHTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBRIGHTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBRIGHTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBTOPSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBTOPSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBTOPSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for THUMBVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THUMBVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THUMBVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TICSSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TICSSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TICSSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TICSVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TICSVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TICSVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TITLEBARSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TITLEBARSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TITLEBARSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOOLBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOOLBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOOLBARPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOOLBARSTYLESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOOLBARSTYLESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOOLBARSTYLESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOOLTIPPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOOLTIPPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOOLTIPPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOOLTIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOOLTIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOOLTIP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOOLTIP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOOLTIP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOOLTIP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOOLTIP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOOLTIP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TOPTABITEMBOTHEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOPTABITEMBOTHEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOPTABITEMBOTHEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOPTABITEMLEFTEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOPTABITEMLEFTEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOPTABITEMLEFTEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOPTABITEMRIGHTEDGESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOPTABITEMRIGHTEDGESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOPTABITEMRIGHTEDGESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOPTABITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOPTABITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOPTABITEMSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOUCH_HIT_TESTING_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOUCH_HIT_TESTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pointerId == other.pointerId && self.point == other.point && self.boundingBox == other.boundingBox && self.nonOccludedBoundingBox == other.nonOccludedBoundingBox && self.orientation == other.orientation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOUCH_HIT_TESTING_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOUCH_HIT_TESTING_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCH_HIT_TESTING_INPUT").field("pointerId", &self.pointerId).field("point", &self.point).field("boundingBox", &self.boundingBox).field("nonOccludedBoundingBox", &self.nonOccludedBoundingBox).field("orientation", &self.orientation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.adjustedPoint == other.adjustedPoint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCH_HIT_TESTING_PROXIMITY_EVALUATION").field("score", &self.score).field("adjustedPoint", &self.adjustedPoint).finish()
    }
}
impl ::core::default::Default for TRACKBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKBARPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRACKBARSTYLESTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKBARSTYLESTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKBARSTYLESTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRACKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRACKVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRAILINGGRIDCELLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRAILINGGRIDCELLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAILINGGRIDCELLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRAILINGGRIDCELLUPPERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRAILINGGRIDCELLUPPERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAILINGGRIDCELLUPPERSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSPARENTBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSPARENTBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPARENTBACKGROUNDSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSPARENTBARSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSPARENTBARSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPARENTBARSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSPARENTBARVERTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSPARENTBARVERTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPARENTBARVERTSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRAYNOTIFYPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRAYNOTIFYPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAYNOTIFYPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TREEITEMSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TREEITEMSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TREEITEMSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TREEVIEWPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TREEVIEWPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TREEVIEWPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TREE_VIEW_ITEM_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TREE_VIEW_ITEM_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TREE_VIEW_ITEM_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRUESIZESCALINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRUESIZESCALINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUESIZESCALINGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TTGETTITLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTGETTITLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uTitleBitmap == other.uTitleBitmap && self.cch == other.cch && self.pszTitle == other.pszTitle
    }
}
impl ::core::cmp::Eq for TTGETTITLE {}
impl ::core::fmt::Debug for TTGETTITLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTGETTITLE").field("dwSize", &self.dwSize).field("uTitleBitmap", &self.uTitleBitmap).field("cch", &self.cch).field("pszTitle", &self.pszTitle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TTHITTESTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TTHITTESTINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pt == other.pt && self.ti == other.ti
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TTHITTESTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TTHITTESTINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTHITTESTINFOA").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TTHITTESTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TTHITTESTINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pt == other.pt && self.ti == other.ti
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TTHITTESTINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TTHITTESTINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTHITTESTINFOW").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TTTOOLINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TTTOOLINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.uFlags == other.uFlags && self.hwnd == other.hwnd && self.uId == other.uId && self.rect == other.rect && self.hinst == other.hinst && self.lpszText == other.lpszText && self.lParam == other.lParam && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TTTOOLINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TTTOOLINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTTOOLINFOA").field("cbSize", &self.cbSize).field("uFlags", &self.uFlags).field("hwnd", &self.hwnd).field("uId", &self.uId).field("rect", &self.rect).field("hinst", &self.hinst).field("lpszText", &self.lpszText).field("lParam", &self.lParam).field("lpReserved", &self.lpReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TTTOOLINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TTTOOLINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.uFlags == other.uFlags && self.hwnd == other.hwnd && self.uId == other.uId && self.rect == other.rect && self.hinst == other.hinst && self.lpszText == other.lpszText && self.lParam == other.lParam && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TTTOOLINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TTTOOLINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTTOOLINFOW").field("cbSize", &self.cbSize).field("uFlags", &self.uFlags).field("hwnd", &self.hwnd).field("uId", &self.uId).field("rect", &self.rect).field("hinst", &self.hinst).field("lpszText", &self.lpszText).field("lParam", &self.lParam).field("lpReserved", &self.lpReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVGETITEMPARTRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hti == other.hti && self.prc == other.prc && self.partID == other.partID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVGETITEMPARTRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVGETITEMPARTRECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVGETITEMPARTRECTINFO").field("hti", &self.hti).field("prc", &self.prc).field("partID", &self.partID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.hItem == other.hItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("hItem", &self.hItem).finish()
    }
}
impl ::core::default::Default for TVHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TVHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMA").field("mask", &self.mask).field("hItem", &self.hItem).field("state", &self.state).field("stateMask", &self.stateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("cChildren", &self.cChildren).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVITEMEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVITEMEXA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam && self.iIntegral == other.iIntegral && self.uStateEx == other.uStateEx && self.hwnd == other.hwnd && self.iExpandedImage == other.iExpandedImage && self.iReserved == other.iReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVITEMEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVITEMEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMEXA")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVITEMEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVITEMEXW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam && self.iIntegral == other.iIntegral && self.uStateEx == other.uStateEx && self.hwnd == other.hwnd && self.iExpandedImage == other.iExpandedImage && self.iReserved == other.iReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVITEMEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVITEMEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMEXW")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
impl ::core::default::Default for TVITEMEXW_CHILDREN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TVITEMEXW_CHILDREN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEMEXW_CHILDREN").field(&self.0).finish()
    }
}
impl ::core::default::Default for TVITEMPART {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TVITEMPART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEMPART").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TVITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TVITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMW").field("mask", &self.mask).field("hItem", &self.hItem).field("state", &self.state).field("stateMask", &self.stateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("cChildren", &self.cChildren).field("lParam", &self.lParam).finish()
    }
}
impl ::core::default::Default for TVITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TVITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEM_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TVITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TVITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TVITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TVITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TVITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TVSORTCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UDACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UDACCEL {
    fn eq(&self, other: &Self) -> bool {
        self.nSec == other.nSec && self.nInc == other.nInc
    }
}
impl ::core::cmp::Eq for UDACCEL {}
impl ::core::fmt::Debug for UDACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UDACCEL").field("nSec", &self.nSec).field("nInc", &self.nInc).finish()
    }
}
impl ::core::default::Default for UPDATEMETADATASTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPDATEMETADATASTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDATEMETADATASTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for UPHORZSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPHORZSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPHORZSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for UPSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for USAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level && self.page == other.page && self.usage == other.usage && self.logicalMinimum == other.logicalMinimum && self.logicalMaximum == other.logicalMaximum && self.unit == other.unit && self.exponent == other.exponent && self.count == other.count && self.physicalMinimum == other.physicalMinimum && self.physicalMaximum == other.physicalMaximum
    }
}
impl ::core::cmp::Eq for USAGE_PROPERTIES {}
impl ::core::fmt::Debug for USAGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USAGE_PROPERTIES").field("level", &self.level).field("page", &self.page).field("usage", &self.usage).field("logicalMinimum", &self.logicalMinimum).field("logicalMaximum", &self.logicalMaximum).field("unit", &self.unit).field("exponent", &self.exponent).field("count", &self.count).field("physicalMinimum", &self.physicalMinimum).field("physicalMaximum", &self.physicalMaximum).finish()
    }
}
impl ::core::default::Default for USERTILEPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USERTILEPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERTILEPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VALIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALIGN").field(&self.0).finish()
    }
}
impl ::core::default::Default for VERTSCROLLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VERTSCROLLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VERTSCROLLSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for VERTTHUMBSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VERTTHUMBSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VERTTHUMBSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WARNINGSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WARNINGSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WARNINGSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOWPARTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOWPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWPARTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOWTHEMEATTRIBUTETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOWTHEMEATTRIBUTETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWTHEMEATTRIBUTETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WORD_BREAK_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORD_BREAK_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORD_BREAK_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRENCHSTATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRENCHSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRENCHSTATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSB_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSB_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSB_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTA_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTA_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwMask == other.dwMask
    }
}
impl ::core::cmp::Eq for WTA_OPTIONS {}
impl ::core::fmt::Debug for WTA_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTA_OPTIONS").field("dwFlags", &self.dwFlags).field("dwMask", &self.dwMask).finish()
    }
}
impl ::core::default::Default for _LI_METRIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _LI_METRIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_LI_METRIC").field(&self.0).finish()
    }
}
