windows_link::link!("usp10.dll" "system" fn ScriptApplyDigitSubstitution(psds : *const SCRIPT_DIGITSUBSTITUTE, psc : *mut SCRIPT_CONTROL, pss : *mut SCRIPT_STATE) -> windows_sys::core::HRESULT);
#[cfg(feature = "wingdi")]
windows_link::link!("usp10.dll" "system" fn ScriptApplyLogicalWidth(pidx : *const i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pabc : *mut super::ABC, pijustify : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptBreak(pwcchars : *const u16, cchars : i32, psa : *const SCRIPT_ANALYSIS, psla : *mut SCRIPT_LOGATTR) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptCPtoX(icp : i32, ftrailing : windows_sys::core::BOOL, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pix : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptCacheGetHeight(hdc : super::HDC, psc : *mut SCRIPT_CACHE, tmheight : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptFreeCache(psc : *mut SCRIPT_CACHE) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetCMap(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwcinchars : *const u16, cchars : i32, dwflags : u32, pwoutglyphs : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetFontAlternateGlyphs(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, wglyphid : u16, cmaxalternates : i32, palternateglyphs : *mut u16, pcalternates : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetFontFeatureTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, cmaxtags : i32, pfeaturetags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetFontLanguageTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, cmaxtags : i32, plangsystags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetFontProperties(hdc : super::HDC, psc : *mut SCRIPT_CACHE, sfp : *mut SCRIPT_FONTPROPERTIES) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptGetFontScriptTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, cmaxtags : i32, pscripttags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("usp10.dll" "system" fn ScriptGetGlyphABCWidth(hdc : super::HDC, psc : *mut SCRIPT_CACHE, wglyph : u16, pabc : *mut super::ABC) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptGetLogicalWidths(psa : *const SCRIPT_ANALYSIS, cchars : i32, cglyphs : i32, piglyphwidth : *const i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, pidx : *const i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptGetProperties(ppsp : *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptIsComplex(pwcinchars : *const u16, cinchars : i32, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptItemize(pwcinchars : *const u16, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pcitems : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptItemizeOpenType(pwcinchars : *const u16, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pscripttags : *mut OPENTYPE_TAG, pcitems : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptJustify(psva : *const SCRIPT_VISATTR, piadvance : *const i32, cglyphs : i32, idx : i32, iminkashida : i32, pijustify : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptLayout(cruns : i32, pblevel : *const u8, pivisualtological : *mut i32, pilogicaltovisual : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("usp10.dll" "system" fn ScriptPlace(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwglyphs : *const u16, cglyphs : i32, psva : *const SCRIPT_VISATTR, psa : *mut SCRIPT_ANALYSIS, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::ABC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("usp10.dll" "system" fn ScriptPlaceOpenType(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, pwlogclust : *const u16, pcharprops : *const SCRIPT_CHARPROP, cchars : i32, pwglyphs : *const u16, pglyphprops : *const SCRIPT_GLYPHPROP, cglyphs : i32, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::ABC) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptPositionSingleGlyph(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, iadvance : i32, goffset : GOFFSET, pioutadvance : *mut i32, poutgoffset : *mut GOFFSET) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("usp10.dll" "system" fn ScriptRecordDigitSubstitution(locale : super::LCID, psds : *mut SCRIPT_DIGITSUBSTITUTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptShape(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, psa : *mut SCRIPT_ANALYSIS, pwoutglyphs : *mut u16, pwlogclust : *mut u16, psva : *mut SCRIPT_VISATTR, pcglyphs : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptShapeOpenType(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, pwlogclust : *mut u16, pcharprops : *mut SCRIPT_CHARPROP, pwoutglyphs : *mut u16, poutglyphprops : *mut SCRIPT_GLYPHPROP, pcglyphs : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptStringAnalyse(hdc : super::HDC, pstring : *const core::ffi::c_void, cstring : i32, cglyphs : i32, icharset : i32, dwflags : u32, ireqwidth : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pidx : *const i32, ptabdef : *const SCRIPT_TABDEF, pbinclass : *const u8, pssa : *mut SCRIPT_STRING_ANALYSIS) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringCPtoX(ssa : SCRIPT_STRING_ANALYSIS, icp : i32, ftrailing : windows_sys::core::BOOL, px : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringFree(pssa : *mut SCRIPT_STRING_ANALYSIS) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringGetLogicalWidths(ssa : SCRIPT_STRING_ANALYSIS, pidx : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringGetOrder(ssa : SCRIPT_STRING_ANALYSIS, puorder : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptStringOut(ssa : SCRIPT_STRING_ANALYSIS, ix : i32, iy : i32, uoptions : u32, prc : *const super::RECT, iminsel : i32, imaxsel : i32, fdisabled : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringValidate(ssa : SCRIPT_STRING_ANALYSIS) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptStringXtoCP(ssa : SCRIPT_STRING_ANALYSIS, ix : i32, pich : *mut i32, pitrailing : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptString_pLogAttr(ssa : SCRIPT_STRING_ANALYSIS) -> *const SCRIPT_LOGATTR);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptString_pSize(ssa : SCRIPT_STRING_ANALYSIS) -> *const super::SIZE);
windows_link::link!("usp10.dll" "system" fn ScriptString_pcOutChars(ssa : SCRIPT_STRING_ANALYSIS) -> *const i32);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptSubstituteSingleGlyph(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, pwoutglyphid : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("usp10.dll" "system" fn ScriptTextOut(hdc : super::HDC, psc : *mut SCRIPT_CACHE, x : i32, y : i32, fuoptions : u32, lprc : *const super::RECT, psa : *const SCRIPT_ANALYSIS, pwcreserved : *const u16, ireserved : i32, pwglyphs : *const u16, cglyphs : i32, piadvance : *const i32, pijustify : *const i32, pgoffset : *const GOFFSET) -> windows_sys::core::HRESULT);
windows_link::link!("usp10.dll" "system" fn ScriptXtoCP(ix : i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, picp : *mut i32, pitrailing : *mut i32) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: OPENTYPE_TAG,
    pub lParameter: i32,
}
pub type OPENTYPE_TAG = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
}
pub type SCRIPT_CACHE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_CHARPROP {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_DIGITSUBSTITUTE {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub dwReserved: u32,
}
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_ITEM {
    pub iCharPos: i32,
    pub a: SCRIPT_ANALYSIS,
}
pub type SCRIPT_JUSTIFY = i32;
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: SCRIPT_JUSTIFY = 9;
pub const SCRIPT_JUSTIFY_ARABIC_BA: SCRIPT_JUSTIFY = 12;
pub const SCRIPT_JUSTIFY_ARABIC_BARA: SCRIPT_JUSTIFY = 13;
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: SCRIPT_JUSTIFY = 1;
pub const SCRIPT_JUSTIFY_ARABIC_HA: SCRIPT_JUSTIFY = 10;
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: SCRIPT_JUSTIFY = 8;
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: SCRIPT_JUSTIFY = 7;
pub const SCRIPT_JUSTIFY_ARABIC_RA: SCRIPT_JUSTIFY = 11;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: SCRIPT_JUSTIFY = 14;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: SCRIPT_JUSTIFY = 15;
pub const SCRIPT_JUSTIFY_BLANK: SCRIPT_JUSTIFY = 4;
pub const SCRIPT_JUSTIFY_CHARACTER: SCRIPT_JUSTIFY = 2;
pub const SCRIPT_JUSTIFY_NONE: SCRIPT_JUSTIFY = 0;
pub const SCRIPT_JUSTIFY_RESERVED1: SCRIPT_JUSTIFY = 3;
pub const SCRIPT_JUSTIFY_RESERVED2: SCRIPT_JUSTIFY = 5;
pub const SCRIPT_JUSTIFY_RESERVED3: SCRIPT_JUSTIFY = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
}
pub type SCRIPT_STRING_ANALYSIS = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCRIPT_TABDEF {
    pub cTabStops: i32,
    pub iScale: i32,
    pub pTabStops: *mut i32,
    pub iTabOrigin: i32,
}
impl Default for SCRIPT_TABDEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRIPT_TAG_UNKNOWN: u32 = 0;
pub const SCRIPT_UNDEFINED: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCRIPT_VISATTR {
    pub _bitfield: u16,
}
pub const SGCM_RTL: u32 = 1;
pub const SIC_ASCIIDIGIT: u32 = 2;
pub const SIC_COMPLEX: u32 = 1;
pub const SIC_NEUTRAL: u32 = 4;
pub const SSA_BREAK: u32 = 64;
pub const SSA_CLIP: u32 = 4;
pub const SSA_DONTGLYPH: u32 = 1073741824;
pub const SSA_DZWG: u32 = 16;
pub const SSA_FALLBACK: u32 = 32;
pub const SSA_FIT: u32 = 8;
pub const SSA_FULLMEASURE: u32 = 67108864;
pub const SSA_GCP: u32 = 512;
pub const SSA_GLYPHS: u32 = 128;
pub const SSA_HIDEHOTKEY: u32 = 8192;
pub const SSA_HOTKEY: u32 = 1024;
pub const SSA_HOTKEYONLY: u32 = 9216;
pub const SSA_LAYOUTRTL: u32 = 536870912;
pub const SSA_LINK: u32 = 4096;
pub const SSA_LPKANSIFALLBACK: u32 = 134217728;
pub const SSA_METAFILE: u32 = 2048;
pub const SSA_NOKASHIDA: u32 = 2147483648;
pub const SSA_PASSWORD: u32 = 1;
pub const SSA_PIDX: u32 = 268435456;
pub const SSA_RTL: u32 = 256;
pub const SSA_TAB: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TEXTRANGE_PROPERTIES {
    pub potfRecords: *mut OPENTYPE_FEATURE_RECORD,
    pub cotfRecords: i32,
}
impl Default for TEXTRANGE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNISCRIBE_OPENTYPE: u32 = 256;
pub const USP_E_SCRIPT_NOT_IN_FONT: i32 = -2147220992;
