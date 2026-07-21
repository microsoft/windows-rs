#[inline]
pub unsafe fn ScriptApplyDigitSubstitution(psds: &[SCRIPT_DIGITSUBSTITUTE; 1], psc: &mut [SCRIPT_CONTROL; 1], pss: &mut [SCRIPT_STATE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyDigitSubstitution(psds : *const SCRIPT_DIGITSUBSTITUTE, psc : *mut SCRIPT_CONTROL, pss : *mut SCRIPT_STATE) -> windows_core::HRESULT);
    unsafe { ScriptApplyDigitSubstitution(psds.as_ptr(), psc.as_mut_ptr(), pss.as_mut_ptr()) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: &[SCRIPT_ANALYSIS; 1], pabc: Option<&mut [super::ABC; 1]>, pijustify: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyLogicalWidth(pidx : *const i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pabc : *mut super::ABC, pijustify : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptApplyLogicalWidth(pidx, cchars, cglyphs, pwlogclust, psva, piadvance, psa.as_ptr(), pabc.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pijustify as _) }
}
#[inline]
pub unsafe fn ScriptBreak(pwcchars: *const u16, cchars: i32, psa: &[SCRIPT_ANALYSIS; 1], psla: *mut SCRIPT_LOGATTR) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptBreak(pwcchars : *const u16, cchars : i32, psa : *const SCRIPT_ANALYSIS, psla : *mut SCRIPT_LOGATTR) -> windows_core::HRESULT);
    unsafe { ScriptBreak(pwcchars, cchars, psa.as_ptr(), psla as _) }
}
#[inline]
pub unsafe fn ScriptCPtoX(icp: i32, ftrailing: bool, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: &[SCRIPT_ANALYSIS; 1]) -> windows_core::Result<i32> {
    windows_core::link!("usp10.dll" "system" fn ScriptCPtoX(icp : i32, ftrailing : windows_core::BOOL, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pix : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptCPtoX(icp, ftrailing.into(), pwlogclust.len().try_into().unwrap(), cglyphs, pwlogclust.as_ptr(), psva, piadvance, psa.as_ptr(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptCacheGetHeight(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], tmheight: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptCacheGetHeight(hdc : super::HDC, psc : *mut SCRIPT_CACHE, tmheight : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptCacheGetHeight(hdc, psc.as_mut_ptr(), tmheight.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptFreeCache(psc: &mut [SCRIPT_CACHE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptFreeCache(psc : *mut SCRIPT_CACHE) -> windows_core::HRESULT);
    unsafe { ScriptFreeCache(psc.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetCMap(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], pwcinchars: *const u16, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetCMap(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwcinchars : *const u16, cchars : i32, dwflags : u32, pwoutglyphs : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptGetCMap(hdc, psc.as_mut_ptr(), pwcinchars, cchars, dwflags, pwoutglyphs as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontAlternateGlyphs(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, wglyphid: u16, palternateglyphs: &mut [u16], pcalternates: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontAlternateGlyphs(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, wglyphid : u16, cmaxalternates : i32, palternateglyphs : *mut u16, pcalternates : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontAlternateGlyphs(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, wglyphid, palternateglyphs.len().try_into().unwrap(), palternateglyphs.as_mut_ptr(), pcalternates as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontFeatureTags(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, pfeaturetags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontFeatureTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, cmaxtags : i32, pfeaturetags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontFeatureTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, pfeaturetags.len().try_into().unwrap(), pfeaturetags.as_mut_ptr(), pctags as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontLanguageTags(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, plangsystags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontLanguageTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, cmaxtags : i32, plangsystags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontLanguageTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, plangsystags.len().try_into().unwrap(), plangsystags.as_mut_ptr(), pctags as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontProperties(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], sfp: &mut [SCRIPT_FONTPROPERTIES; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontProperties(hdc : super::HDC, psc : *mut SCRIPT_CACHE, sfp : *mut SCRIPT_FONTPROPERTIES) -> windows_core::HRESULT);
    unsafe { ScriptGetFontProperties(hdc, psc.as_mut_ptr(), sfp.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontScriptTags(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, pscripttags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontScriptTags(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, cmaxtags : i32, pscripttags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontScriptTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, pscripttags.len().try_into().unwrap(), pscripttags.as_mut_ptr(), pctags as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ScriptGetGlyphABCWidth(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], wglyph: u16, pabc: &mut [super::ABC; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetGlyphABCWidth(hdc : super::HDC, psc : *mut SCRIPT_CACHE, wglyph : u16, pabc : *mut super::ABC) -> windows_core::HRESULT);
    unsafe { ScriptGetGlyphABCWidth(hdc, psc.as_mut_ptr(), wglyph, pabc.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptGetLogicalWidths(psa: &[SCRIPT_ANALYSIS; 1], cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetLogicalWidths(psa : *const SCRIPT_ANALYSIS, cchars : i32, cglyphs : i32, piglyphwidth : *const i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, pidx : *const i32) -> windows_core::HRESULT);
    unsafe { ScriptGetLogicalWidths(psa.as_ptr(), cchars, cglyphs, piglyphwidth, pwlogclust, psva, pidx) }
}
#[inline]
pub unsafe fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetProperties(ppsp : *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetProperties(ppsp as _, pinumscripts as _) }
}
#[inline]
pub unsafe fn ScriptIsComplex(pwcinchars: &[u16], dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptIsComplex(pwcinchars : *const u16, cinchars : i32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { ScriptIsComplex(pwcinchars.as_ptr(), pwcinchars.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn ScriptItemize(pwcinchars: &[u16], pscontrol: Option<&[SCRIPT_CONTROL; 1]>, psstate: Option<&[SCRIPT_STATE; 1]>, pitems: &mut [SCRIPT_ITEM], pcitems: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptItemize(pwcinchars : *const u16, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pcitems : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptItemize(pwcinchars.as_ptr(), pwcinchars.len().try_into().unwrap(), pitems.len().try_into().unwrap(), pscontrol.map_or(core::ptr::null(), |slice| slice.as_ptr()), psstate.map_or(core::ptr::null(), |slice| slice.as_ptr()), pitems.as_mut_ptr(), pcitems.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptItemizeOpenType(pwcinchars: &[u16], cmaxitems: i32, pscontrol: Option<*const SCRIPT_CONTROL>, psstate: Option<*const SCRIPT_STATE>, pitems: *mut SCRIPT_ITEM, pscripttags: *mut OPENTYPE_TAG, pcitems: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptItemizeOpenType(pwcinchars : *const u16, cinchars : i32, cmaxitems : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pitems : *mut SCRIPT_ITEM, pscripttags : *mut OPENTYPE_TAG, pcitems : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptItemizeOpenType(pwcinchars.as_ptr(), pwcinchars.len().try_into().unwrap(), cmaxitems, pscontrol.unwrap_or(core::mem::zeroed()) as _, psstate.unwrap_or(core::mem::zeroed()) as _, pitems as _, pscripttags as _, pcitems as _) }
}
#[inline]
pub unsafe fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptJustify(psva : *const SCRIPT_VISATTR, piadvance : *const i32, cglyphs : i32, idx : i32, iminkashida : i32, pijustify : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptJustify(psva, piadvance, cglyphs, idx, iminkashida, pijustify as _) }
}
#[inline]
pub unsafe fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: Option<*mut i32>, pilogicaltovisual: Option<*mut i32>) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptLayout(cruns : i32, pblevel : *const u8, pivisualtological : *mut i32, pilogicaltovisual : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptLayout(cruns, pblevel, pivisualtological.unwrap_or(core::mem::zeroed()) as _, pilogicaltovisual.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ScriptPlace(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: &mut [SCRIPT_ANALYSIS; 1], piadvance: *mut i32, pgoffset: Option<*mut GOFFSET>, pabc: &mut [super::ABC; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPlace(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwglyphs : *const u16, cglyphs : i32, psva : *const SCRIPT_VISATTR, psa : *mut SCRIPT_ANALYSIS, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlace(hdc, psc.as_mut_ptr(), pwglyphs, cglyphs, psva, psa.as_mut_ptr(), piadvance as _, pgoffset.unwrap_or(core::mem::zeroed()) as _, pabc.as_mut_ptr()) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ScriptPlaceOpenType(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: *const u16, pwlogclust: *const u16, pcharprops: *const SCRIPT_CHARPROP, cchars: i32, pwglyphs: *const u16, pglyphprops: *const SCRIPT_GLYPHPROP, cglyphs: i32, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: Option<*mut super::ABC>) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPlaceOpenType(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, pwlogclust : *const u16, pcharprops : *const SCRIPT_CHARPROP, cchars : i32, pwglyphs : *const u16, pglyphprops : *const SCRIPT_GLYPHPROP, cglyphs : i32, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlaceOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars, pwlogclust, pcharprops, cchars, pwglyphs, pglyphprops, cglyphs, piadvance as _, pgoffset as _, pabc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptPositionSingleGlyph(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPositionSingleGlyph(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, iadvance : i32, goffset : GOFFSET, pioutadvance : *mut i32, poutgoffset : *mut GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptPositionSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, iadvance, goffset, pioutadvance as _, poutgoffset as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ScriptRecordDigitSubstitution(locale: super::LCID, psds: &mut [SCRIPT_DIGITSUBSTITUTE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptRecordDigitSubstitution(locale : super::LCID, psds : *mut SCRIPT_DIGITSUBSTITUTE) -> windows_core::HRESULT);
    unsafe { ScriptRecordDigitSubstitution(locale, psds.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptShape(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], pwcchars: *const u16, cchars: i32, cmaxglyphs: i32, psa: &mut [SCRIPT_ANALYSIS; 1], pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptShape(hdc : super::HDC, psc : *mut SCRIPT_CACHE, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, psa : *mut SCRIPT_ANALYSIS, pwoutglyphs : *mut u16, pwlogclust : *mut u16, psva : *mut SCRIPT_VISATTR, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShape(hdc, psc.as_mut_ptr(), pwcchars, cchars, cmaxglyphs, psa.as_mut_ptr(), pwoutglyphs as _, pwlogclust as _, psva as _, pcglyphs.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptShapeOpenType(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: *const u16, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut SCRIPT_CHARPROP, pwoutglyphs: *mut u16, poutglyphprops: *mut SCRIPT_GLYPHPROP, pcglyphs: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptShapeOpenType(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, pwlogclust : *mut u16, pcharprops : *mut SCRIPT_CHARPROP, pwoutglyphs : *mut u16, poutglyphprops : *mut SCRIPT_GLYPHPROP, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShapeOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars, cchars, cmaxglyphs, pwlogclust as _, pcharprops as _, pwoutglyphs as _, poutglyphprops as _, pcglyphs as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptStringAnalyse(hdc: super::HDC, pstring: *const core::ffi::c_void, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: Option<&[SCRIPT_CONTROL; 1]>, psstate: Option<&[SCRIPT_STATE; 1]>, pidx: Option<&[i32]>, ptabdef: Option<&[SCRIPT_TABDEF; 1]>, pbinclass: *const u8) -> windows_core::Result<SCRIPT_STRING_ANALYSIS> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringAnalyse(hdc : super::HDC, pstring : *const core::ffi::c_void, cstring : i32, cglyphs : i32, icharset : i32, dwflags : u32, ireqwidth : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pidx : *const i32, ptabdef : *const SCRIPT_TABDEF, pbinclass : *const u8, pssa : *mut SCRIPT_STRING_ANALYSIS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptStringAnalyse(hdc, pstring, pidx.map_or(0, |slice| slice.len().try_into().unwrap()), cglyphs, icharset, dwflags, ireqwidth, pscontrol.map_or(core::ptr::null(), |slice| slice.as_ptr()), psstate.map_or(core::ptr::null(), |slice| slice.as_ptr()), pidx.map_or(core::ptr::null(), |slice| slice.as_ptr()), ptabdef.map_or(core::ptr::null(), |slice| slice.as_ptr()), pbinclass, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ScriptStringCPtoX(ssa: &[SCRIPT_STRING_ANALYSIS; 1], icp: i32, ftrailing: bool, px: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringCPtoX(ssa : SCRIPT_STRING_ANALYSIS, icp : i32, ftrailing : windows_core::BOOL, px : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptStringCPtoX(core::mem::transmute(ssa.as_ptr()), icp, ftrailing.into(), px.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptStringFree(pssa: &mut [SCRIPT_STRING_ANALYSIS; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringFree(pssa : *mut SCRIPT_STRING_ANALYSIS) -> windows_core::HRESULT);
    unsafe { ScriptStringFree(pssa.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptStringGetLogicalWidths(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> windows_core::Result<i32> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringGetLogicalWidths(ssa : SCRIPT_STRING_ANALYSIS, pidx : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptStringGetLogicalWidths(core::mem::transmute(ssa.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ScriptStringGetOrder(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> windows_core::Result<u32> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringGetOrder(ssa : SCRIPT_STRING_ANALYSIS, puorder : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ScriptStringGetOrder(core::mem::transmute(ssa.as_ptr()), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptStringOut(ssa: &[SCRIPT_STRING_ANALYSIS; 1], ix: i32, iy: i32, uoptions: u32, prc: Option<&[super::RECT; 1]>, iminsel: i32, imaxsel: i32, fdisabled: bool) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringOut(ssa : SCRIPT_STRING_ANALYSIS, ix : i32, iy : i32, uoptions : u32, prc : *const super::RECT, iminsel : i32, imaxsel : i32, fdisabled : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { ScriptStringOut(core::mem::transmute(ssa.as_ptr()), ix, iy, uoptions, prc.map_or(core::ptr::null(), |slice| slice.as_ptr()), iminsel, imaxsel, fdisabled.into()) }
}
#[inline]
pub unsafe fn ScriptStringValidate(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringValidate(ssa : SCRIPT_STRING_ANALYSIS) -> windows_core::HRESULT);
    unsafe { ScriptStringValidate(core::mem::transmute(ssa.as_ptr())) }
}
#[inline]
pub unsafe fn ScriptStringXtoCP(ssa: &[SCRIPT_STRING_ANALYSIS; 1], ix: i32, pich: &mut [i32; 1], pitrailing: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringXtoCP(ssa : SCRIPT_STRING_ANALYSIS, ix : i32, pich : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptStringXtoCP(core::mem::transmute(ssa.as_ptr()), ix, pich.as_mut_ptr(), pitrailing.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptString_pLogAttr(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> *const SCRIPT_LOGATTR {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pLogAttr(ssa : SCRIPT_STRING_ANALYSIS) -> *const SCRIPT_LOGATTR);
    unsafe { ScriptString_pLogAttr(core::mem::transmute(ssa.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptString_pSize(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> *const super::SIZE {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pSize(ssa : SCRIPT_STRING_ANALYSIS) -> *const super::SIZE);
    unsafe { ScriptString_pSize(core::mem::transmute(ssa.as_ptr())) }
}
#[inline]
pub unsafe fn ScriptString_pcOutChars(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> *const i32 {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pcOutChars(ssa : SCRIPT_STRING_ANALYSIS) -> *const i32);
    unsafe { ScriptString_pcOutChars(core::mem::transmute(ssa.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptSubstituteSingleGlyph(hdc: Option<super::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptSubstituteSingleGlyph(hdc : super::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, pwoutglyphid : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptSubstituteSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, pwoutglyphid as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptTextOut(hdc: super::HDC, psc: &mut [SCRIPT_CACHE; 1], x: i32, y: i32, fuoptions: u32, lprc: Option<&[super::RECT; 1]>, psa: &[SCRIPT_ANALYSIS; 1], pwcreserved: Option<*const u16>, ireserved: Option<i32>, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: Option<*const i32>, pgoffset: *const GOFFSET) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptTextOut(hdc : super::HDC, psc : *mut SCRIPT_CACHE, x : i32, y : i32, fuoptions : u32, lprc : *const super::RECT, psa : *const SCRIPT_ANALYSIS, pwcreserved : *const u16, ireserved : i32, pwglyphs : *const u16, cglyphs : i32, piadvance : *const i32, pijustify : *const i32, pgoffset : *const GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptTextOut(hdc, psc.as_mut_ptr(), x, y, fuoptions, lprc.map_or(core::ptr::null(), |slice| slice.as_ptr()), psa.as_ptr(), pwcreserved.unwrap_or(core::mem::zeroed()) as _, ireserved.unwrap_or(core::mem::zeroed()) as _, pwglyphs, cglyphs, piadvance, pijustify.unwrap_or(core::mem::zeroed()) as _, pgoffset) }
}
#[inline]
pub unsafe fn ScriptXtoCP(ix: i32, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: &[SCRIPT_ANALYSIS; 1], picp: &mut [i32; 1], pitrailing: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptXtoCP(ix : i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, picp : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptXtoCP(ix, pwlogclust.len().try_into().unwrap(), cglyphs, pwlogclust.as_ptr(), psva, piadvance, psa.as_ptr(), picp.as_mut_ptr(), pitrailing.as_mut_ptr()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: OPENTYPE_TAG,
    pub lParameter: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OPENTYPE_TAG(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
}
impl SCRIPT_ANALYSIS {
    pub fn eScript(&self) -> u16 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_eScript(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !1023) | (value & 1023);
    }
    pub fn fRTL(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_fRTL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn fLayoutRTL(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_fLayoutRTL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn fLinkBefore(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_fLinkBefore(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn fLinkAfter(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fLinkAfter(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn fLogicalOrder(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_fLogicalOrder(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fNoGlyphIndex(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fNoGlyphIndex(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SCRIPT_CACHE(pub *mut core::ffi::c_void);
impl Default for SCRIPT_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_CHARPROP {
    pub _bitfield: u16,
}
impl SCRIPT_CHARPROP {
    pub fn fCanGlyphAlone(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_fCanGlyphAlone(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
impl SCRIPT_CONTROL {
    pub fn uDefaultLanguage(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_uDefaultLanguage(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn fContextDigits(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_fContextDigits(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn fInvertPreBoundDir(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_fInvertPreBoundDir(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u32) << 17);
    }
    pub fn fInvertPostBoundDir(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_fInvertPostBoundDir(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u32) << 18);
    }
    pub fn fLinkStringBefore(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_fLinkStringBefore(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u32) << 19);
    }
    pub fn fLinkStringAfter(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_fLinkStringAfter(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u32) << 20);
    }
    pub fn fNeutralOverride(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_fNeutralOverride(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u32) << 21);
    }
    pub fn fNumericOverride(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_fNumericOverride(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u32) << 22);
    }
    pub fn fLegacyBidiClass(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_fLegacyBidiClass(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn fMergeNeutralItems(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_fMergeNeutralItems(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u32) << 24);
    }
    pub fn fUseStandardBidi(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_fUseStandardBidi(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u32) << 25);
    }
    pub fn fReserved(&self) -> u32 {
        self._bitfield >> 26
    }
    pub fn set_fReserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_DIGITSUBSTITUTE {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub dwReserved: u32,
}
impl SCRIPT_DIGITSUBSTITUTE {
    pub fn NationalDigitLanguage(&self) -> u32 {
        (self._bitfield1 << 16) >> 16
    }
    pub fn set_NationalDigitLanguage(&mut self, value: u32) {
        self._bitfield1 = (self._bitfield1 & !65535) | (value & 65535);
    }
    pub fn TraditionalDigitLanguage(&self) -> u32 {
        self._bitfield1 >> 16
    }
    pub fn set_TraditionalDigitLanguage(&mut self, value: u32) {
        self._bitfield1 = (self._bitfield1 & !(65535 << 16)) | ((value & 65535) << 16);
    }
    pub fn DigitSubstitute(&self) -> u32 {
        (self._bitfield2 << 24) >> 24
    }
    pub fn set_DigitSubstitute(&mut self, value: u32) {
        self._bitfield2 = (self._bitfield2 & !255) | (value & 255);
    }
}
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
impl SCRIPT_LOGATTR {
    pub fn fSoftBreak(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_fSoftBreak(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn fWhiteSpace(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_fWhiteSpace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn fCharStop(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_fCharStop(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn fWordStop(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_fWordStop(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn fInvalid(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_fInvalid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn fReserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_fReserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
impl SCRIPT_PROPERTIES {
    pub fn langid(&self) -> u32 {
        (self._bitfield1 << 16) >> 16
    }
    pub fn set_langid(&mut self, value: u32) {
        self._bitfield1 = (self._bitfield1 & !65535) | (value & 65535);
    }
    pub fn fNumeric(&self) -> bool {
        (self._bitfield1 >> 16) & 1 != 0
    }
    pub fn set_fNumeric(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn fComplex(&self) -> bool {
        (self._bitfield1 >> 17) & 1 != 0
    }
    pub fn set_fComplex(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 17)) | ((value as u32) << 17);
    }
    pub fn fNeedsWordBreaking(&self) -> bool {
        (self._bitfield1 >> 18) & 1 != 0
    }
    pub fn set_fNeedsWordBreaking(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 18)) | ((value as u32) << 18);
    }
    pub fn fNeedsCaretInfo(&self) -> bool {
        (self._bitfield1 >> 19) & 1 != 0
    }
    pub fn set_fNeedsCaretInfo(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 19)) | ((value as u32) << 19);
    }
    pub fn bCharSet(&self) -> u32 {
        (self._bitfield1 << 4) >> 24
    }
    pub fn set_bCharSet(&mut self, value: u32) {
        self._bitfield1 = (self._bitfield1 & !(255 << 20)) | ((value & 255) << 20);
    }
    pub fn fControl(&self) -> bool {
        (self._bitfield1 >> 28) & 1 != 0
    }
    pub fn set_fControl(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 28)) | ((value as u32) << 28);
    }
    pub fn fPrivateUseArea(&self) -> bool {
        (self._bitfield1 >> 29) & 1 != 0
    }
    pub fn set_fPrivateUseArea(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 29)) | ((value as u32) << 29);
    }
    pub fn fNeedsCharacterJustify(&self) -> bool {
        (self._bitfield1 >> 30) & 1 != 0
    }
    pub fn set_fNeedsCharacterJustify(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 30)) | ((value as u32) << 30);
    }
    pub fn fInvalidGlyph(&self) -> bool {
        (self._bitfield1 >> 31) & 1 != 0
    }
    pub fn set_fInvalidGlyph(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 31)) | ((value as u32) << 31);
    }
    pub fn fInvalidLogAttr(&self) -> bool {
        self._bitfield2 & 1 != 0
    }
    pub fn set_fInvalidLogAttr(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !1) | (value as u32);
    }
    pub fn fCDM(&self) -> bool {
        (self._bitfield2 >> 1) & 1 != 0
    }
    pub fn set_fCDM(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn fAmbiguousCharSet(&self) -> bool {
        (self._bitfield2 >> 2) & 1 != 0
    }
    pub fn set_fAmbiguousCharSet(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn fClusterSizeVaries(&self) -> bool {
        (self._bitfield2 >> 3) & 1 != 0
    }
    pub fn set_fClusterSizeVaries(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn fRejectInvalid(&self) -> bool {
        (self._bitfield2 >> 4) & 1 != 0
    }
    pub fn set_fRejectInvalid(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 4)) | ((value as u32) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
}
impl SCRIPT_STATE {
    pub fn uBidiLevel(&self) -> u16 {
        (self._bitfield << 11) >> 11
    }
    pub fn set_uBidiLevel(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn fOverrideDirection(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_fOverrideDirection(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn fInhibitSymSwap(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_fInhibitSymSwap(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn fCharShape(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_fCharShape(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn fDigitSubstitute(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_fDigitSubstitute(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn fInhibitLigate(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_fInhibitLigate(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn fDisplayZWG(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_fDisplayZWG(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn fArabicNumContext(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_fArabicNumContext(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn fGcpClusters(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_fGcpClusters(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn fReserved(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fReserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn fEngineReserved(&self) -> u16 {
        self._bitfield >> 14
    }
    pub fn set_fEngineReserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SCRIPT_STRING_ANALYSIS(pub *mut core::ffi::c_void);
impl Default for SCRIPT_STRING_ANALYSIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_VISATTR {
    pub _bitfield: u16,
}
impl SCRIPT_VISATTR {
    pub fn uJustification(&self) -> u16 {
        (self._bitfield << 12) >> 12
    }
    pub fn set_uJustification(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn fClusterStart(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_fClusterStart(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn fDiacritic(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_fDiacritic(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn fZeroWidth(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_fZeroWidth(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn fReserved(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_fReserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn fShapeReserved(&self) -> u16 {
        self._bitfield >> 8
    }
    pub fn set_fShapeReserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
