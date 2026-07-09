#[cfg(feature = "Win32_dxgitype")]
pub type DWRITE_COLOR_F = super::dxgitype::D3DCOLORVALUE;
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
#[derive(Clone, Debug, PartialEq)]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: super::dwrite::DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl Default for DWRITE_COLOR_GLYPH_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DWRITE_GRID_FIT_MODE = i32;
pub const DWRITE_GRID_FIT_MODE_DEFAULT: DWRITE_GRID_FIT_MODE = 0;
pub const DWRITE_GRID_FIT_MODE_DISABLED: DWRITE_GRID_FIT_MODE = 1;
pub const DWRITE_GRID_FIT_MODE_ENABLED: DWRITE_GRID_FIT_MODE = 2;
pub const DWRITE_NO_PALETTE_INDEX: u32 = 65535;
pub type DWRITE_OPTICAL_ALIGNMENT = i32;
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: DWRITE_OPTICAL_ALIGNMENT = 0;
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: DWRITE_OPTICAL_ALIGNMENT = 1;
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: super::dwrite::DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
windows_core::imp::define_interface!(IDWriteColorGlyphRunEnumerator, IDWriteColorGlyphRunEnumerator_Vtbl, 0xd31fbe17_f157_41a2_8d24_cb779e0560e8);
windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator, windows_core::IUnknown);
impl IDWriteColorGlyphRunEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
    pub unsafe fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteColorGlyphRunEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
    pub GetCurrentRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DWRITE_COLOR_GLYPH_RUN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dxgitype")))]
    GetCurrentRun: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait IDWriteColorGlyphRunEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl IDWriteColorGlyphRunEnumerator_Vtbl {
    pub const fn new<Identity: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasrun: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteColorGlyphRunEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasrun.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentRun<Identity: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteColorGlyphRunEnumerator_Impl::GetCurrentRun(this) {
                    Ok(ok__) => {
                        colorglyphrun.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            GetCurrentRun: GetCurrentRun::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for IDWriteColorGlyphRunEnumerator {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteFactory2, IDWriteFactory2_Vtbl, 0x0439fc60_ca44_4994_8dee_3a9af7b732ec);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteFactory2 {
    type Target = super::dwrite_1::IDWriteFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory2, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteFactory2 {
    pub unsafe fn GetSystemFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> windows_core::Result<IDWriteFontFallbackBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFallbackBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldtodevicetransform: Option<*const super::dwrite::DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, measuringmode, worldtodevicetransform.unwrap_or(core::mem::zeroed()) as _, colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: Option<*const super::dwrite::DWRITE_MATRIX>, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<super::dwrite::IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphrun), transform.unwrap_or(core::mem::zeroed()) as _, renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory2_Vtbl {
    pub base__: super::dwrite_1::IDWriteFactory1_Vtbl,
    pub GetSystemFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFallbackBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    TranslateColorGlyphRun: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, super::dwrite::DWRITE_PIXEL_GEOMETRY, super::dwrite::DWRITE_RENDERING_MODE, DWRITE_GRID_FIT_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_MATRIX, super::dwrite::DWRITE_RENDERING_MODE, super::dcommon::DWRITE_MEASURING_MODE, DWRITE_GRID_FIT_MODE, super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateGlyphRunAnalysis: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory2_Impl: super::dwrite_1::IDWriteFactory1_Impl {
    fn GetSystemFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
    fn CreateFontFallbackBuilder(&self) -> windows_core::Result<IDWriteFontFallbackBuilder>;
    fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldtodevicetransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams2>;
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: *const super::dwrite::DWRITE_MATRIX, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<super::dwrite::IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory2_Vtbl {
    pub const fn new<Identity: IDWriteFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemFontFallback<Identity: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory2_Impl::GetSystemFontFallback(this) {
                    Ok(ok__) => {
                        fontfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Identity: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallbackbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory2_Impl::CreateFontFallbackBuilder(this) {
                    Ok(ok__) => {
                        fontfallbackbuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldtodevicetransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory2_Impl::TranslateColorGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldtodevicetransform), core::mem::transmute_copy(&colorpaletteindex)) {
                    Ok(ok__) => {
                        colorlayers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory2_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&grayscaleenhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: *const super::dwrite::DWRITE_MATRIX, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory2_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&gridfitmode), core::mem::transmute_copy(&antialiasmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                    Ok(ok__) => {
                        glyphrunanalysis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite_1::IDWriteFactory1_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontFallback: GetSystemFontFallback::<Identity, OFFSET>,
            CreateFontFallbackBuilder: CreateFontFallbackBuilder::<Identity, OFFSET>,
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory2 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteFont2, IDWriteFont2_Vtbl, 0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteFont2 {
    type Target = super::dwrite_1::IDWriteFont1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteFont2, windows_core::IUnknown, super::dwrite::IDWriteFont, super::dwrite_1::IDWriteFont1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteFont2 {
    pub unsafe fn IsColorFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsColorFont)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont2_Vtbl {
    pub base__: super::dwrite_1::IDWriteFont1_Vtbl,
    pub IsColorFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteFont2_Impl: super::dwrite_1::IDWriteFont1_Impl {
    fn IsColorFont(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteFont2_Vtbl {
    pub const fn new<Identity: IDWriteFont2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsColorFont<Identity: IDWriteFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont2_Impl::IsColorFont(this)
            }
        }
        Self { base__: super::dwrite_1::IDWriteFont1_Vtbl::new::<Identity, OFFSET>(), IsColorFont: IsColorFont::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFont as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFont1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteFont2 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteFontFace2, IDWriteFontFace2_Vtbl, 0xd8b768ff_64bc_4e66_982b_ec8e87f693f7);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteFontFace2 {
    type Target = super::dwrite_1::IDWriteFontFace1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace2, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteFontFace2 {
    pub unsafe fn IsColorFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsColorFont)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetColorPaletteCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPaletteEntryCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dxgitype")]
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), core::mem::transmute(paletteentries.as_ptr())) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRecommendedRenderingMode<P7>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const super::dwrite::DWRITE_MATRIX>, issideways: bool, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: P7, renderingmode: *mut super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT
    where
        P7: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, transform.unwrap_or(core::mem::zeroed()) as _, issideways.into(), outlinethreshold, measuringmode, renderingparams.param().abi(), renderingmode as _, gridfitmode as _) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace2_Vtbl {
    pub base__: super::dwrite_1::IDWriteFontFace1_Vtbl,
    pub IsColorFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetColorPaletteCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPaletteEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_dxgitype")]
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut DWRITE_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgitype"))]
    GetPaletteEntries: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const super::dwrite::DWRITE_MATRIX, windows_core::BOOL, super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, super::dcommon::DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut super::dwrite::DWRITE_RENDERING_MODE, *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRecommendedRenderingMode: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dxgitype"))]
pub trait IDWriteFontFace2_Impl: super::dwrite_1::IDWriteFontFace1_Impl {
    fn IsColorFont(&self) -> windows_core::BOOL;
    fn GetColorPaletteCount(&self) -> u32;
    fn GetPaletteEntryCount(&self) -> u32;
    fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> windows_core::Result<()>;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>, renderingmode: *mut super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dxgitype"))]
impl IDWriteFontFace2_Vtbl {
    pub const fn new<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsColorFont<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace2_Impl::IsColorFont(this)
            }
        }
        unsafe extern "system" fn GetColorPaletteCount<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace2_Impl::GetColorPaletteCount(this)
            }
        }
        unsafe extern "system" fn GetPaletteEntryCount<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace2_Impl::GetPaletteEntryCount(this)
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace2_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&firstentryindex), core::mem::transmute_copy(&entrycount), core::mem::transmute_copy(&paletteentries)).into()
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut super::dwrite::DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace2_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&renderingparams), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)).into()
            }
        }
        Self {
            base__: super::dwrite_1::IDWriteFontFace1_Vtbl::new::<Identity, OFFSET>(),
            IsColorFont: IsColorFont::<Identity, OFFSET>,
            GetColorPaletteCount: GetColorPaletteCount::<Identity, OFFSET>,
            GetPaletteEntryCount: GetPaletteEntryCount::<Identity, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for IDWriteFontFace2 {}
windows_core::imp::define_interface!(IDWriteFontFallback, IDWriteFontFallback_Vtbl, 0xefa008f9_f7a1_48bf_b05c_f224713cc0ff);
windows_core::imp::interface_hierarchy!(IDWriteFontFallback, windows_core::IUnknown);
impl IDWriteFontFallback {
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn MapCharacters<P0, P3, P4>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P3, basefamilyname: P4, baseweight: super::dwrite::DWRITE_FONT_WEIGHT, basestyle: super::dwrite::DWRITE_FONT_STYLE, basestretch: super::dwrite::DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut Option<super::dwrite::IDWriteFont>, scale: *mut f32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteTextAnalysisSource>,
        P3: windows_core::Param<super::dwrite::IDWriteFontCollection>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MapCharacters)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, basefontcollection.param().abi(), basefamilyname.param().abi(), baseweight, basestyle, basestretch, mappedlength as _, core::mem::transmute(mappedfont), scale as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_dwrite")]
    pub MapCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, super::dwrite::DWRITE_FONT_WEIGHT, super::dwrite::DWRITE_FONT_STYLE, super::dwrite::DWRITE_FONT_STRETCH, *mut u32, *mut *mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    MapCharacters: usize,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontFallback_Impl: windows_core::IUnknownImpl {
    fn MapCharacters(&self, analysissource: windows_core::Ref<super::dwrite::IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: windows_core::Ref<super::dwrite::IDWriteFontCollection>, basefamilyname: &windows_core::PCWSTR, baseweight: super::dwrite::DWRITE_FONT_WEIGHT, basestyle: super::dwrite::DWRITE_FONT_STYLE, basestretch: super::dwrite::DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: windows_core::OutRef<super::dwrite::IDWriteFont>, scale: *mut f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontFallback_Vtbl {
    pub const fn new<Identity: IDWriteFontFallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapCharacters<Identity: IDWriteFontFallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut core::ffi::c_void, basefamilyname: windows_core::PCWSTR, baseweight: super::dwrite::DWRITE_FONT_WEIGHT, basestyle: super::dwrite::DWRITE_FONT_STYLE, basestretch: super::dwrite::DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut core::ffi::c_void, scale: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFallback_Impl::MapCharacters(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&basefontcollection), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&baseweight), core::mem::transmute_copy(&basestyle), core::mem::transmute_copy(&basestretch), core::mem::transmute_copy(&mappedlength), core::mem::transmute_copy(&mappedfont), core::mem::transmute_copy(&scale)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MapCharacters: MapCharacters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontFallback {}
windows_core::imp::define_interface!(IDWriteFontFallbackBuilder, IDWriteFontFallbackBuilder_Vtbl, 0xfd882d06_8aba_4fb8_b849_8be8b73e14de);
windows_core::imp::interface_hierarchy!(IDWriteFontFallbackBuilder, windows_core::IUnknown);
impl IDWriteFontFallbackBuilder {
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
    pub unsafe fn AddMapping<P4, P5, P6>(&self, ranges: &[super::dwrite_1::DWRITE_UNICODE_RANGE], targetfamilynames: &[*const u16], fontcollection: P4, localename: P5, basefamilyname: P6, scale: f32) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::dwrite::IDWriteFontCollection>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMapping)(windows_core::Interface::as_raw(self), core::mem::transmute(ranges.as_ptr()), ranges.len().try_into().unwrap(), core::mem::transmute(targetfamilynames.as_ptr()), targetfamilynames.len().try_into().unwrap(), fontcollection.param().abi(), localename.param().abi(), basefamilyname.param().abi(), scale) }
    }
    pub unsafe fn AddMappings<P0>(&self, fontfallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMappings)(windows_core::Interface::as_raw(self), fontfallback.param().abi()) }
    }
    pub unsafe fn CreateFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallbackBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
    pub AddMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite_1::DWRITE_UNICODE_RANGE, u32, *const *const u16, u32, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, f32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1")))]
    AddMapping: usize,
    pub AddMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteFontFallbackBuilder_Impl: windows_core::IUnknownImpl {
    fn AddMapping(&self, ranges: *const super::dwrite_1::DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: windows_core::Ref<super::dwrite::IDWriteFontCollection>, localename: &windows_core::PCWSTR, basefamilyname: &windows_core::PCWSTR, scale: f32) -> windows_core::Result<()>;
    fn AddMappings(&self, fontfallback: windows_core::Ref<IDWriteFontFallback>) -> windows_core::Result<()>;
    fn CreateFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteFontFallbackBuilder_Vtbl {
    pub const fn new<Identity: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddMapping<Identity: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ranges: *const super::dwrite_1::DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut core::ffi::c_void, localename: windows_core::PCWSTR, basefamilyname: windows_core::PCWSTR, scale: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFallbackBuilder_Impl::AddMapping(this, core::mem::transmute_copy(&ranges), core::mem::transmute_copy(&rangescount), core::mem::transmute_copy(&targetfamilynames), core::mem::transmute_copy(&targetfamilynamescount), core::mem::transmute_copy(&fontcollection), core::mem::transmute(&localename), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&scale)).into()
            }
        }
        unsafe extern "system" fn AddMappings<Identity: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFallbackBuilder_Impl::AddMappings(this, core::mem::transmute_copy(&fontfallback)).into()
            }
        }
        unsafe extern "system" fn CreateFontFallback<Identity: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFallbackBuilder_Impl::CreateFontFallback(this) {
                    Ok(ok__) => {
                        fontfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMapping: AddMapping::<Identity, OFFSET>,
            AddMappings: AddMappings::<Identity, OFFSET>,
            CreateFontFallback: CreateFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallbackBuilder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteFontFallbackBuilder {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteRenderingParams2, IDWriteRenderingParams2_Vtbl, 0xf9d711c3_9777_40ae_87e8_3e5af9bf0948);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteRenderingParams2 {
    type Target = super::dwrite_1::IDWriteRenderingParams1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams2, windows_core::IUnknown, super::dwrite::IDWriteRenderingParams, super::dwrite_1::IDWriteRenderingParams1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteRenderingParams2 {
    pub unsafe fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetGridFitMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams2_Vtbl {
    pub base__: super::dwrite_1::IDWriteRenderingParams1_Vtbl,
    pub GetGridFitMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_GRID_FIT_MODE,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteRenderingParams2_Impl: super::dwrite_1::IDWriteRenderingParams1_Impl {
    fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteRenderingParams2_Vtbl {
    pub const fn new<Identity: IDWriteRenderingParams2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGridFitMode<Identity: IDWriteRenderingParams2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_GRID_FIT_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams2_Impl::GetGridFitMode(this)
            }
        }
        Self { base__: super::dwrite_1::IDWriteRenderingParams1_Vtbl::new::<Identity, OFFSET>(), GetGridFitMode: GetGridFitMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteRenderingParams as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteRenderingParams1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteRenderingParams2 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteTextAnalyzer2, IDWriteTextAnalyzer2_Vtbl, 0x553a9ff3_5693_4df7_b52b_74806f7f2eb9);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteTextAnalyzer2 {
    type Target = super::dwrite_1::IDWriteTextAnalyzer1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer2, windows_core::IUnknown, super::dwrite::IDWriteTextAnalyzer, super::dwrite_1::IDWriteTextAnalyzer1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextAnalyzer2 {
    pub unsafe fn GetGlyphOrientationTransform(&self, glyphorientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: bool, originx: f32, originy: f32, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphOrientationTransform)(windows_core::Interface::as_raw(self), glyphorientationangle, issideways.into(), originx, originy, transform as _) }
    }
    pub unsafe fn GetTypographicFeatures<P0, P2>(&self, fontface: P0, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: P2, actualtagcount: *mut u32, tags: &mut [super::dwrite::DWRITE_FONT_FEATURE_TAG]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTypographicFeatures)(windows_core::Interface::as_raw(self), fontface.param().abi(), core::mem::transmute(scriptanalysis), localename.param().abi(), tags.len().try_into().unwrap(), actualtagcount as _, core::mem::transmute(tags.as_ptr())) }
    }
    pub unsafe fn CheckTypographicFeature<P0, P2>(&self, fontface: P0, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: P2, featuretag: super::dwrite::DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CheckTypographicFeature)(windows_core::Interface::as_raw(self), fontface.param().abi(), core::mem::transmute(scriptanalysis), localename.param().abi(), featuretag, glyphcount, glyphindices, featureapplies as _) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer2_Vtbl {
    pub base__: super::dwrite_1::IDWriteTextAnalyzer1_Vtbl,
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, windows_core::BOOL, f32, f32, *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetTypographicFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dwrite::DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, u32, *mut u32, *mut super::dwrite::DWRITE_FONT_FEATURE_TAG) -> windows_core::HRESULT,
    pub CheckTypographicFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dwrite::DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, super::dwrite::DWRITE_FONT_FEATURE_TAG, u32, *const u16, *mut u8) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteTextAnalyzer2_Impl: super::dwrite_1::IDWriteTextAnalyzer1_Impl {
    fn GetGlyphOrientationTransform(&self, glyphorientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: windows_core::BOOL, originx: f32, originy: f32, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetTypographicFeatures(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut super::dwrite::DWRITE_FONT_FEATURE_TAG) -> windows_core::Result<()>;
    fn CheckTypographicFeature(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, featuretag: super::dwrite::DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextAnalyzer2_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: windows_core::BOOL, originx: f32, originy: f32, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer2_Impl::GetGlyphOrientationTransform(this, core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetTypographicFeatures<Identity: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut super::dwrite::DWRITE_FONT_FEATURE_TAG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer2_Impl::GetTypographicFeatures(this, core::mem::transmute_copy(&fontface), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&maxtagcount), core::mem::transmute_copy(&actualtagcount), core::mem::transmute_copy(&tags)).into()
            }
        }
        unsafe extern "system" fn CheckTypographicFeature<Identity: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, featuretag: super::dwrite::DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer2_Impl::CheckTypographicFeature(this, core::mem::transmute_copy(&fontface), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&featuretag), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&featureapplies)).into()
            }
        }
        Self {
            base__: super::dwrite_1::IDWriteTextAnalyzer1_Vtbl::new::<Identity, OFFSET>(),
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, OFFSET>,
            GetTypographicFeatures: GetTypographicFeatures::<Identity, OFFSET>,
            CheckTypographicFeature: CheckTypographicFeature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextAnalyzer as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteTextAnalyzer1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteTextAnalyzer2 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextFormat1, IDWriteTextFormat1_Vtbl, 0x5f174b49_0d8b_4cfb_8bca_f1cce9d06c67);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextFormat1 {
    type Target = super::dwrite::IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextFormat1, windows_core::IUnknown, super::dwrite::IDWriteTextFormat);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextFormat1 {
    #[cfg(feature = "Win32_dwrite_1")]
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), glyphorientation) }
    }
    #[cfg(feature = "Win32_dwrite_1")]
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION {
        unsafe { (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLastLineWrapping(&self, islastlinewrappingenabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastLineWrapping)(windows_core::Interface::as_raw(self), islastlinewrappingenabled.into()) }
    }
    pub unsafe fn GetLastLineWrapping(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetLastLineWrapping)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOpticalAlignment)(windows_core::Interface::as_raw(self), opticalalignment) }
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        unsafe { (windows_core::Interface::vtable(self).GetOpticalAlignment)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontFallback)(windows_core::Interface::as_raw(self), fontfallback.param().abi()) }
    }
    pub unsafe fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat1_Vtbl {
    pub base__: super::dwrite::IDWriteTextFormat_Vtbl,
    #[cfg(feature = "Win32_dwrite_1")]
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite_1"))]
    SetVerticalGlyphOrientation: usize,
    #[cfg(feature = "Win32_dwrite_1")]
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION,
    #[cfg(not(feature = "Win32_dwrite_1"))]
    GetVerticalGlyphOrientation: usize,
    pub SetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteTextFormat1_Impl: super::dwrite::IDWriteTextFormat_Impl {
    fn SetVerticalGlyphOrientation(&self, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLastLineWrapping(&self) -> windows_core::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: windows_core::Ref<IDWriteFontFallback>) -> windows_core::Result<()>;
    fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextFormat1_Vtbl {
    pub const fn new<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::SetVerticalGlyphOrientation(this, core::mem::transmute_copy(&glyphorientation)).into()
            }
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::GetVerticalGlyphOrientation(this)
            }
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, islastlinewrappingenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::SetLastLineWrapping(this, core::mem::transmute_copy(&islastlinewrappingenabled)).into()
            }
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::GetLastLineWrapping(this)
            }
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::SetOpticalAlignment(this, core::mem::transmute_copy(&opticalalignment)).into()
            }
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::GetOpticalAlignment(this)
            }
        }
        unsafe extern "system" fn SetFontFallback<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat1_Impl::SetFontFallback(this, core::mem::transmute_copy(&fontfallback)).into()
            }
        }
        unsafe extern "system" fn GetFontFallback<Identity: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextFormat1_Impl::GetFontFallback(this) {
                    Ok(ok__) => {
                        fontfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteTextFormat_Vtbl::new::<Identity, OFFSET>(),
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteTextFormat1 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteTextLayout2, IDWriteTextLayout2_Vtbl, 0x1093c18f_8d5e_43f0_b064_0917311b525e);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteTextLayout2 {
    type Target = super::dwrite_1::IDWriteTextLayout1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteTextLayout2, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite::IDWriteTextLayout, super::dwrite_1::IDWriteTextLayout1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextLayout2 {
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics as _) }
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), glyphorientation) }
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION {
        unsafe { (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLastLineWrapping(&self, islastlinewrappingenabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastLineWrapping)(windows_core::Interface::as_raw(self), islastlinewrappingenabled.into()) }
    }
    pub unsafe fn GetLastLineWrapping(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetLastLineWrapping)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOpticalAlignment)(windows_core::Interface::as_raw(self), opticalalignment) }
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        unsafe { (windows_core::Interface::vtable(self).GetOpticalAlignment)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontFallback)(windows_core::Interface::as_raw(self), fontfallback.param().abi()) }
    }
    pub unsafe fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout2_Vtbl {
    pub base__: super::dwrite_1::IDWriteTextLayout1_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS1) -> windows_core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION,
    pub SetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteTextLayout2_Impl: super::dwrite_1::IDWriteTextLayout1_Impl {
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::Result<()>;
    fn SetVerticalGlyphOrientation(&self, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLastLineWrapping(&self) -> windows_core::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: windows_core::Ref<IDWriteFontFallback>) -> windows_core::Result<()>;
    fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextLayout2_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetrics<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::GetMetrics(this, core::mem::transmute_copy(&textmetrics)).into()
            }
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientation: super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::SetVerticalGlyphOrientation(this, core::mem::transmute_copy(&glyphorientation)).into()
            }
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite_1::DWRITE_VERTICAL_GLYPH_ORIENTATION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::GetVerticalGlyphOrientation(this)
            }
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, islastlinewrappingenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::SetLastLineWrapping(this, core::mem::transmute_copy(&islastlinewrappingenabled)).into()
            }
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::GetLastLineWrapping(this)
            }
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::SetOpticalAlignment(this, core::mem::transmute_copy(&opticalalignment)).into()
            }
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::GetOpticalAlignment(this)
            }
        }
        unsafe extern "system" fn SetFontFallback<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout2_Impl::SetFontFallback(this, core::mem::transmute_copy(&fontfallback)).into()
            }
        }
        unsafe extern "system" fn GetFontFallback<Identity: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextLayout2_Impl::GetFontFallback(this) {
                    Ok(ok__) => {
                        fontfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite_1::IDWriteTextLayout1_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextLayout as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteTextLayout1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteTextLayout2 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextRenderer1, IDWriteTextRenderer1_Vtbl, 0xd3e0e934_22a0_427e_aae4_7d9574b59db1);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextRenderer1 {
    type Target = super::dwrite::IDWriteTextRenderer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextRenderer1, windows_core::IUnknown, super::dwrite::IDWritePixelSnapping, super::dwrite::IDWriteTextRenderer);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextRenderer1 {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub unsafe fn DrawGlyphRun<P7>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P7) -> windows_core::HRESULT
    where
        P7: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, orientationangle, measuringmode, core::mem::transmute(glyphrun), glyphrundescription, clientdrawingeffect.param().abi()) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub unsafe fn DrawUnderline<P5>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const super::dwrite::DWRITE_UNDERLINE, clientdrawingeffect: P5) -> windows_core::HRESULT
    where
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawUnderline)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, orientationangle, underline, clientdrawingeffect.param().abi()) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub unsafe fn DrawStrikethrough<P5>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const super::dwrite::DWRITE_STRIKETHROUGH, clientdrawingeffect: P5) -> windows_core::HRESULT
    where
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawStrikethrough)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, orientationangle, strikethrough, clientdrawingeffect.param().abi()) }
    }
    #[cfg(feature = "Win32_dwrite_1")]
    pub unsafe fn DrawInlineObject<P4, P7>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, originx: f32, originy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: P4, issideways: bool, isrighttoleft: bool, clientdrawingeffect: P7) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::dwrite::IDWriteInlineObject>,
        P7: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawInlineObject)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, originx, originy, orientationangle, inlineobject.param().abi(), issideways.into(), isrighttoleft.into(), clientdrawingeffect.param().abi()) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextRenderer1_Vtbl {
    pub base__: super::dwrite::IDWriteTextRenderer_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1")))]
    DrawGlyphRun: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub DrawUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, *const super::dwrite::DWRITE_UNDERLINE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1")))]
    DrawUnderline: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1"))]
    pub DrawStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, *const super::dwrite::DWRITE_STRIKETHROUGH, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite_1")))]
    DrawStrikethrough: usize,
    #[cfg(feature = "Win32_dwrite_1")]
    pub DrawInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite_1"))]
    DrawInlineObject: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
pub trait IDWriteTextRenderer1_Impl: super::dwrite::IDWriteTextRenderer_Impl {
    fn DrawGlyphRun(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawUnderline(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const super::dwrite::DWRITE_UNDERLINE, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawStrikethrough(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const super::dwrite::DWRITE_STRIKETHROUGH, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawInlineObject(&self, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: windows_core::Ref<super::dwrite::IDWriteInlineObject>, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteTextRenderer1_Vtbl {
    pub const fn new<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawGlyphRun<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawUnderline<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const super::dwrite::DWRITE_UNDERLINE, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawUnderline(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&underline), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const super::dwrite::DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawStrikethrough(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&strikethrough), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawInlineObject<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, orientationangle: super::dwrite_1::DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut core::ffi::c_void, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawInlineObject(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&inlineobject), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        Self {
            base__: super::dwrite::IDWriteTextRenderer_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWritePixelSnapping as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextRenderer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl windows_core::RuntimeName for IDWriteTextRenderer1 {}
