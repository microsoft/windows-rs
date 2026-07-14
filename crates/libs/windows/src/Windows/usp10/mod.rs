#[inline]
pub unsafe fn ScriptApplyDigitSubstitution(psds: &[SCRIPT_DIGITSUBSTITUTE; 1], psc: &mut [SCRIPT_CONTROL; 1], pss: &mut [SCRIPT_STATE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyDigitSubstitution(psds : *const SCRIPT_DIGITSUBSTITUTE, psc : *mut SCRIPT_CONTROL, pss : *mut SCRIPT_STATE) -> windows_core::HRESULT);
    unsafe { ScriptApplyDigitSubstitution(psds.as_ptr(), psc.as_mut_ptr(), pss.as_mut_ptr()) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: &[SCRIPT_ANALYSIS; 1], pabc: Option<&mut [super::wingdi::ABC; 1]>, pijustify: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptApplyLogicalWidth(pidx : *const i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, pabc : *mut super::wingdi::ABC, pijustify : *mut i32) -> windows_core::HRESULT);
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
pub unsafe fn ScriptCacheGetHeight(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], tmheight: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptCacheGetHeight(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, tmheight : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptCacheGetHeight(hdc, psc.as_mut_ptr(), tmheight.as_mut_ptr()) }
}
#[inline]
pub unsafe fn ScriptFreeCache(psc: &mut [SCRIPT_CACHE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptFreeCache(psc : *mut SCRIPT_CACHE) -> windows_core::HRESULT);
    unsafe { ScriptFreeCache(psc.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetCMap(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], pwcinchars: *const u16, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetCMap(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, pwcinchars : *const u16, cchars : i32, dwflags : u32, pwoutglyphs : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptGetCMap(hdc, psc.as_mut_ptr(), pwcinchars, cchars, dwflags, pwoutglyphs as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontAlternateGlyphs(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, wglyphid: u16, palternateglyphs: &mut [u16], pcalternates: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontAlternateGlyphs(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, wglyphid : u16, cmaxalternates : i32, palternateglyphs : *mut u16, pcalternates : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontAlternateGlyphs(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, wglyphid, palternateglyphs.len().try_into().unwrap(), palternateglyphs.as_mut_ptr(), pcalternates as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontFeatureTags(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, pfeaturetags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontFeatureTags(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, cmaxtags : i32, pfeaturetags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontFeatureTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, pfeaturetags.len().try_into().unwrap(), pfeaturetags.as_mut_ptr(), pctags as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontLanguageTags(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, plangsystags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontLanguageTags(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, cmaxtags : i32, plangsystags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontLanguageTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, plangsystags.len().try_into().unwrap(), plangsystags.as_mut_ptr(), pctags as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontProperties(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], sfp: &mut [SCRIPT_FONTPROPERTIES; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontProperties(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, sfp : *mut SCRIPT_FONTPROPERTIES) -> windows_core::HRESULT);
    unsafe { ScriptGetFontProperties(hdc, psc.as_mut_ptr(), sfp.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptGetFontScriptTags(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, pscripttags: &mut [OPENTYPE_TAG], pctags: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetFontScriptTags(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, cmaxtags : i32, pscripttags : *mut OPENTYPE_TAG, pctags : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptGetFontScriptTags(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, pscripttags.len().try_into().unwrap(), pscripttags.as_mut_ptr(), pctags as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ScriptGetGlyphABCWidth(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], wglyph: u16, pabc: &mut [super::wingdi::ABC; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptGetGlyphABCWidth(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, wglyph : u16, pabc : *mut super::wingdi::ABC) -> windows_core::HRESULT);
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
pub unsafe fn ScriptPlace(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: &mut [SCRIPT_ANALYSIS; 1], piadvance: *mut i32, pgoffset: Option<*mut GOFFSET>, pabc: &mut [super::wingdi::ABC; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPlace(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, pwglyphs : *const u16, cglyphs : i32, psva : *const SCRIPT_VISATTR, psa : *mut SCRIPT_ANALYSIS, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::wingdi::ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlace(hdc, psc.as_mut_ptr(), pwglyphs, cglyphs, psva, psa.as_mut_ptr(), piadvance as _, pgoffset.unwrap_or(core::mem::zeroed()) as _, pabc.as_mut_ptr()) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ScriptPlaceOpenType(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: *const u16, pwlogclust: *const u16, pcharprops: *const SCRIPT_CHARPROP, cchars: i32, pwglyphs: *const u16, pglyphprops: *const SCRIPT_GLYPHPROP, cglyphs: i32, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: Option<*mut super::wingdi::ABC>) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPlaceOpenType(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, pwlogclust : *const u16, pcharprops : *const SCRIPT_CHARPROP, cchars : i32, pwglyphs : *const u16, pglyphprops : *const SCRIPT_GLYPHPROP, cglyphs : i32, piadvance : *mut i32, pgoffset : *mut GOFFSET, pabc : *mut super::wingdi::ABC) -> windows_core::HRESULT);
    unsafe { ScriptPlaceOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars, pwlogclust, pcharprops, cchars, pwglyphs, pglyphprops, cglyphs, piadvance as _, pgoffset as _, pabc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptPositionSingleGlyph(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptPositionSingleGlyph(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, iadvance : i32, goffset : GOFFSET, pioutadvance : *mut i32, poutgoffset : *mut GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptPositionSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, iadvance, core::mem::transmute(goffset), pioutadvance as _, poutgoffset as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ScriptRecordDigitSubstitution(locale: super::winnt::LCID, psds: &mut [SCRIPT_DIGITSUBSTITUTE; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptRecordDigitSubstitution(locale : super::winnt::LCID, psds : *mut SCRIPT_DIGITSUBSTITUTE) -> windows_core::HRESULT);
    unsafe { ScriptRecordDigitSubstitution(locale, psds.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptShape(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], pwcchars: *const u16, cchars: i32, cmaxglyphs: i32, psa: &mut [SCRIPT_ANALYSIS; 1], pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptShape(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, psa : *mut SCRIPT_ANALYSIS, pwoutglyphs : *mut u16, pwlogclust : *mut u16, psva : *mut SCRIPT_VISATTR, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShape(hdc, psc.as_mut_ptr(), pwcchars, cchars, cmaxglyphs, psa.as_mut_ptr(), pwoutglyphs as _, pwlogclust as _, psva as _, pcglyphs.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptShapeOpenType(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: *mut SCRIPT_ANALYSIS, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, rcrangechars: Option<*const i32>, rprangeproperties: Option<*const *const TEXTRANGE_PROPERTIES>, cranges: i32, pwcchars: *const u16, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut SCRIPT_CHARPROP, pwoutglyphs: *mut u16, poutglyphprops: *mut SCRIPT_GLYPHPROP, pcglyphs: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptShapeOpenType(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *mut SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, rcrangechars : *const i32, rprangeproperties : *const *const TEXTRANGE_PROPERTIES, cranges : i32, pwcchars : *const u16, cchars : i32, cmaxglyphs : i32, pwlogclust : *mut u16, pcharprops : *mut SCRIPT_CHARPROP, pwoutglyphs : *mut u16, poutglyphprops : *mut SCRIPT_GLYPHPROP, pcglyphs : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptShapeOpenType(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa as _, tagscript, taglangsys, rcrangechars.unwrap_or(core::mem::zeroed()) as _, rprangeproperties.unwrap_or(core::mem::zeroed()) as _, cranges, pwcchars, cchars, cmaxglyphs, pwlogclust as _, pcharprops as _, pwoutglyphs as _, poutglyphprops as _, pcglyphs as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptStringAnalyse(hdc: super::windef::HDC, pstring: *const core::ffi::c_void, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: Option<&[SCRIPT_CONTROL; 1]>, psstate: Option<&[SCRIPT_STATE; 1]>, pidx: Option<&[i32]>, ptabdef: Option<&[SCRIPT_TABDEF; 1]>, pbinclass: *const u8) -> windows_core::Result<SCRIPT_STRING_ANALYSIS> {
    windows_core::link!("usp10.dll" "system" fn ScriptStringAnalyse(hdc : super::windef::HDC, pstring : *const core::ffi::c_void, cstring : i32, cglyphs : i32, icharset : i32, dwflags : u32, ireqwidth : i32, pscontrol : *const SCRIPT_CONTROL, psstate : *const SCRIPT_STATE, pidx : *const i32, ptabdef : *const SCRIPT_TABDEF, pbinclass : *const u8, pssa : *mut SCRIPT_STRING_ANALYSIS) -> windows_core::HRESULT);
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
pub unsafe fn ScriptStringOut(ssa: &[SCRIPT_STRING_ANALYSIS; 1], ix: i32, iy: i32, uoptions: u32, prc: Option<&[super::windef::RECT; 1]>, iminsel: i32, imaxsel: i32, fdisabled: bool) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptStringOut(ssa : SCRIPT_STRING_ANALYSIS, ix : i32, iy : i32, uoptions : u32, prc : *const super::windef::RECT, iminsel : i32, imaxsel : i32, fdisabled : windows_core::BOOL) -> windows_core::HRESULT);
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
pub unsafe fn ScriptString_pSize(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> *const super::windef::SIZE {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pSize(ssa : SCRIPT_STRING_ANALYSIS) -> *const super::windef::SIZE);
    unsafe { ScriptString_pSize(core::mem::transmute(ssa.as_ptr())) }
}
#[inline]
pub unsafe fn ScriptString_pcOutChars(ssa: &[SCRIPT_STRING_ANALYSIS; 1]) -> *const i32 {
    windows_core::link!("usp10.dll" "system" fn ScriptString_pcOutChars(ssa : SCRIPT_STRING_ANALYSIS) -> *const i32);
    unsafe { ScriptString_pcOutChars(core::mem::transmute(ssa.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptSubstituteSingleGlyph(hdc: Option<super::windef::HDC>, psc: *mut SCRIPT_CACHE, psa: Option<*const SCRIPT_ANALYSIS>, tagscript: OPENTYPE_TAG, taglangsys: OPENTYPE_TAG, tagfeature: OPENTYPE_TAG, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptSubstituteSingleGlyph(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, psa : *const SCRIPT_ANALYSIS, tagscript : OPENTYPE_TAG, taglangsys : OPENTYPE_TAG, tagfeature : OPENTYPE_TAG, lparameter : i32, wglyphid : u16, pwoutglyphid : *mut u16) -> windows_core::HRESULT);
    unsafe { ScriptSubstituteSingleGlyph(hdc.unwrap_or(core::mem::zeroed()) as _, psc as _, psa.unwrap_or(core::mem::zeroed()) as _, tagscript, taglangsys, tagfeature, lparameter, wglyphid, pwoutglyphid as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScriptTextOut(hdc: super::windef::HDC, psc: &mut [SCRIPT_CACHE; 1], x: i32, y: i32, fuoptions: u32, lprc: Option<&[super::windef::RECT; 1]>, psa: &[SCRIPT_ANALYSIS; 1], pwcreserved: Option<*const u16>, ireserved: Option<i32>, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: Option<*const i32>, pgoffset: *const GOFFSET) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptTextOut(hdc : super::windef::HDC, psc : *mut SCRIPT_CACHE, x : i32, y : i32, fuoptions : u32, lprc : *const super::windef::RECT, psa : *const SCRIPT_ANALYSIS, pwcreserved : *const u16, ireserved : i32, pwglyphs : *const u16, cglyphs : i32, piadvance : *const i32, pijustify : *const i32, pgoffset : *const GOFFSET) -> windows_core::HRESULT);
    unsafe { ScriptTextOut(hdc, psc.as_mut_ptr(), x, y, fuoptions, lprc.map_or(core::ptr::null(), |slice| slice.as_ptr()), psa.as_ptr(), pwcreserved.unwrap_or(core::mem::zeroed()) as _, ireserved.unwrap_or(core::mem::zeroed()) as _, pwglyphs, cglyphs, piadvance, pijustify.unwrap_or(core::mem::zeroed()) as _, pgoffset) }
}
#[inline]
pub unsafe fn ScriptXtoCP(ix: i32, cglyphs: i32, pwlogclust: &[u16], psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: &[SCRIPT_ANALYSIS; 1], picp: &mut [i32; 1], pitrailing: &mut [i32; 1]) -> windows_core::HRESULT {
    windows_core::link!("usp10.dll" "system" fn ScriptXtoCP(ix : i32, cchars : i32, cglyphs : i32, pwlogclust : *const u16, psva : *const SCRIPT_VISATTR, piadvance : *const i32, psa : *const SCRIPT_ANALYSIS, picp : *mut i32, pitrailing : *mut i32) -> windows_core::HRESULT);
    unsafe { ScriptXtoCP(ix, pwlogclust.len().try_into().unwrap(), cglyphs, pwlogclust.as_ptr(), psva, piadvance, psa.as_ptr(), picp.as_mut_ptr(), pitrailing.as_mut_ptr()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: OPENTYPE_TAG,
    pub lParameter: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OPENTYPE_TAG(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_CHARPROP {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
