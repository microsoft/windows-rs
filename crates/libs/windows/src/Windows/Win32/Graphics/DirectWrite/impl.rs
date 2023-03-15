#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteAsyncResult_Impl: Sized {
    fn GetWaitHandle(&self) -> super::super::Foundation::HANDLE;
    fn GetResult(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteAsyncResult {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: isize>() -> IDWriteAsyncResult_Vtbl {
        unsafe extern "system" fn GetWaitHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWaitHandle()
        }
        unsafe extern "system" fn GetResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResult().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWaitHandle: GetWaitHandle::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteAsyncResult as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTarget_Impl: Sized {
    fn DrawGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetMemoryDC(&self) -> super::Gdi::HDC;
    fn GetPixelsPerDip(&self) -> f32;
    fn SetPixelsPerDip(&self, pixelsperdip: f32) -> ::windows::core::Result<()>;
    fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()>;
    fn SetCurrentTransform(&self, transform: *const DWRITE_MATRIX) -> ::windows::core::Result<()>;
    fn GetSize(&self) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteBitmapRenderTarget {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteBitmapRenderTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget_Vtbl {
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut ::core::ffi::c_void, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawGlyphRun(::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::windows::core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&textcolor), ::core::mem::transmute_copy(&blackboxrect)).into()
        }
        unsafe extern "system" fn GetMemoryDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Gdi::HDC {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemoryDC()
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelsPerDip()
        }
        unsafe extern "system" fn SetPixelsPerDip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelsperdip: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPixelsPerDip(::core::mem::transmute_copy(&pixelsperdip)).into()
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn SetCurrentTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            GetMemoryDC: GetMemoryDC::<Identity, Impl, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, Impl, OFFSET>,
            SetPixelsPerDip: SetPixelsPerDip::<Identity, Impl, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, Impl, OFFSET>,
            SetCurrentTransform: SetCurrentTransform::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTarget1_Impl: Sized + IDWriteBitmapRenderTarget_Impl {
    fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteBitmapRenderTarget1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteBitmapRenderTarget1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget1_Vtbl {
        unsafe extern "system" fn GetTextAntialiasMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextAntialiasMode()
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextAntialiasMode(::core::mem::transmute_copy(&antialiasmode)).into()
        }
        Self {
            base__: IDWriteBitmapRenderTarget_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, Impl, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteBitmapRenderTarget as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentRun(&self) -> ::windows::core::Result<*mut DWRITE_COLOR_GLYPH_RUN>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteColorGlyphRunEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteColorGlyphRunEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentRun() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorglyphrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            GetCurrentRun: GetCurrentRun::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumerator1_Impl: Sized + IDWriteColorGlyphRunEnumerator_Impl {
    fn GetCurrentRun2(&self) -> ::windows::core::Result<*mut DWRITE_COLOR_GLYPH_RUN1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteColorGlyphRunEnumerator1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator1_Vtbl {
        unsafe extern "system" fn GetCurrentRun2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentRun2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorglyphrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteColorGlyphRunEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(), GetCurrentRun2: GetCurrentRun2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteColorGlyphRunEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory_Impl: Sized {
    fn GetSystemFontCollection(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateCustomFontCollection(&self, collectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(&self, fontcollectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>) -> ::windows::core::Result<()>;
    fn UnregisterFontCollectionLoader(&self, fontcollectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>) -> ::windows::core::Result<()>;
    fn CreateFontFileReference(&self, filepath: &::windows::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows::core::Result<IDWriteFontFile>;
    fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const ::core::option::Option<IDWriteFontFile>, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(&self, monitor: super::Gdi::HMONITOR) -> ::windows::core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(&self, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows::core::Result<()>;
    fn UnregisterFontFileLoader(&self, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows::core::Result<()>;
    fn CreateTextFormat(&self, fontfamilyname: &::windows::core::PCWSTR, fontcollection: ::core::option::Option<&IDWriteFontCollection>, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: &::windows::core::PCWSTR) -> ::windows::core::Result<IDWriteTextFormat>;
    fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography>;
    fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(&self, string: &::windows::core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&IDWriteTextFormat>, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(&self, string: &::windows::core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&IDWriteTextFormat>, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(&self, textformat: ::core::option::Option<&IDWriteTextFormat>) -> ::windows::core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: &::windows::core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>() -> IDWriteFactory_Vtbl {
        unsafe extern "system" fn GetSystemFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSystemFontCollection(::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn CreateCustomFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionloader: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomFontCollection(::windows::core::from_raw_borrowed(&collectionloader), ::core::mem::transmute_copy(&collectionkey), ::core::mem::transmute_copy(&collectionkeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterFontCollectionLoader(::windows::core::from_raw_borrowed(&fontcollectionloader)).into()
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterFontCollectionLoader(::windows::core::from_raw_borrowed(&fontcollectionloader)).into()
        }
        unsafe extern "system" fn CreateFontFileReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::windows::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFileReference(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&lastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomFontFileReference(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize), ::windows::core::from_raw_borrowed(&fontfileloader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut ::core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace(::core::mem::transmute_copy(&fontfacetype), ::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&fontfiles), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontfacesimulationflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderingParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRenderingParams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMonitorRenderingParams(::core::mem::transmute_copy(&monitor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomRenderingParams(::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterFontFileLoader(::windows::core::from_raw_borrowed(&fontfileloader)).into()
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterFontFileLoader(::windows::core::from_raw_borrowed(&fontfileloader)).into()
        }
        unsafe extern "system" fn CreateTextFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows::core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: ::windows::core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTextFormat(::core::mem::transmute(&fontfamilyname), ::windows::core::from_raw_borrowed(&fontcollection), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&localename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTypography<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTypography() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typography, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiInterop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdiinterop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGdiInterop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gdiinterop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::windows::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTextLayout(::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows::core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&maxwidth), ::core::mem::transmute_copy(&maxheight)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textlayout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::windows::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGdiCompatibleTextLayout(::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows::core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&layoutwidth), ::core::mem::transmute_copy(&layoutheight), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textlayout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textformat: *mut ::core::ffi::c_void, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEllipsisTrimmingSign(::windows::core::from_raw_borrowed(&textformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(trimmingsign, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textanalyzer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTextAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textanalyzer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: ::windows::core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNumberSubstitution(::core::mem::transmute_copy(&substitutionmethod), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&ignoreuseroverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numbersubstitution, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGlyphRunAnalysis(::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontCollection: GetSystemFontCollection::<Identity, Impl, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, Impl, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, Impl, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, Impl, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, Impl, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, Impl, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, Impl, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, Impl, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, Impl, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, Impl, OFFSET>,
            CreateTypography: CreateTypography::<Identity, Impl, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, Impl, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, Impl, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, Impl, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, Impl, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, Impl, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, Impl, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory1_Impl: Sized + IDWriteFactory_Impl {
    fn GetEudcFontCollection(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: isize>() -> IDWriteFactory1_Vtbl {
        unsafe extern "system" fn GetEudcFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEudcFontCollection(::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn CreateCustomRenderingParams2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomRenderingParams2(::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&enhancedcontrastgrayscale), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetEudcFontCollection: GetEudcFontCollection::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams2: CreateCustomRenderingParams2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory2_Impl: Sized + IDWriteFactory1_Impl {
    fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback>;
    fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder>;
    fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator>;
    fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2>;
    fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>() -> IDWriteFactory2_Vtbl {
        unsafe extern "system" fn GetSystemFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallbackbuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFallbackBuilder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallbackbuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TranslateColorGlyphRun(::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&worldtodevicetransform), ::core::mem::transmute_copy(&colorpaletteindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorlayers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomRenderingParams3(::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&grayscaleenhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGlyphRunAnalysis2(::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&gridfitmode), ::core::mem::transmute_copy(&antialiasmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSystemFontFallback: GetSystemFontFallback::<Identity, Impl, OFFSET>,
            CreateFontFallbackBuilder: CreateFontFallbackBuilder::<Identity, Impl, OFFSET>,
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams3: CreateCustomRenderingParams3::<Identity, Impl, OFFSET>,
            CreateGlyphRunAnalysis2: CreateGlyphRunAnalysis2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory3_Impl: Sized + IDWriteFactory2_Impl {
    fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis>;
    fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams3>;
    fn CreateFontFaceReference(&self, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>;
    fn CreateFontFaceReference2(&self, filepath: &::windows::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>;
    fn GetSystemFontSet(&self) -> ::windows::core::Result<IDWriteFontSet>;
    fn CreateFontSetBuilder(&self) -> ::windows::core::Result<IDWriteFontSetBuilder>;
    fn CreateFontCollectionFromFontSet(&self, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows::core::Result<IDWriteFontCollection1>;
    fn GetSystemFontCollection2(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFontDownloadQueue(&self) -> ::windows::core::Result<IDWriteFontDownloadQueue>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>() -> IDWriteFactory3_Vtbl {
        unsafe extern "system" fn CreateGlyphRunAnalysis3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGlyphRunAnalysis3(::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&gridfitmode), ::core::mem::transmute_copy(&antialiasmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomRenderingParams4(::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&grayscaleenhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceReference(::windows::core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::windows::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceReference2(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&lastwritetime), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontSetBuilder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontCollectionFromFontSet(::windows::core::from_raw_borrowed(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSystemFontCollection2(::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn GetFontDownloadQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontdownloadqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontDownloadQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontdownloadqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateGlyphRunAnalysis3: CreateGlyphRunAnalysis3::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams4: CreateCustomRenderingParams4::<Identity, Impl, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, Impl, OFFSET>,
            CreateFontFaceReference2: CreateFontFaceReference2::<Identity, Impl, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, Impl, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, Impl, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, Impl, OFFSET>,
            GetSystemFontCollection2: GetSystemFontCollection2::<Identity, Impl, OFFSET>,
            GetFontDownloadQueue: GetFontDownloadQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory4_Impl: Sized + IDWriteFactory3_Impl {
    fn TranslateColorGlyphRun2(&self, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator1>;
    fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F>;
    fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: isize>() -> IDWriteFactory4_Vtbl {
        unsafe extern "system" fn TranslateColorGlyphRun2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TranslateColorGlyphRun2(::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::core::mem::transmute_copy(&desiredglyphimageformats), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&worldanddpitransform), ::core::mem::transmute_copy(&colorpaletteindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorlayers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputeGlyphOrigins(::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute(&baselineorigin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphorigins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputeGlyphOrigins2(::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&worldanddpitransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphorigins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory3_Vtbl::new::<Identity, Impl, OFFSET>(),
            TranslateColorGlyphRun2: TranslateColorGlyphRun2::<Identity, Impl, OFFSET>,
            ComputeGlyphOrigins: ComputeGlyphOrigins::<Identity, Impl, OFFSET>,
            ComputeGlyphOrigins2: ComputeGlyphOrigins2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory5_Impl: Sized + IDWriteFactory4_Impl {
    fn CreateFontSetBuilder2(&self) -> ::windows::core::Result<IDWriteFontSetBuilder1>;
    fn CreateInMemoryFontFileLoader(&self) -> ::windows::core::Result<IDWriteInMemoryFontFileLoader>;
    fn CreateHttpFontFileLoader(&self, referrerurl: &::windows::core::PCWSTR, extraheaders: &::windows::core::PCWSTR) -> ::windows::core::Result<IDWriteRemoteFontFileLoader>;
    fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE;
    fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows::core::Result<IDWriteFontFileStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>() -> IDWriteFactory5_Vtbl {
        unsafe extern "system" fn CreateFontSetBuilder2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontSetBuilder2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newloader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInMemoryFontFileLoader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newloader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referrerurl: ::windows::core::PCWSTR, extraheaders: ::windows::core::PCWSTR, newloader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateHttpFontFileLoader(::core::mem::transmute(&referrerurl), ::core::mem::transmute(&extraheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newloader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeContainerType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeContainerType(::core::mem::transmute_copy(&filedata), ::core::mem::transmute_copy(&filedatasize))
        }
        unsafe extern "system" fn UnpackFontFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnpackFontFile(::core::mem::transmute_copy(&containertype), ::core::mem::transmute_copy(&filedata), ::core::mem::transmute_copy(&filedatasize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unpackedfontstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory4_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFontSetBuilder2: CreateFontSetBuilder2::<Identity, Impl, OFFSET>,
            CreateInMemoryFontFileLoader: CreateInMemoryFontFileLoader::<Identity, Impl, OFFSET>,
            CreateHttpFontFileLoader: CreateHttpFontFileLoader::<Identity, Impl, OFFSET>,
            AnalyzeContainerType: AnalyzeContainerType::<Identity, Impl, OFFSET>,
            UnpackFontFile: UnpackFontFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory5 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory6_Impl: Sized + IDWriteFactory5_Impl {
    fn CreateFontFaceReference3(&self, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32) -> ::windows::core::Result<IDWriteFontResource>;
    fn GetSystemFontSet2(&self, includedownloadablefonts: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetSystemFontCollection3(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteFontCollection2>;
    fn CreateFontCollectionFromFontSet2(&self, fontset: ::core::option::Option<&IDWriteFontSet>, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteFontCollection2>;
    fn CreateFontSetBuilder3(&self) -> ::windows::core::Result<IDWriteFontSetBuilder2>;
    fn CreateTextFormat2(&self, fontfamilyname: &::windows::core::PCWSTR, fontcollection: ::core::option::Option<&IDWriteFontCollection>, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: &::windows::core::PCWSTR) -> ::windows::core::Result<IDWriteTextFormat3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>() -> IDWriteFactory6_Vtbl {
        unsafe extern "system" fn CreateFontFaceReference3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceReference3(::windows::core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontResource(::windows::core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontSet2(::core::mem::transmute_copy(&includedownloadablefonts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontCollection3(::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontCollectionFromFontSet2(::windows::core::from_raw_borrowed(&fontset), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontSetBuilder3() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextFormat2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows::core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: ::windows::core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTextFormat2(::core::mem::transmute(&fontfamilyname), ::windows::core::from_raw_borrowed(&fontcollection), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&localename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory5_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFontFaceReference3: CreateFontFaceReference3::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            GetSystemFontSet2: GetSystemFontSet2::<Identity, Impl, OFFSET>,
            GetSystemFontCollection3: GetSystemFontCollection3::<Identity, Impl, OFFSET>,
            CreateFontCollectionFromFontSet2: CreateFontCollectionFromFontSet2::<Identity, Impl, OFFSET>,
            CreateFontSetBuilder3: CreateFontSetBuilder3::<Identity, Impl, OFFSET>,
            CreateTextFormat2: CreateTextFormat2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory6 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory7_Impl: Sized + IDWriteFactory6_Impl {
    fn GetSystemFontSet3(&self, includedownloadablefonts: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteFontSet2>;
    fn GetSystemFontCollection4(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteFontCollection3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteFactory7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: isize>() -> IDWriteFactory7_Vtbl {
        unsafe extern "system" fn GetSystemFontSet3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontSet3(::core::mem::transmute_copy(&includedownloadablefonts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemFontCollection4(::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory6_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSystemFontSet3: GetSystemFontSet3::<Identity, Impl, OFFSET>,
            GetSystemFontCollection4: GetSystemFontCollection4::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory7 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory5 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFactory6 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont_Impl: Sized {
    fn GetFontFamily(&self) -> ::windows::core::Result<IDWriteFontFamily>;
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> DWRITE_FONT_STYLE;
    fn IsSymbolFont(&self) -> super::super::Foundation::BOOL;
    fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS);
    fn HasCharacter(&self, unicodevalue: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFont {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>() -> IDWriteFont_Vtbl {
        unsafe extern "system" fn GetFontFamily<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWeight()
        }
        unsafe extern "system" fn GetStretch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStretch()
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStyle()
        }
        unsafe extern "system" fn IsSymbolFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSymbolFont()
        }
        unsafe extern "system" fn GetFaceNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFaceNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInformationalStrings(::core::mem::transmute_copy(&informationalstringid), ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSimulations()
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics(::core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn HasCharacter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCharacter(::core::mem::transmute_copy(&unicodevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamily: GetFontFamily::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            GetStretch: GetStretch::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, Impl, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, Impl, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            HasCharacter: HasCharacter::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont1_Impl: Sized + IDWriteFont_Impl {
    fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetPanose(&self, panose: *mut DWRITE_PANOSE) -> ();
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::Result<()>;
    fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFont1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: isize>() -> IDWriteFont1_Vtbl {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics2(::core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn GetPanose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPanose(::core::mem::transmute_copy(&panose))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUnicodeRanges(::core::mem::transmute_copy(&maxrangecount), ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)).into()
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMonospacedFont()
        }
        Self {
            base__: IDWriteFont_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            GetPanose: GetPanose::<Identity, Impl, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, Impl, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont2_Impl: Sized + IDWriteFont1_Impl {
    fn IsColorFont(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFont2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont2_Impl, const OFFSET: isize>() -> IDWriteFont2_Vtbl {
        unsafe extern "system" fn IsColorFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsColorFont()
        }
        Self { base__: IDWriteFont1_Vtbl::new::<Identity, Impl, OFFSET>(), IsColorFont: IsColorFont::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont3_Impl: Sized + IDWriteFont2_Impl {
    fn CreateFontFace2(&self) -> ::windows::core::Result<IDWriteFontFace3>;
    fn Equals(&self, font: ::core::option::Option<&IDWriteFont>) -> super::super::Foundation::BOOL;
    fn GetFontFaceReference(&self) -> ::windows::core::Result<IDWriteFontFaceReference>;
    fn HasCharacter2(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFont3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>() -> IDWriteFont3_Vtbl {
        unsafe extern "system" fn CreateFontFace2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Equals(::windows::core::from_raw_borrowed(&font))
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCharacter2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasCharacter2(::core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocality()
        }
        Self {
            base__: IDWriteFont2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFontFace2: CreateFontFace2::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            HasCharacter2: HasCharacter2::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFont2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection_Impl: Sized {
    fn GetFontFamilyCount(&self) -> u32;
    fn GetFontFamily(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily>;
    fn FindFamilyName(&self, familyname: &::windows::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFontFromFontFace(&self, fontface: ::core::option::Option<&IDWriteFontFace>) -> ::windows::core::Result<IDWriteFont>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontCollection {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: isize>() -> IDWriteFontCollection_Vtbl {
        unsafe extern "system" fn GetFontFamilyCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyCount()
        }
        unsafe extern "system" fn GetFontFamily<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFamily(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: ::windows::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindFamilyName(::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetFontFromFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFromFontFace(::windows::core::from_raw_borrowed(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamilyCount: GetFontFamilyCount::<Identity, Impl, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, Impl, OFFSET>,
            FindFamilyName: FindFamilyName::<Identity, Impl, OFFSET>,
            GetFontFromFontFace: GetFontFromFontFace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection1_Impl: Sized + IDWriteFontCollection_Impl {
    fn GetFontSet(&self) -> ::windows::core::Result<IDWriteFontSet>;
    fn GetFontFamily2(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontCollection1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: isize>() -> IDWriteFontCollection1_Vtbl {
        unsafe extern "system" fn GetFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamily2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFamily2(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontSet: GetFontSet::<Identity, Impl, OFFSET>,
            GetFontFamily2: GetFontFamily2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection2_Impl: Sized + IDWriteFontCollection1_Impl {
    fn GetFontFamily3(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily2>;
    fn GetMatchingFonts(&self, familyname: &::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontList2>;
    fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL;
    fn GetFontSet2(&self) -> ::windows::core::Result<IDWriteFontSet1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontCollection2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: isize>() -> IDWriteFontCollection2_Vtbl {
        unsafe extern "system" fn GetFontFamily3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFamily3(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: ::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts(::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyModel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyModel()
        }
        unsafe extern "system" fn GetFontSet2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontSet2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontCollection1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontFamily3: GetFontFamily3::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
            GetFontFamilyModel: GetFontFamilyModel::<Identity, Impl, OFFSET>,
            GetFontSet2: GetFontSet2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection3_Impl: Sized + IDWriteFontCollection2_Impl {
    fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontCollection3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection3_Impl, const OFFSET: isize>() -> IDWriteFontCollection3_Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollection3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExpirationEvent()
        }
        Self { base__: IDWriteFontCollection2_Vtbl::new::<Identity, Impl, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontCollection2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontCollectionLoader_Impl: Sized {
    fn CreateEnumeratorFromKey(&self, factory: ::core::option::Option<&IDWriteFactory>, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontFileEnumerator>;
}
impl ::windows::core::RuntimeName for IDWriteFontCollectionLoader {}
impl IDWriteFontCollectionLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollectionLoader_Impl, const OFFSET: isize>() -> IDWriteFontCollectionLoader_Vtbl {
        unsafe extern "system" fn CreateEnumeratorFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontCollectionLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEnumeratorFromKey(::windows::core::from_raw_borrowed(&factory), ::core::mem::transmute_copy(&collectionkey), ::core::mem::transmute_copy(&collectionkeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfileenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateEnumeratorFromKey: CreateEnumeratorFromKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollectionLoader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontDownloadListener_Impl: Sized {
    fn DownloadCompleted(&self, downloadqueue: ::core::option::Option<&IDWriteFontDownloadQueue>, context: ::core::option::Option<&::windows::core::IUnknown>, downloadresult: ::windows::core::HRESULT);
}
impl ::windows::core::RuntimeName for IDWriteFontDownloadListener {}
impl IDWriteFontDownloadListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadListener_Impl, const OFFSET: isize>() -> IDWriteFontDownloadListener_Vtbl {
        unsafe extern "system" fn DownloadCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadqueue: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, downloadresult: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadCompleted(::windows::core::from_raw_borrowed(&downloadqueue), ::windows::core::from_raw_borrowed(&context), ::core::mem::transmute_copy(&downloadresult))
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DownloadCompleted: DownloadCompleted::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontDownloadListener as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontDownloadQueue_Impl: Sized {
    fn AddListener(&self, listener: ::core::option::Option<&IDWriteFontDownloadListener>) -> ::windows::core::Result<u32>;
    fn RemoveListener(&self, token: u32) -> ::windows::core::Result<()>;
    fn IsEmpty(&self) -> super::super::Foundation::BOOL;
    fn BeginDownload(&self, context: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CancelDownload(&self) -> ::windows::core::Result<()>;
    fn GetGenerationCount(&self) -> u64;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontDownloadQueue {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontDownloadQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>() -> IDWriteFontDownloadQueue_Vtbl {
        unsafe extern "system" fn AddListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: *mut ::core::ffi::c_void, token: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddListener(::windows::core::from_raw_borrowed(&listener)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveListener(::core::mem::transmute_copy(&token)).into()
        }
        unsafe extern "system" fn IsEmpty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEmpty()
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDownload(::windows::core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn CancelDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelDownload().into()
        }
        unsafe extern "system" fn GetGenerationCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGenerationCount()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddListener: AddListener::<Identity, Impl, OFFSET>,
            RemoveListener: RemoveListener::<Identity, Impl, OFFSET>,
            IsEmpty: IsEmpty::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
            CancelDownload: CancelDownload::<Identity, Impl, OFFSET>,
            GetGenerationCount: GetGenerationCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontDownloadQueue as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace_Impl: Sized {
    fn GetType(&self) -> DWRITE_FONT_FACE_TYPE;
    fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: *mut ::core::option::Option<IDWriteFontFile>) -> ::windows::core::Result<()>;
    fn GetIndex(&self) -> u32;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn IsSymbolFont(&self) -> super::super::Foundation::BOOL;
    fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS);
    fn GetGlyphCount(&self) -> u16;
    fn GetDesignGlyphMetrics(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()>;
    fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void);
    fn GetGlyphRunOutline(&self, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: ::core::option::Option<&super::Direct2D::Common::ID2D1SimplifiedGeometrySink>) -> ::windows::core::Result<()>;
    fn GetRecommendedRenderingMode(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>) -> ::windows::core::Result<DWRITE_RENDERING_MODE>;
    fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()>;
    fn GetGdiCompatibleGlyphMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>() -> IDWriteFontFace_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType()
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFiles(::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&fontfiles)).into()
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIndex()
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSimulations()
        }
        unsafe extern "system" fn IsSymbolFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSymbolFont()
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics(::core::mem::transmute_copy(&fontfacemetrics))
        }
        unsafe extern "system" fn GetGlyphCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphCount()
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesignGlyphMetrics(::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphmetrics), ::core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphIndices(::core::mem::transmute_copy(&codepoints), ::core::mem::transmute_copy(&codepointcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn TryGetFontTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryGetFontTable(::core::mem::transmute_copy(&opentypetabletag), ::core::mem::transmute_copy(&tabledata), ::core::mem::transmute_copy(&tablesize), ::core::mem::transmute_copy(&tablecontext), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn ReleaseFontTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablecontext: *const ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFontTable(::core::mem::transmute_copy(&tablecontext))
        }
        unsafe extern "system" fn GetGlyphRunOutline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphRunOutline(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&glyphoffsets), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows::core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecommendedRenderingMode(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&measuringmode), ::windows::core::from_raw_borrowed(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGdiCompatibleMetrics(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&fontfacemetrics)).into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGdiCompatibleGlyphMetrics(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphmetrics), ::core::mem::transmute_copy(&issideways)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetGlyphCount: GetGlyphCount::<Identity, Impl, OFFSET>,
            GetDesignGlyphMetrics: GetDesignGlyphMetrics::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            TryGetFontTable: TryGetFontTable::<Identity, Impl, OFFSET>,
            ReleaseFontTable: ReleaseFontTable::<Identity, Impl, OFFSET>,
            GetGlyphRunOutline: GetGlyphRunOutline::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, Impl, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphMetrics: GetGdiCompatibleGlyphMetrics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace1_Impl: Sized + IDWriteFontFace_Impl {
    fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()>;
    fn GetCaretMetrics(&self, caretmetrics: *mut DWRITE_CARET_METRICS) -> ();
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::Result<()>;
    fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL;
    fn GetDesignGlyphAdvances(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetGdiCompatibleGlyphAdvances(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>;
    fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()>;
    fn HasKerningPairs(&self) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode2(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>;
    fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()>;
    fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>() -> IDWriteFontFace1_Vtbl {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics2(::core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGdiCompatibleMetrics2(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&fontmetrics)).into()
        }
        unsafe extern "system" fn GetCaretMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaretMetrics(::core::mem::transmute_copy(&caretmetrics))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUnicodeRanges(::core::mem::transmute_copy(&maxrangecount), ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)).into()
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMonospacedFont()
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesignGlyphAdvances(::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGdiCompatibleGlyphAdvances(::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances)).into()
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKerningPairAdjustments(::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvanceadjustments)).into()
        }
        unsafe extern "system" fn HasKerningPairs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasKerningPairs()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecommendedRenderingMode2(::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalGlyphVariants(::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&nominalglyphindices), ::core::mem::transmute_copy(&verticalglyphindices)).into()
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasVerticalGlyphVariants()
        }
        Self {
            base__: IDWriteFontFace_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            GetGdiCompatibleMetrics2: GetGdiCompatibleMetrics2::<Identity, Impl, OFFSET>,
            GetCaretMetrics: GetCaretMetrics::<Identity, Impl, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, Impl, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, Impl, OFFSET>,
            GetDesignGlyphAdvances: GetDesignGlyphAdvances::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphAdvances: GetGdiCompatibleGlyphAdvances::<Identity, Impl, OFFSET>,
            GetKerningPairAdjustments: GetKerningPairAdjustments::<Identity, Impl, OFFSET>,
            HasKerningPairs: HasKerningPairs::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode2: GetRecommendedRenderingMode2::<Identity, Impl, OFFSET>,
            GetVerticalGlyphVariants: GetVerticalGlyphVariants::<Identity, Impl, OFFSET>,
            HasVerticalGlyphVariants: HasVerticalGlyphVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace2_Impl: Sized + IDWriteFontFace1_Impl {
    fn IsColorFont(&self) -> super::super::Foundation::BOOL;
    fn GetColorPaletteCount(&self) -> u32;
    fn GetPaletteEntryCount(&self) -> u32;
    fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows::core::Result<()>;
    fn GetRecommendedRenderingMode3(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>() -> IDWriteFontFace2_Vtbl {
        unsafe extern "system" fn IsColorFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsColorFont()
        }
        unsafe extern "system" fn GetColorPaletteCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorPaletteCount()
        }
        unsafe extern "system" fn GetPaletteEntryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPaletteEntryCount()
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPaletteEntries(::core::mem::transmute_copy(&colorpaletteindex), ::core::mem::transmute_copy(&firstentryindex), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute_copy(&paletteentries)).into()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecommendedRenderingMode3(::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode), ::windows::core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)).into()
        }
        Self {
            base__: IDWriteFontFace1_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsColorFont: IsColorFont::<Identity, Impl, OFFSET>,
            GetColorPaletteCount: GetColorPaletteCount::<Identity, Impl, OFFSET>,
            GetPaletteEntryCount: GetPaletteEntryCount::<Identity, Impl, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode3: GetRecommendedRenderingMode3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace3_Impl: Sized + IDWriteFontFace2_Impl {
    fn GetFontFaceReference(&self) -> ::windows::core::Result<IDWriteFontFaceReference>;
    fn GetPanose(&self, panose: *mut DWRITE_PANOSE) -> ();
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode4(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>;
    fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL;
    fn AreCharactersLocal(&self, characters: &::windows::core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AreGlyphsLocal(&self, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>() -> IDWriteFontFace3_Vtbl {
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPanose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPanose(::core::mem::transmute_copy(&panose))
        }
        unsafe extern "system" fn GetWeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWeight()
        }
        unsafe extern "system" fn GetStretch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStretch()
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStyle()
        }
        unsafe extern "system" fn GetFamilyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFaceNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInformationalStrings(::core::mem::transmute_copy(&informationalstringid), ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn HasCharacter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasCharacter(::core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn GetRecommendedRenderingMode4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecommendedRenderingMode4(::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode), ::windows::core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)).into()
        }
        unsafe extern "system" fn IsCharacterLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCharacterLocal(::core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn IsGlyphLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsGlyphLocal(::core::mem::transmute_copy(&glyphid))
        }
        unsafe extern "system" fn AreCharactersLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: ::windows::core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreCharactersLocal(::core::mem::transmute(&characters), ::core::mem::transmute_copy(&charactercount), ::core::mem::transmute_copy(&enqueueifnotlocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreGlyphsLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreGlyphsLocal(::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&enqueueifnotlocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFace2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            GetPanose: GetPanose::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            GetStretch: GetStretch::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            GetFamilyNames: GetFamilyNames::<Identity, Impl, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, Impl, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, Impl, OFFSET>,
            HasCharacter: HasCharacter::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode4: GetRecommendedRenderingMode4::<Identity, Impl, OFFSET>,
            IsCharacterLocal: IsCharacterLocal::<Identity, Impl, OFFSET>,
            IsGlyphLocal: IsGlyphLocal::<Identity, Impl, OFFSET>,
            AreCharactersLocal: AreCharactersLocal::<Identity, Impl, OFFSET>,
            AreGlyphsLocal: AreGlyphsLocal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace4_Impl: Sized + IDWriteFontFace3_Impl {
    fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows::core::Result<DWRITE_GLYPH_IMAGE_FORMATS>;
    fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS;
    fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: isize>() -> IDWriteFontFace4_Vtbl {
        unsafe extern "system" fn GetGlyphImageFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphImageFormats(::core::mem::transmute_copy(&glyphid), ::core::mem::transmute_copy(&pixelsperemfirst), ::core::mem::transmute_copy(&pixelsperemlast)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphimageformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphImageFormats2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphImageFormats2()
        }
        unsafe extern "system" fn GetGlyphImageData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphImageData(::core::mem::transmute_copy(&glyphid), ::core::mem::transmute_copy(&pixelsperem), ::core::mem::transmute_copy(&glyphimageformat), ::core::mem::transmute_copy(&glyphdata), ::core::mem::transmute_copy(&glyphdatacontext)).into()
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphdatacontext: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseGlyphImageData(::core::mem::transmute_copy(&glyphdatacontext))
        }
        Self {
            base__: IDWriteFontFace3_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGlyphImageFormats: GetGlyphImageFormats::<Identity, Impl, OFFSET>,
            GetGlyphImageFormats2: GetGlyphImageFormats2::<Identity, Impl, OFFSET>,
            GetGlyphImageData: GetGlyphImageData::<Identity, Impl, OFFSET>,
            ReleaseGlyphImageData: ReleaseGlyphImageData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace5_Impl: Sized + IDWriteFontFace4_Impl {
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<()>;
    fn HasVariations(&self) -> super::super::Foundation::BOOL;
    fn GetFontResource(&self) -> ::windows::core::Result<IDWriteFontResource>;
    fn Equals(&self, fontface: ::core::option::Option<&IDWriteFontFace>) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>() -> IDWriteFontFace5_Vtbl {
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValueCount()
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn HasVariations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasVariations()
        }
        unsafe extern "system" fn GetFontResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Equals(::windows::core::from_raw_borrowed(&fontface))
        }
        Self {
            base__: IDWriteFontFace4_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            HasVariations: HasVariations::<Identity, Impl, OFFSET>,
            GetFontResource: GetFontResource::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace5 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace6_Impl: Sized + IDWriteFontFace5_Impl {
    fn GetFamilyNames2(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames2(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteLocalizedStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows::core::RuntimeName for IDWriteFontFace6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: isize>() -> IDWriteFontFace6_Vtbl {
        unsafe extern "system" fn GetFamilyNames2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFamilyNames2(::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFaceNames2(::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFace5_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFamilyNames2: GetFamilyNames2::<Identity, Impl, OFFSET>,
            GetFaceNames2: GetFaceNames2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace6 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFace5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReference_Impl: Sized {
    fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace3>;
    fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace3>;
    fn Equals(&self, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>) -> super::super::Foundation::BOOL;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn GetFontFile(&self) -> ::windows::core::Result<IDWriteFontFile>;
    fn GetLocalFileSize(&self) -> u64;
    fn GetFileSize(&self) -> u64;
    fn GetFileTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn EnqueueFontDownloadRequest(&self) -> ::windows::core::Result<()>;
    fn EnqueueCharacterDownloadRequest(&self, characters: &::windows::core::PCWSTR, charactercount: u32) -> ::windows::core::Result<()>;
    fn EnqueueGlyphDownloadRequest(&self, glyphindices: *const u16, glyphcount: u32) -> ::windows::core::Result<()>;
    fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontFaceReference {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFaceReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>() -> IDWriteFontFaceReference_Vtbl {
        unsafe extern "system" fn CreateFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceWithSimulations(::core::mem::transmute_copy(&fontfacesimulationflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Equals(::windows::core::from_raw_borrowed(&fontfacereference))
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFaceIndex()
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSimulations()
        }
        unsafe extern "system" fn GetFontFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocalFileSize()
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileSize()
        }
        unsafe extern "system" fn GetFileTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocality()
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueFontDownloadRequest().into()
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: ::windows::core::PCWSTR, charactercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueCharacterDownloadRequest(::core::mem::transmute(&characters), ::core::mem::transmute_copy(&charactercount)).into()
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueGlyphDownloadRequest(::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount)).into()
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueFileFragmentDownloadRequest(::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateFontFaceWithSimulations: CreateFontFaceWithSimulations::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            GetFontFile: GetFontFile::<Identity, Impl, OFFSET>,
            GetLocalFileSize: GetLocalFileSize::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetFileTime: GetFileTime::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
            EnqueueFontDownloadRequest: EnqueueFontDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueCharacterDownloadRequest: EnqueueCharacterDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueGlyphDownloadRequest: EnqueueGlyphDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueFileFragmentDownloadRequest: EnqueueFileFragmentDownloadRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReference1_Impl: Sized + IDWriteFontFaceReference_Impl {
    fn CreateFontFace2(&self) -> ::windows::core::Result<IDWriteFontFace5>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontFaceReference1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFaceReference1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: isize>() -> IDWriteFontFaceReference1_Vtbl {
        unsafe extern "system" fn CreateFontFace2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValueCount()
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        Self {
            base__: IDWriteFontFaceReference_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFontFace2: CreateFontFace2::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFaceReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFallback_Impl: Sized {
    fn MapCharacters(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: ::core::option::Option<&IDWriteFontCollection>, basefamilyname: &::windows::core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::core::option::Option<IDWriteFont>, scale: *mut f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteFontFallback {}
impl IDWriteFontFallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallback_Impl, const OFFSET: isize>() -> IDWriteFontFallback_Vtbl {
        unsafe extern "system" fn MapCharacters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows::core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut ::core::ffi::c_void, scale: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapCharacters(
                ::windows::core::from_raw_borrowed(&analysissource),
                ::core::mem::transmute_copy(&textposition),
                ::core::mem::transmute_copy(&textlength),
                ::windows::core::from_raw_borrowed(&basefontcollection),
                ::core::mem::transmute(&basefamilyname),
                ::core::mem::transmute_copy(&baseweight),
                ::core::mem::transmute_copy(&basestyle),
                ::core::mem::transmute_copy(&basestretch),
                ::core::mem::transmute_copy(&mappedlength),
                ::core::mem::transmute_copy(&mappedfont),
                ::core::mem::transmute_copy(&scale),
            )
            .into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MapCharacters: MapCharacters::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFallback1_Impl: Sized + IDWriteFontFallback_Impl {
    fn MapCharacters2(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: ::core::option::Option<&IDWriteFontCollection>, basefamilyname: &::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut ::core::option::Option<IDWriteFontFace5>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteFontFallback1 {}
impl IDWriteFontFallback1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallback1_Impl, const OFFSET: isize>() -> IDWriteFontFallback1_Vtbl {
        unsafe extern "system" fn MapCharacters2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallback1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapCharacters2(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&basefontcollection), ::core::mem::transmute(&basefamilyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&mappedlength), ::core::mem::transmute_copy(&scale), ::core::mem::transmute_copy(&mappedfontface))
                .into()
        }
        Self { base__: IDWriteFontFallback_Vtbl::new::<Identity, Impl, OFFSET>(), MapCharacters2: MapCharacters2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallback1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFallbackBuilder_Impl: Sized {
    fn AddMapping(&self, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: ::core::option::Option<&IDWriteFontCollection>, localename: &::windows::core::PCWSTR, basefamilyname: &::windows::core::PCWSTR, scale: f32) -> ::windows::core::Result<()>;
    fn AddMappings(&self, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows::core::Result<()>;
    fn CreateFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback>;
}
impl ::windows::core::RuntimeName for IDWriteFontFallbackBuilder {}
impl IDWriteFontFallbackBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>() -> IDWriteFontFallbackBuilder_Vtbl {
        unsafe extern "system" fn AddMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut ::core::ffi::c_void, localename: ::windows::core::PCWSTR, basefamilyname: ::windows::core::PCWSTR, scale: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMapping(::core::mem::transmute_copy(&ranges), ::core::mem::transmute_copy(&rangescount), ::core::mem::transmute_copy(&targetfamilynames), ::core::mem::transmute_copy(&targetfamilynamescount), ::windows::core::from_raw_borrowed(&fontcollection), ::core::mem::transmute(&localename), ::core::mem::transmute(&basefamilyname), ::core::mem::transmute_copy(&scale)).into()
        }
        unsafe extern "system" fn AddMappings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMappings(::windows::core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn CreateFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMapping: AddMapping::<Identity, Impl, OFFSET>,
            AddMappings: AddMappings::<Identity, Impl, OFFSET>,
            CreateFontFallback: CreateFontFallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallbackBuilder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFamily_Impl: Sized + IDWriteFontList_Impl {
    fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFont>;
    fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontList>;
}
impl ::windows::core::RuntimeName for IDWriteFontFamily {}
impl IDWriteFontFamily_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: isize>() -> IDWriteFontFamily_Vtbl {
        unsafe extern "system" fn GetFamilyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstMatchingFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstMatchingFont(::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&stretch), ::core::mem::transmute_copy(&style)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfont, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts(::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&stretch), ::core::mem::transmute_copy(&style)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFamilyNames: GetFamilyNames::<Identity, Impl, OFFSET>,
            GetFirstMatchingFont: GetFirstMatchingFont::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFamily1_Impl: Sized + IDWriteFontFamily_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference>;
}
impl ::windows::core::RuntimeName for IDWriteFontFamily1 {}
impl IDWriteFontFamily1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: isize>() -> IDWriteFontFamily1_Vtbl {
        unsafe extern "system" fn GetFontLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontLocality(::core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFont2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFont2(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFamily_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFamily as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFamily2_Impl: Sized + IDWriteFontFamily1_Impl {
    fn GetMatchingFonts2(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontList2>;
    fn GetFontSet(&self) -> ::windows::core::Result<IDWriteFontSet1>;
}
impl ::windows::core::RuntimeName for IDWriteFontFamily2 {}
impl IDWriteFontFamily2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: isize>() -> IDWriteFontFamily2_Vtbl {
        unsafe extern "system" fn GetMatchingFonts2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts2(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFamily1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMatchingFonts2: GetMatchingFonts2::<Identity, Impl, OFFSET>,
            GetFontSet: GetFontSet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFamily as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFamily1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFile_Impl: Sized {
    fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows::core::Result<()>;
    fn GetLoader(&self) -> ::windows::core::Result<IDWriteFontFileLoader>;
    fn Analyze(&self, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontFile {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: isize>() -> IDWriteFontFile_Vtbl {
        unsafe extern "system" fn GetReferenceKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReferenceKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)).into()
        }
        unsafe extern "system" fn GetLoader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLoader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfileloader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Analyze<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Analyze(::core::mem::transmute_copy(&issupportedfonttype), ::core::mem::transmute_copy(&fontfiletype), ::core::mem::transmute_copy(&fontfacetype), ::core::mem::transmute_copy(&numberoffaces)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReferenceKey: GetReferenceKey::<Identity, Impl, OFFSET>,
            GetLoader: GetLoader::<Identity, Impl, OFFSET>,
            Analyze: Analyze::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFileEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentFontFile(&self) -> ::windows::core::Result<IDWriteFontFile>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontFileEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFileEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>() -> IDWriteFontFileEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrentfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFontFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentFontFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            GetCurrentFontFile: GetCurrentFontFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileEnumerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFileLoader_Impl: Sized {
    fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<IDWriteFontFileStream>;
}
impl ::windows::core::RuntimeName for IDWriteFontFileLoader {}
impl IDWriteFontFileLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileLoader_Impl, const OFFSET: isize>() -> IDWriteFontFileLoader_Vtbl {
        unsafe extern "system" fn CreateStreamFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStreamFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfilestream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateStreamFromKey: CreateStreamFromKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileLoader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontFileStream_Impl: Sized {
    fn ReadFileFragment(&self, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseFileFragment(&self, fragmentcontext: *mut ::core::ffi::c_void);
    fn GetFileSize(&self) -> ::windows::core::Result<u64>;
    fn GetLastWriteTime(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDWriteFontFileStream {}
impl IDWriteFontFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: isize>() -> IDWriteFontFileStream_Vtbl {
        unsafe extern "system" fn ReadFileFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadFileFragment(::core::mem::transmute_copy(&fragmentstart), ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize), ::core::mem::transmute_copy(&fragmentcontext)).into()
        }
        unsafe extern "system" fn ReleaseFileFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentcontext: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFileFragment(::core::mem::transmute_copy(&fragmentcontext))
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastWriteTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadFileFragment: ReadFileFragment::<Identity, Impl, OFFSET>,
            ReleaseFileFragment: ReleaseFileFragment::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetLastWriteTime: GetLastWriteTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontList_Impl: Sized {
    fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection>;
    fn GetFontCount(&self) -> u32;
    fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont>;
}
impl ::windows::core::RuntimeName for IDWriteFontList {}
impl IDWriteFontList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: isize>() -> IDWriteFontList_Vtbl {
        unsafe extern "system" fn GetFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontCount()
        }
        unsafe extern "system" fn GetFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFont(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCollection: GetFontCollection::<Identity, Impl, OFFSET>,
            GetFontCount: GetFontCount::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontList1_Impl: Sized + IDWriteFontList_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference>;
}
impl ::windows::core::RuntimeName for IDWriteFontList1 {}
impl IDWriteFontList1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: isize>() -> IDWriteFontList1_Vtbl {
        unsafe extern "system" fn GetFontLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontLocality(::core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFont2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFont2(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontList2_Impl: Sized + IDWriteFontList1_Impl {
    fn GetFontSet(&self) -> ::windows::core::Result<IDWriteFontSet1>;
}
impl ::windows::core::RuntimeName for IDWriteFontList2 {}
impl IDWriteFontList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList2_Impl, const OFFSET: isize>() -> IDWriteFontList2_Vtbl {
        unsafe extern "system" fn GetFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteFontList1_Vtbl::new::<Identity, Impl, OFFSET>(), GetFontSet: GetFontSet::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontList1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontResource_Impl: Sized {
    fn GetFontFile(&self) -> ::windows::core::Result<IDWriteFontFile>;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetFontAxisCount(&self) -> u32;
    fn GetDefaultFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<()>;
    fn GetFontAxisRanges(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows::core::Result<()>;
    fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES;
    fn GetAxisNames(&self, axisindex: u32) -> ::windows::core::Result<IDWriteLocalizedStrings>;
    fn GetAxisValueNameCount(&self, axisindex: u32) -> u32;
    fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows::core::Result<()>;
    fn HasVariations(&self) -> super::super::Foundation::BOOL;
    fn CreateFontFace(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontFace5>;
    fn CreateFontFaceReference(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontFaceReference1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontResource {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>() -> IDWriteFontResource_Vtbl {
        unsafe extern "system" fn GetFontFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFaceIndex()
        }
        unsafe extern "system" fn GetFontAxisCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisCount()
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisRanges(::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontAxisAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisAttributes(::core::mem::transmute_copy(&axisindex))
        }
        unsafe extern "system" fn GetAxisNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAxisNames(::core::mem::transmute_copy(&axisindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisValueNameCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAxisValueNameCount(::core::mem::transmute_copy(&axisindex))
        }
        unsafe extern "system" fn GetAxisValueNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAxisValueNames(::core::mem::transmute_copy(&axisindex), ::core::mem::transmute_copy(&axisvalueindex), ::core::mem::transmute_copy(&fontaxisrange), ::core::mem::transmute_copy(&names)).into()
        }
        unsafe extern "system" fn HasVariations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasVariations()
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace(::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceReference(::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFile: GetFontFile::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetFontAxisCount: GetFontAxisCount::<Identity, Impl, OFFSET>,
            GetDefaultFontAxisValues: GetDefaultFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, Impl, OFFSET>,
            GetFontAxisAttributes: GetFontAxisAttributes::<Identity, Impl, OFFSET>,
            GetAxisNames: GetAxisNames::<Identity, Impl, OFFSET>,
            GetAxisValueNameCount: GetAxisValueNameCount::<Identity, Impl, OFFSET>,
            GetAxisValueNames: GetAxisValueNames::<Identity, Impl, OFFSET>,
            HasVariations: HasVariations::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontResource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet_Impl: Sized {
    fn GetFontCount(&self) -> u32;
    fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference>;
    fn FindFontFaceReference(&self, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FindFontFace(&self, fontface: ::core::option::Option<&IDWriteFontFace>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows::core::Result<IDWriteStringList>;
    fn GetPropertyValues2(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: &::windows::core::PCWSTR) -> ::windows::core::Result<IDWriteStringList>;
    fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows::core::Result<()>;
    fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows::core::Result<u32>;
    fn GetMatchingFonts(&self, familyname: &::windows::core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontSet>;
    fn GetMatchingFonts2(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::Result<IDWriteFontSet>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontSet {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>() -> IDWriteFontSet_Vtbl {
        unsafe extern "system" fn GetFontCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontCount()
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindFontFaceReference(::windows::core::from_raw_borrowed(&fontfacereference), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn FindFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindFontFace(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetPropertyValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyValues(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(values, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: ::windows::core::PCWSTR, values: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyValues2(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&preferredlocalenames)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(values, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyValues3(::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&exists), ::core::mem::transmute_copy(&values)).into()
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyOccurrenceCount(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyoccurrencecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: ::windows::core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts(::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts2(::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCount: GetFontCount::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            FindFontFaceReference: FindFontFaceReference::<Identity, Impl, OFFSET>,
            FindFontFace: FindFontFace::<Identity, Impl, OFFSET>,
            GetPropertyValues: GetPropertyValues::<Identity, Impl, OFFSET>,
            GetPropertyValues2: GetPropertyValues2::<Identity, Impl, OFFSET>,
            GetPropertyValues3: GetPropertyValues3::<Identity, Impl, OFFSET>,
            GetPropertyOccurrenceCount: GetPropertyOccurrenceCount::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
            GetMatchingFonts2: GetMatchingFonts2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet1_Impl: Sized + IDWriteFontSet_Impl {
    fn GetMatchingFonts3(&self, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetFirstFontResources(&self) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts(&self, indices: *const u32, indexcount: u32) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts2(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts3(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL) -> ::windows::core::Result<IDWriteFontSet1>;
    fn GetFilteredFontIndices(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilteredFontIndices2(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFontAxisRanges2(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFontFaceReference2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontResource>;
    fn CreateFontFace(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFace5>;
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontSet1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>() -> IDWriteFontSet1_Vtbl {
        unsafe extern "system" fn GetMatchingFonts3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts3(::core::mem::transmute_copy(&fontproperty), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstFontResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstFontResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilteredFonts(::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&indexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilteredFonts2(::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&selectanyrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilteredFonts3(::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount), ::core::mem::transmute_copy(&selectanyproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilteredFontIndices(::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&selectanyrange), ::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&maxindexcount), ::core::mem::transmute_copy(&actualindexcount)).into()
        }
        unsafe extern "system" fn GetFilteredFontIndices2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilteredFontIndices2(::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount), ::core::mem::transmute_copy(&selectanyproperty), ::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&maxindexcount), ::core::mem::transmute_copy(&actualindexcount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisRanges(::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&maxfontaxisrangecount), ::core::mem::transmute_copy(&actualfontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisRanges2(::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&maxfontaxisrangecount), ::core::mem::transmute_copy(&actualfontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontFaceReference2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceReference2(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontResource(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFace(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontLocality(::core::mem::transmute_copy(&listindex))
        }
        Self {
            base__: IDWriteFontSet_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMatchingFonts3: GetMatchingFonts3::<Identity, Impl, OFFSET>,
            GetFirstFontResources: GetFirstFontResources::<Identity, Impl, OFFSET>,
            GetFilteredFonts: GetFilteredFonts::<Identity, Impl, OFFSET>,
            GetFilteredFonts2: GetFilteredFonts2::<Identity, Impl, OFFSET>,
            GetFilteredFonts3: GetFilteredFonts3::<Identity, Impl, OFFSET>,
            GetFilteredFontIndices: GetFilteredFontIndices::<Identity, Impl, OFFSET>,
            GetFilteredFontIndices2: GetFilteredFontIndices2::<Identity, Impl, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, Impl, OFFSET>,
            GetFontAxisRanges2: GetFontAxisRanges2::<Identity, Impl, OFFSET>,
            GetFontFaceReference2: GetFontFaceReference2::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet2_Impl: Sized + IDWriteFontSet1_Impl {
    fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontSet2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet2_Impl, const OFFSET: isize>() -> IDWriteFontSet2_Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExpirationEvent()
        }
        Self { base__: IDWriteFontSet1_Vtbl::new::<Identity, Impl, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet3_Impl: Sized + IDWriteFontSet2_Impl {
    fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE;
    fn GetFontSourceNameLength(&self, listindex: u32) -> u32;
    fn GetFontSourceName(&self, listindex: u32, stringbuffer: ::windows::core::PWSTR, stringbuffersize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontSet3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: isize>() -> IDWriteFontSet3_Vtbl {
        unsafe extern "system" fn GetFontSourceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSourceType(::core::mem::transmute_copy(&fontindex))
        }
        unsafe extern "system" fn GetFontSourceNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSourceNameLength(::core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFontSourceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows::core::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSourceName(::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&stringbuffersize)).into()
        }
        Self {
            base__: IDWriteFontSet2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFontSourceType: GetFontSourceType::<Identity, Impl, OFFSET>,
            GetFontSourceNameLength: GetFontSourceNameLength::<Identity, Impl, OFFSET>,
            GetFontSourceName: GetFontSourceName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet4_Impl: Sized + IDWriteFontSet3_Impl {
    fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32;
    fn GetMatchingFonts4(&self, familyname: &::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontSet4>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteFontSet4 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: isize>() -> IDWriteFontSet4_Vtbl {
        unsafe extern "system" fn ConvertWeightStretchStyleToFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertWeightStretchStyleToFontAxisValues(::core::mem::transmute_copy(&inputaxisvalues), ::core::mem::transmute_copy(&inputaxiscount), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute_copy(&outputaxisvalues))
        }
        unsafe extern "system" fn GetMatchingFonts4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: ::windows::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFonts4(::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&allowedsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontSet3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConvertWeightStretchStyleToFontAxisValues: ConvertWeightStretchStyleToFontAxisValues::<Identity, Impl, OFFSET>,
            GetMatchingFonts4: GetMatchingFonts4::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSet3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontSetBuilder_Impl: Sized {
    fn AddFontFaceReference(&self, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::Result<()>;
    fn AddFontFaceReference2(&self, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>) -> ::windows::core::Result<()>;
    fn AddFontSet(&self, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows::core::Result<()>;
    fn CreateFontSet(&self) -> ::windows::core::Result<IDWriteFontSet>;
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder {}
impl IDWriteFontSetBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: isize>() -> IDWriteFontSetBuilder_Vtbl {
        unsafe extern "system" fn AddFontFaceReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontFaceReference(::windows::core::from_raw_borrowed(&fontfacereference), ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddFontFaceReference2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontFaceReference2(::windows::core::from_raw_borrowed(&fontfacereference)).into()
        }
        unsafe extern "system" fn AddFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontSet(::windows::core::from_raw_borrowed(&fontset)).into()
        }
        unsafe extern "system" fn CreateFontSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFaceReference: AddFontFaceReference::<Identity, Impl, OFFSET>,
            AddFontFaceReference2: AddFontFaceReference2::<Identity, Impl, OFFSET>,
            AddFontSet: AddFontSet::<Identity, Impl, OFFSET>,
            CreateFontSet: CreateFontSet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontSetBuilder1_Impl: Sized + IDWriteFontSetBuilder_Impl {
    fn AddFontFile(&self, fontfile: ::core::option::Option<&IDWriteFontFile>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder1 {}
impl IDWriteFontSetBuilder1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder1_Impl, const OFFSET: isize>() -> IDWriteFontSetBuilder1_Vtbl {
        unsafe extern "system" fn AddFontFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontFile(::windows::core::from_raw_borrowed(&fontfile)).into()
        }
        Self { base__: IDWriteFontSetBuilder_Vtbl::new::<Identity, Impl, OFFSET>(), AddFontFile: AddFontFile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSetBuilder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteFontSetBuilder2_Impl: Sized + IDWriteFontSetBuilder1_Impl {
    fn AddFont(&self, fontfile: ::core::option::Option<&IDWriteFontFile>, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::Result<()>;
    fn AddFontFile2(&self, filepath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder2 {}
impl IDWriteFontSetBuilder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>() -> IDWriteFontSetBuilder2_Vtbl {
        unsafe extern "system" fn AddFont<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFont(::windows::core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&fontfaceindex), ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddFontFile2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontFile2(::core::mem::transmute(&filepath)).into()
        }
        Self {
            base__: IDWriteFontSetBuilder1_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddFont: AddFont::<Identity, Impl, OFFSET>,
            AddFontFile2: AddFontFile2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSetBuilder as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontSetBuilder1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop_Impl: Sized {
    fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> ::windows::core::Result<IDWriteFont>;
    fn ConvertFontToLOGFONT(&self, font: ::core::option::Option<&IDWriteFont>, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ConvertFontFaceToLOGFONT(&self, font: ::core::option::Option<&IDWriteFontFace>, logfont: *mut super::Gdi::LOGFONTW) -> ::windows::core::Result<()>;
    fn CreateFontFaceFromHdc(&self, hdc: super::Gdi::HDC) -> ::windows::core::Result<IDWriteFontFace>;
    fn CreateBitmapRenderTarget(&self, hdc: super::Gdi::HDC, width: u32, height: u32) -> ::windows::core::Result<IDWriteBitmapRenderTarget>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteGdiInterop {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteGdiInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>() -> IDWriteGdiInterop_Vtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFromLOGFONT(::core::mem::transmute_copy(&logfont)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertFontToLOGFONT(::windows::core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&logfont), ::core::mem::transmute_copy(&issystemfont)).into()
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertFontFaceToLOGFONT(::windows::core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&logfont)).into()
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFaceFromHdc(::core::mem::transmute_copy(&hdc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBitmapRenderTarget(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rendertarget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, Impl, OFFSET>,
            ConvertFontToLOGFONT: ConvertFontToLOGFONT::<Identity, Impl, OFFSET>,
            ConvertFontFaceToLOGFONT: ConvertFontFaceToLOGFONT::<Identity, Impl, OFFSET>,
            CreateFontFaceFromHdc: CreateFontFaceFromHdc::<Identity, Impl, OFFSET>,
            CreateBitmapRenderTarget: CreateBitmapRenderTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGdiInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop1_Impl: Sized + IDWriteGdiInterop_Impl {
    fn CreateFontFromLOGFONT2(&self, logfont: *const super::Gdi::LOGFONTW, fontcollection: ::core::option::Option<&IDWriteFontCollection>) -> ::windows::core::Result<IDWriteFont>;
    fn GetFontSignature(&self, fontface: ::core::option::Option<&IDWriteFontFace>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::Result<()>;
    fn GetFontSignature2(&self, font: ::core::option::Option<&IDWriteFont>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::Result<()>;
    fn GetMatchingFontsByLOGFONT(&self, logfont: *const super::Gdi::LOGFONTA, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows::core::Result<IDWriteFontSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDWriteGdiInterop1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl IDWriteGdiInterop1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: isize>() -> IDWriteGdiInterop1_Vtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFromLOGFONT2(::core::mem::transmute_copy(&logfont), ::windows::core::from_raw_borrowed(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSignature(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&fontsignature)).into()
        }
        unsafe extern "system" fn GetFontSignature2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSignature2(::windows::core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&fontsignature)).into()
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: *mut ::core::ffi::c_void, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingFontsByLOGFONT(::core::mem::transmute_copy(&logfont), ::windows::core::from_raw_borrowed(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteGdiInterop_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFontFromLOGFONT2: CreateFontFromLOGFONT2::<Identity, Impl, OFFSET>,
            GetFontSignature: GetFontSignature::<Identity, Impl, OFFSET>,
            GetFontSignature2: GetFontSignature2::<Identity, Impl, OFFSET>,
            GetMatchingFontsByLOGFONT: GetMatchingFontsByLOGFONT::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGdiInterop1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteGdiInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteGlyphRunAnalysis_Impl: Sized {
    fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows::core::Result<()>;
    fn GetAlphaBlendParams(&self, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteGlyphRunAnalysis {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteGlyphRunAnalysis_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>() -> IDWriteGlyphRunAnalysis_Vtbl {
        unsafe extern "system" fn GetAlphaTextureBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAlphaTextureBounds(::core::mem::transmute_copy(&texturetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(texturebounds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAlphaTexture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAlphaTexture(::core::mem::transmute_copy(&texturetype), ::core::mem::transmute_copy(&texturebounds), ::core::mem::transmute_copy(&alphavalues), ::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetAlphaBlendParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: *mut ::core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAlphaBlendParams(::windows::core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&blendgamma), ::core::mem::transmute_copy(&blendenhancedcontrast), ::core::mem::transmute_copy(&blendcleartypelevel)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAlphaTextureBounds: GetAlphaTextureBounds::<Identity, Impl, OFFSET>,
            CreateAlphaTexture: CreateAlphaTexture::<Identity, Impl, OFFSET>,
            GetAlphaBlendParams: GetAlphaBlendParams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGlyphRunAnalysis as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteInMemoryFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn CreateInMemoryFontFileReference(&self, factory: ::core::option::Option<&IDWriteFactory>, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<IDWriteFontFile>;
    fn GetFileCount(&self) -> u32;
}
impl ::windows::core::RuntimeName for IDWriteInMemoryFontFileLoader {}
impl IDWriteInMemoryFontFileLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>() -> IDWriteInMemoryFontFileLoader_Vtbl {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInMemoryFontFileReference(::windows::core::from_raw_borrowed(&factory), ::core::mem::transmute_copy(&fontdata), ::core::mem::transmute_copy(&fontdatasize), ::windows::core::from_raw_borrowed(&ownerobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileCount()
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateInMemoryFontFileReference: CreateInMemoryFontFileReference::<Identity, Impl, OFFSET>,
            GetFileCount: GetFileCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteInMemoryFontFileLoader as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFileLoader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteInlineObject_Impl: Sized {
    fn Draw(&self, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::core::option::Option<&IDWriteTextRenderer>, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetMetrics(&self) -> ::windows::core::Result<DWRITE_INLINE_OBJECT_METRICS>;
    fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteInlineObject {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteInlineObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: isize>() -> IDWriteInlineObject_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&clientdrawingcontext), ::windows::core::from_raw_borrowed(&renderer), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metrics, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOverhangMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overhangs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakConditions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakConditions(::core::mem::transmute_copy(&breakconditionbefore), ::core::mem::transmute_copy(&breakconditionafter)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, Impl, OFFSET>,
            GetBreakConditions: GetBreakConditions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteInlineObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<u32>;
    fn GetFilePathFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows::core::PWSTR, filepathsize: u32) -> ::windows::core::Result<()>;
    fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteLocalFontFileLoader {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteLocalFontFileLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>() -> IDWriteLocalFontFileLoader_Vtbl {
        unsafe extern "system" fn GetFilePathLengthFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilePathLengthFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepathlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilePathFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows::core::PWSTR, filepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFilePathFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize), ::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&filepathsize)).into()
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastWriteTimeFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFilePathLengthFromKey: GetFilePathLengthFromKey::<Identity, Impl, OFFSET>,
            GetFilePathFromKey: GetFilePathFromKey::<Identity, Impl, OFFSET>,
            GetLastWriteTimeFromKey: GetLastWriteTimeFromKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteLocalFontFileLoader as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFileLoader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalizedStrings_Impl: Sized {
    fn GetCount(&self) -> u32;
    fn FindLocaleName(&self, localename: &::windows::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocaleNameLength(&self, index: u32) -> ::windows::core::Result<u32>;
    fn GetLocaleName(&self, index: u32, localename: ::windows::core::PWSTR, size: u32) -> ::windows::core::Result<()>;
    fn GetStringLength(&self, index: u32) -> ::windows::core::Result<u32>;
    fn GetString(&self, index: u32, stringbuffer: ::windows::core::PWSTR, size: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteLocalizedStrings {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteLocalizedStrings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>() -> IDWriteLocalizedStrings_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount()
        }
        unsafe extern "system" fn FindLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: ::windows::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindLocaleName(::core::mem::transmute(&localename), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocaleNameLength(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, localename: ::windows::core::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleName(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringLength(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stringbuffer: ::windows::core::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetString(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&size)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            FindLocaleName: FindLocaleName::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteLocalizedStrings as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteNumberSubstitution_Impl: Sized {}
impl ::windows::core::RuntimeName for IDWriteNumberSubstitution {}
impl IDWriteNumberSubstitution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteNumberSubstitution_Impl, const OFFSET: isize>() -> IDWriteNumberSubstitution_Vtbl {
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWritePixelSnapping_Impl: Sized {
    fn IsPixelSnappingDisabled(&self, clientdrawingcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentTransform(&self, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()>;
    fn GetPixelsPerDip(&self, clientdrawingcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWritePixelSnapping {}
#[cfg(feature = "Win32_Foundation")]
impl IDWritePixelSnapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: isize>() -> IDWritePixelSnapping_Vtbl {
        unsafe extern "system" fn IsPixelSnappingDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPixelSnappingDisabled(::core::mem::transmute_copy(&clientdrawingcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdisabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentTransform(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPixelsPerDip(::core::mem::transmute_copy(&clientdrawingcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pixelsperdip, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPixelSnappingDisabled: IsPixelSnappingDisabled::<Identity, Impl, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, Impl, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWritePixelSnapping as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteRemoteFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<IDWriteRemoteFontFileStream>;
    fn GetLocalityFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<DWRITE_LOCALITY>;
    fn CreateFontFileReferenceFromUrl(&self, factory: ::core::option::Option<&IDWriteFactory>, baseurl: &::windows::core::PCWSTR, fontfileurl: &::windows::core::PCWSTR) -> ::windows::core::Result<IDWriteFontFile>;
}
impl ::windows::core::RuntimeName for IDWriteRemoteFontFileLoader {}
impl IDWriteRemoteFontFileLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>() -> IDWriteRemoteFontFileLoader_Vtbl {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRemoteStreamFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfilestream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalityFromKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalityFromKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(locality, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, baseurl: ::windows::core::PCWSTR, fontfileurl: ::windows::core::PCWSTR, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontFileReferenceFromUrl(::windows::core::from_raw_borrowed(&factory), ::core::mem::transmute(&baseurl), ::core::mem::transmute(&fontfileurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateRemoteStreamFromKey: CreateRemoteStreamFromKey::<Identity, Impl, OFFSET>,
            GetLocalityFromKey: GetLocalityFromKey::<Identity, Impl, OFFSET>,
            CreateFontFileReferenceFromUrl: CreateFontFileReferenceFromUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileLoader as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFileLoader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteRemoteFontFileStream_Impl: Sized + IDWriteFontFileStream_Impl {
    fn GetLocalFileSize(&self) -> ::windows::core::Result<u64>;
    fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows::core::Result<()>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn BeginDownload(&self, downloadoperationid: *const ::windows::core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32) -> ::windows::core::Result<IDWriteAsyncResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteRemoteFontFileStream {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteRemoteFontFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>() -> IDWriteRemoteFontFileStream_Vtbl {
        unsafe extern "system" fn GetLocalFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localfilesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localfilesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileFragmentLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileFragmentLocality(::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize), ::core::mem::transmute_copy(&islocal), ::core::mem::transmute_copy(&partialsize)).into()
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocality()
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadoperationid: *const ::windows::core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDownload(::core::mem::transmute_copy(&downloadoperationid), ::core::mem::transmute_copy(&filefragments), ::core::mem::transmute_copy(&fragmentcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(asyncresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLocalFileSize: GetLocalFileSize::<Identity, Impl, OFFSET>,
            GetFileFragmentLocality: GetFileFragmentLocality::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileStream as ::windows::core::ComInterface>::IID || iid == &<IDWriteFontFileStream as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteRenderingParams_Impl: Sized {
    fn GetGamma(&self) -> f32;
    fn GetEnhancedContrast(&self) -> f32;
    fn GetClearTypeLevel(&self) -> f32;
    fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY;
    fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE;
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams {}
impl IDWriteRenderingParams_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>() -> IDWriteRenderingParams_Vtbl {
        unsafe extern "system" fn GetGamma<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGamma()
        }
        unsafe extern "system" fn GetEnhancedContrast<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEnhancedContrast()
        }
        unsafe extern "system" fn GetClearTypeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClearTypeLevel()
        }
        unsafe extern "system" fn GetPixelGeometry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPixelGeometry()
        }
        unsafe extern "system" fn GetRenderingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRenderingMode()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGamma: GetGamma::<Identity, Impl, OFFSET>,
            GetEnhancedContrast: GetEnhancedContrast::<Identity, Impl, OFFSET>,
            GetClearTypeLevel: GetClearTypeLevel::<Identity, Impl, OFFSET>,
            GetPixelGeometry: GetPixelGeometry::<Identity, Impl, OFFSET>,
            GetRenderingMode: GetRenderingMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteRenderingParams1_Impl: Sized + IDWriteRenderingParams_Impl {
    fn GetGrayscaleEnhancedContrast(&self) -> f32;
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams1 {}
impl IDWriteRenderingParams1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams1_Impl, const OFFSET: isize>() -> IDWriteRenderingParams1_Vtbl {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGrayscaleEnhancedContrast()
        }
        Self {
            base__: IDWriteRenderingParams_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGrayscaleEnhancedContrast: GetGrayscaleEnhancedContrast::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteRenderingParams2_Impl: Sized + IDWriteRenderingParams1_Impl {
    fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE;
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams2 {}
impl IDWriteRenderingParams2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams2_Impl, const OFFSET: isize>() -> IDWriteRenderingParams2_Vtbl {
        unsafe extern "system" fn GetGridFitMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GRID_FIT_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGridFitMode()
        }
        Self { base__: IDWriteRenderingParams1_Vtbl::new::<Identity, Impl, OFFSET>(), GetGridFitMode: GetGridFitMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteRenderingParams3_Impl: Sized + IDWriteRenderingParams2_Impl {
    fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1;
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams3 {}
impl IDWriteRenderingParams3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams3_Impl, const OFFSET: isize>() -> IDWriteRenderingParams3_Vtbl {
        unsafe extern "system" fn GetRenderingMode1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteRenderingParams3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE1 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRenderingMode1()
        }
        Self { base__: IDWriteRenderingParams2_Vtbl::new::<Identity, Impl, OFFSET>(), GetRenderingMode1: GetRenderingMode1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteRenderingParams2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteStringList_Impl: Sized {
    fn GetCount(&self) -> u32;
    fn GetLocaleNameLength(&self, listindex: u32) -> ::windows::core::Result<u32>;
    fn GetLocaleName(&self, listindex: u32, localename: ::windows::core::PWSTR, size: u32) -> ::windows::core::Result<()>;
    fn GetStringLength(&self, listindex: u32) -> ::windows::core::Result<u32>;
    fn GetString(&self, listindex: u32, stringbuffer: ::windows::core::PWSTR, stringbuffersize: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteStringList {}
impl IDWriteStringList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>() -> IDWriteStringList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount()
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocaleNameLength(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, localename: ::windows::core::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleName(::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringLength(::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows::core::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetString(::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&stringbuffersize)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteStringList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteTextAnalysisSink_Impl: Sized {
    fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows::core::Result<()>;
    fn SetLineBreakpoints(&self, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows::core::Result<()>;
    fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows::core::Result<()>;
    fn SetNumberSubstitution(&self, textposition: u32, textlength: u32, numbersubstitution: ::core::option::Option<&IDWriteNumberSubstitution>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSink {}
impl IDWriteTextAnalysisSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSink_Vtbl {
        unsafe extern "system" fn SetScriptAnalysis<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScriptAnalysis(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&scriptanalysis)).into()
        }
        unsafe extern "system" fn SetLineBreakpoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineBreakpoints(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&linebreakpoints)).into()
        }
        unsafe extern "system" fn SetBidiLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBidiLevel(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&explicitlevel), ::core::mem::transmute_copy(&resolvedlevel)).into()
        }
        unsafe extern "system" fn SetNumberSubstitution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNumberSubstitution(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&numbersubstitution)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScriptAnalysis: SetScriptAnalysis::<Identity, Impl, OFFSET>,
            SetLineBreakpoints: SetLineBreakpoints::<Identity, Impl, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, Impl, OFFSET>,
            SetNumberSubstitution: SetNumberSubstitution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalysisSink1_Impl: Sized + IDWriteTextAnalysisSink_Impl {
    fn SetGlyphOrientation(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSink1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalysisSink1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink1_Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSink1_Vtbl {
        unsafe extern "system" fn SetGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSink1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGlyphOrientation(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&adjustedbidilevel), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft)).into()
        }
        Self { base__: IDWriteTextAnalysisSink_Vtbl::new::<Identity, Impl, OFFSET>(), SetGlyphOrientation: SetGlyphOrientation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextAnalysisSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteTextAnalysisSource_Impl: Sized {
    fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows::core::Result<()>;
    fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::core::option::Option<IDWriteNumberSubstitution>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSource {}
impl IDWriteTextAnalysisSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSource_Vtbl {
        unsafe extern "system" fn GetTextAtPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextAtPosition(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)).into()
        }
        unsafe extern "system" fn GetTextBeforePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextBeforePosition(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)).into()
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParagraphReadingDirection()
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleName(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&localename)).into()
        }
        unsafe extern "system" fn GetNumberSubstitution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumberSubstitution(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&numbersubstitution)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTextAtPosition: GetTextAtPosition::<Identity, Impl, OFFSET>,
            GetTextBeforePosition: GetTextBeforePosition::<Identity, Impl, OFFSET>,
            GetParagraphReadingDirection: GetParagraphReadingDirection::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetNumberSubstitution: GetNumberSubstitution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteTextAnalysisSource1_Impl: Sized + IDWriteTextAnalysisSource_Impl {
    fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSource1 {}
impl IDWriteTextAnalysisSource1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource1_Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSource1_Vtbl {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalysisSource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalGlyphOrientation(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphorientation), ::core::mem::transmute_copy(&bidilevel)).into()
        }
        Self {
            base__: IDWriteTextAnalysisSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextAnalysisSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer_Impl: Sized {
    fn AnalyzeScript(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows::core::Result<()>;
    fn AnalyzeBidi(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows::core::Result<()>;
    fn AnalyzeNumberSubstitution(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows::core::Result<()>;
    fn AnalyzeLineBreakpoints(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows::core::Result<()>;
    fn GetGlyphs(&self, textstring: &::windows::core::PCWSTR, textlength: u32, fontface: ::core::option::Option<&IDWriteFontFace>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &::windows::core::PCWSTR, numbersubstitution: ::core::option::Option<&IDWriteNumberSubstitution>, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetGlyphPlacements(&self, textstring: &::windows::core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &::windows::core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>;
    fn GetGdiCompatibleGlyphPlacements(
        &self,
        textstring: &::windows::core::PCWSTR,
        clustermap: *const u16,
        textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
        textlength: u32,
        glyphindices: *const u16,
        glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphcount: u32,
        fontface: ::core::option::Option<&IDWriteFontFace>,
        fontemsize: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: super::super::Foundation::BOOL,
        issideways: super::super::Foundation::BOOL,
        isrighttoleft: super::super::Foundation::BOOL,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localename: &::windows::core::PCWSTR,
        features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
        featurerangelengths: *const u32,
        featureranges: u32,
        glyphadvances: *mut f32,
        glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>() -> IDWriteTextAnalyzer_Vtbl {
        unsafe extern "system" fn AnalyzeScript<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeScript(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeBidi<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeBidi(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeNumberSubstitution(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeLineBreakpoints(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn GetGlyphs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows::core::PCWSTR,
            textlength: u32,
            fontface: *mut ::core::ffi::c_void,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows::core::PCWSTR,
            numbersubstitution: *mut ::core::ffi::c_void,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            maxglyphcount: u32,
            clustermap: *mut u16,
            textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
            glyphindices: *mut u16,
            glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES,
            actualglyphcount: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphs(
                ::core::mem::transmute(&textstring),
                ::core::mem::transmute_copy(&textlength),
                ::windows::core::from_raw_borrowed(&fontface),
                ::core::mem::transmute_copy(&issideways),
                ::core::mem::transmute_copy(&isrighttoleft),
                ::core::mem::transmute_copy(&scriptanalysis),
                ::core::mem::transmute(&localename),
                ::windows::core::from_raw_borrowed(&numbersubstitution),
                ::core::mem::transmute_copy(&features),
                ::core::mem::transmute_copy(&featurerangelengths),
                ::core::mem::transmute_copy(&featureranges),
                ::core::mem::transmute_copy(&maxglyphcount),
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&textprops),
                ::core::mem::transmute_copy(&glyphindices),
                ::core::mem::transmute_copy(&glyphprops),
                ::core::mem::transmute_copy(&actualglyphcount),
            )
            .into()
        }
        unsafe extern "system" fn GetGlyphPlacements<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows::core::PCWSTR,
            clustermap: *const u16,
            textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut ::core::ffi::c_void,
            fontemsize: f32,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows::core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphPlacements(
                ::core::mem::transmute(&textstring),
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&textprops),
                ::core::mem::transmute_copy(&textlength),
                ::core::mem::transmute_copy(&glyphindices),
                ::core::mem::transmute_copy(&glyphprops),
                ::core::mem::transmute_copy(&glyphcount),
                ::windows::core::from_raw_borrowed(&fontface),
                ::core::mem::transmute_copy(&fontemsize),
                ::core::mem::transmute_copy(&issideways),
                ::core::mem::transmute_copy(&isrighttoleft),
                ::core::mem::transmute_copy(&scriptanalysis),
                ::core::mem::transmute(&localename),
                ::core::mem::transmute_copy(&features),
                ::core::mem::transmute_copy(&featurerangelengths),
                ::core::mem::transmute_copy(&featureranges),
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&glyphoffsets),
            )
            .into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphPlacements<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows::core::PCWSTR,
            clustermap: *const u16,
            textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut ::core::ffi::c_void,
            fontemsize: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: super::super::Foundation::BOOL,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows::core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGdiCompatibleGlyphPlacements(
                ::core::mem::transmute(&textstring),
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&textprops),
                ::core::mem::transmute_copy(&textlength),
                ::core::mem::transmute_copy(&glyphindices),
                ::core::mem::transmute_copy(&glyphprops),
                ::core::mem::transmute_copy(&glyphcount),
                ::windows::core::from_raw_borrowed(&fontface),
                ::core::mem::transmute_copy(&fontemsize),
                ::core::mem::transmute_copy(&pixelsperdip),
                ::core::mem::transmute_copy(&transform),
                ::core::mem::transmute_copy(&usegdinatural),
                ::core::mem::transmute_copy(&issideways),
                ::core::mem::transmute_copy(&isrighttoleft),
                ::core::mem::transmute_copy(&scriptanalysis),
                ::core::mem::transmute(&localename),
                ::core::mem::transmute_copy(&features),
                ::core::mem::transmute_copy(&featurerangelengths),
                ::core::mem::transmute_copy(&featureranges),
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&glyphoffsets),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AnalyzeScript: AnalyzeScript::<Identity, Impl, OFFSET>,
            AnalyzeBidi: AnalyzeBidi::<Identity, Impl, OFFSET>,
            AnalyzeNumberSubstitution: AnalyzeNumberSubstitution::<Identity, Impl, OFFSET>,
            AnalyzeLineBreakpoints: AnalyzeLineBreakpoints::<Identity, Impl, OFFSET>,
            GetGlyphs: GetGlyphs::<Identity, Impl, OFFSET>,
            GetGlyphPlacements: GetGlyphPlacements::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphPlacements: GetGdiCompatibleGlyphPlacements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer1_Impl: Sized + IDWriteTextAnalyzer_Impl {
    fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>;
    fn GetBaseline(&self, fontface: ::core::option::Option<&IDWriteFontFace>, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows::core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AnalyzeVerticalGlyphOrientation(&self, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource1>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink1>) -> ::windows::core::Result<()>;
    fn GetGlyphOrientationTransform(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()>;
    fn GetScriptProperties(&self, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows::core::Result<()>;
    fn GetTextComplexity(&self, textstring: &::windows::core::PCWSTR, textlength: u32, fontface: ::core::option::Option<&IDWriteFontFace>, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows::core::Result<()>;
    fn GetJustificationOpportunities(&self, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: &::windows::core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows::core::Result<()>;
    fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>;
    fn GetJustifiedGlyphs(&self, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzer1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>() -> IDWriteTextAnalyzer1_Vtbl {
        unsafe extern "system" fn ApplyCharacterSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyCharacterSpacing(
                ::core::mem::transmute_copy(&leadingspacing),
                ::core::mem::transmute_copy(&trailingspacing),
                ::core::mem::transmute_copy(&minimumadvancewidth),
                ::core::mem::transmute_copy(&textlength),
                ::core::mem::transmute_copy(&glyphcount),
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&glyphoffsets),
                ::core::mem::transmute_copy(&glyphproperties),
                ::core::mem::transmute_copy(&modifiedglyphadvances),
                ::core::mem::transmute_copy(&modifiedglyphoffsets),
            )
            .into()
        }
        unsafe extern "system" fn GetBaseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows::core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBaseline(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&baseline), ::core::mem::transmute_copy(&isvertical), ::core::mem::transmute_copy(&issimulationallowed), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&baselinecoordinate), ::core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnalyzeVerticalGlyphOrientation(::windows::core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphOrientationTransform(::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetScriptProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptProperties(::core::mem::transmute(&scriptanalysis), ::core::mem::transmute_copy(&scriptproperties)).into()
        }
        unsafe extern "system" fn GetTextComplexity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textstring: ::windows::core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextComplexity(::core::mem::transmute(&textstring), ::core::mem::transmute_copy(&textlength), ::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&istextsimple), ::core::mem::transmute_copy(&textlengthread), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetJustificationOpportunities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: ::windows::core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJustificationOpportunities(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute(&textstring), ::core::mem::transmute_copy(&clustermap), ::core::mem::transmute_copy(&glyphproperties), ::core::mem::transmute_copy(&justificationopportunities)).into()
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JustifyGlyphAdvances(::core::mem::transmute_copy(&linewidth), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&justificationopportunities), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&glyphoffsets), ::core::mem::transmute_copy(&justifiedglyphadvances), ::core::mem::transmute_copy(&justifiedglyphoffsets)).into()
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJustifiedGlyphs(
                ::windows::core::from_raw_borrowed(&fontface),
                ::core::mem::transmute_copy(&fontemsize),
                ::core::mem::transmute(&scriptanalysis),
                ::core::mem::transmute_copy(&textlength),
                ::core::mem::transmute_copy(&glyphcount),
                ::core::mem::transmute_copy(&maxglyphcount),
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&glyphindices),
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&justifiedglyphadvances),
                ::core::mem::transmute_copy(&justifiedglyphoffsets),
                ::core::mem::transmute_copy(&glyphproperties),
                ::core::mem::transmute_copy(&actualglyphcount),
                ::core::mem::transmute_copy(&modifiedclustermap),
                ::core::mem::transmute_copy(&modifiedglyphindices),
                ::core::mem::transmute_copy(&modifiedglyphadvances),
                ::core::mem::transmute_copy(&modifiedglyphoffsets),
            )
            .into()
        }
        Self {
            base__: IDWriteTextAnalyzer_Vtbl::new::<Identity, Impl, OFFSET>(),
            ApplyCharacterSpacing: ApplyCharacterSpacing::<Identity, Impl, OFFSET>,
            GetBaseline: GetBaseline::<Identity, Impl, OFFSET>,
            AnalyzeVerticalGlyphOrientation: AnalyzeVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, Impl, OFFSET>,
            GetScriptProperties: GetScriptProperties::<Identity, Impl, OFFSET>,
            GetTextComplexity: GetTextComplexity::<Identity, Impl, OFFSET>,
            GetJustificationOpportunities: GetJustificationOpportunities::<Identity, Impl, OFFSET>,
            JustifyGlyphAdvances: JustifyGlyphAdvances::<Identity, Impl, OFFSET>,
            GetJustifiedGlyphs: GetJustifiedGlyphs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextAnalyzer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer2_Impl: Sized + IDWriteTextAnalyzer1_Impl {
    fn GetGlyphOrientationTransform2(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()>;
    fn GetTypographicFeatures(&self, fontface: ::core::option::Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows::core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows::core::Result<()>;
    fn CheckTypographicFeature(&self, fontface: ::core::option::Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows::core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>() -> IDWriteTextAnalyzer2_Vtbl {
        unsafe extern "system" fn GetGlyphOrientationTransform2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphOrientationTransform2(::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetTypographicFeatures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows::core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypographicFeatures(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&maxtagcount), ::core::mem::transmute_copy(&actualtagcount), ::core::mem::transmute_copy(&tags)).into()
        }
        unsafe extern "system" fn CheckTypographicFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows::core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckTypographicFeature(::windows::core::from_raw_borrowed(&fontface), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&featuretag), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&featureapplies)).into()
        }
        Self {
            base__: IDWriteTextAnalyzer1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGlyphOrientationTransform2: GetGlyphOrientationTransform2::<Identity, Impl, OFFSET>,
            GetTypographicFeatures: GetTypographicFeatures::<Identity, Impl, OFFSET>,
            CheckTypographicFeature: CheckTypographicFeature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextAnalyzer as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextAnalyzer1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteTextFormat_Impl: Sized {
    fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()>;
    fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()>;
    fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()>;
    fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()>;
    fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()>;
    fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()>;
    fn SetTrimming(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: ::core::option::Option<&IDWriteInlineObject>) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()>;
    fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT;
    fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT;
    fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING;
    fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION;
    fn GetIncrementalTabStop(&self) -> f32;
    fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()>;
    fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()>;
    fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection>;
    fn GetFontFamilyNameLength(&self) -> u32;
    fn GetFontFamilyName(&self, fontfamilyname: ::windows::core::PWSTR, namesize: u32) -> ::windows::core::Result<()>;
    fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetFontStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetFontSize(&self) -> f32;
    fn GetLocaleNameLength(&self) -> u32;
    fn GetLocaleName(&self, localename: ::windows::core::PWSTR, namesize: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDWriteTextFormat {}
impl IDWriteTextFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>() -> IDWriteTextFormat_Vtbl {
        unsafe extern "system" fn SetTextAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextAlignment(::core::mem::transmute_copy(&textalignment)).into()
        }
        unsafe extern "system" fn SetParagraphAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParagraphAlignment(::core::mem::transmute_copy(&paragraphalignment)).into()
        }
        unsafe extern "system" fn SetWordWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWordWrapping(::core::mem::transmute_copy(&wordwrapping)).into()
        }
        unsafe extern "system" fn SetReadingDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReadingDirection(::core::mem::transmute_copy(&readingdirection)).into()
        }
        unsafe extern "system" fn SetFlowDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlowDirection(::core::mem::transmute_copy(&flowdirection)).into()
        }
        unsafe extern "system" fn SetIncrementalTabStop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, incrementaltabstop: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIncrementalTabStop(::core::mem::transmute_copy(&incrementaltabstop)).into()
        }
        unsafe extern "system" fn SetTrimming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrimming(::core::mem::transmute_copy(&trimmingoptions), ::windows::core::from_raw_borrowed(&trimmingsign)).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineSpacing(::core::mem::transmute_copy(&linespacingmethod), ::core::mem::transmute_copy(&linespacing), ::core::mem::transmute_copy(&baseline)).into()
        }
        unsafe extern "system" fn GetTextAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextAlignment()
        }
        unsafe extern "system" fn GetParagraphAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParagraphAlignment()
        }
        unsafe extern "system" fn GetWordWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_WORD_WRAPPING {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWordWrapping()
        }
        unsafe extern "system" fn GetReadingDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReadingDirection()
        }
        unsafe extern "system" fn GetFlowDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FLOW_DIRECTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlowDirection()
        }
        unsafe extern "system" fn GetIncrementalTabStop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIncrementalTabStop()
        }
        unsafe extern "system" fn GetTrimming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTrimming(::core::mem::transmute_copy(&trimmingoptions), ::core::mem::transmute_copy(&trimmingsign)).into()
        }
        unsafe extern "system" fn GetLineSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineSpacing(::core::mem::transmute_copy(&linespacingmethod), ::core::mem::transmute_copy(&linespacing), ::core::mem::transmute_copy(&baseline)).into()
        }
        unsafe extern "system" fn GetFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyNameLength()
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows::core::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyName(::core::mem::transmute_copy(&fontfamilyname), ::core::mem::transmute_copy(&namesize)).into()
        }
        unsafe extern "system" fn GetFontWeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontWeight()
        }
        unsafe extern "system" fn GetFontStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontStyle()
        }
        unsafe extern "system" fn GetFontStretch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontStretch()
        }
        unsafe extern "system" fn GetFontSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSize()
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleNameLength()
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: ::windows::core::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleName(::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&namesize)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTextAlignment: SetTextAlignment::<Identity, Impl, OFFSET>,
            SetParagraphAlignment: SetParagraphAlignment::<Identity, Impl, OFFSET>,
            SetWordWrapping: SetWordWrapping::<Identity, Impl, OFFSET>,
            SetReadingDirection: SetReadingDirection::<Identity, Impl, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, Impl, OFFSET>,
            SetIncrementalTabStop: SetIncrementalTabStop::<Identity, Impl, OFFSET>,
            SetTrimming: SetTrimming::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, Impl, OFFSET>,
            GetParagraphAlignment: GetParagraphAlignment::<Identity, Impl, OFFSET>,
            GetWordWrapping: GetWordWrapping::<Identity, Impl, OFFSET>,
            GetReadingDirection: GetReadingDirection::<Identity, Impl, OFFSET>,
            GetFlowDirection: GetFlowDirection::<Identity, Impl, OFFSET>,
            GetIncrementalTabStop: GetIncrementalTabStop::<Identity, Impl, OFFSET>,
            GetTrimming: GetTrimming::<Identity, Impl, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, Impl, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, Impl, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, Impl, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, Impl, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, Impl, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, Impl, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, Impl, OFFSET>,
            GetFontSize: GetFontSize::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat1_Impl: Sized + IDWriteTextFormat_Impl {
    fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows::core::Result<()>;
    fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextFormat1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>() -> IDWriteTextFormat1_Vtbl {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVerticalGlyphOrientation(::core::mem::transmute_copy(&glyphorientation)).into()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalGlyphOrientation()
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastLineWrapping(::core::mem::transmute_copy(&islastlinewrappingenabled)).into()
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastLineWrapping()
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpticalAlignment(::core::mem::transmute_copy(&opticalalignment)).into()
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOpticalAlignment()
        }
        unsafe extern "system" fn SetFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontFallback(::windows::core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn GetFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, Impl, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, Impl, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, Impl, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, Impl, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, Impl, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat2_Impl: Sized + IDWriteTextFormat1_Impl {
    fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::Result<()>;
    fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextFormat2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: isize>() -> IDWriteTextFormat2_Vtbl {
        unsafe extern "system" fn SetLineSpacing2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineSpacing2(::core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineSpacing2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineSpacing2(::core::mem::transmute_copy(&linespacingoptions)).into()
        }
        Self {
            base__: IDWriteTextFormat1_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetLineSpacing2: SetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineSpacing2: GetLineSpacing2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat3_Impl: Sized + IDWriteTextFormat2_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<()>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextFormat3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>() -> IDWriteTextFormat3_Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValueCount()
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAutomaticFontAxes()
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutomaticFontAxes(::core::mem::transmute_copy(&automaticfontaxes)).into()
        }
        Self {
            base__: IDWriteTextFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, Impl, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout_Impl: Sized + IDWriteTextFormat_Impl {
    fn SetMaxWidth(&self, maxwidth: f32) -> ::windows::core::Result<()>;
    fn SetMaxHeight(&self, maxheight: f32) -> ::windows::core::Result<()>;
    fn SetFontCollection(&self, fontcollection: ::core::option::Option<&IDWriteFontCollection>, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetFontFamilyName(&self, fontfamilyname: &::windows::core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetFontSize(&self, fontsize: f32, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetUnderline(&self, hasunderline: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetStrikethrough(&self, hasstrikethrough: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetDrawingEffect(&self, drawingeffect: ::core::option::Option<&::windows::core::IUnknown>, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetInlineObject(&self, inlineobject: ::core::option::Option<&IDWriteInlineObject>, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetTypography(&self, typography: ::core::option::Option<&IDWriteTypography>, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetLocaleName(&self, localename: &::windows::core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetMaxWidth(&self) -> f32;
    fn GetMaxHeight(&self) -> f32;
    fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: ::windows::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows::core::IUnknown>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetLocaleName2(&self, currentposition: u32, localename: ::windows::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn Draw(&self, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::core::option::Option<&IDWriteTextRenderer>, originx: f32, originy: f32) -> ::windows::core::Result<()>;
    fn GetLineMetrics(&self, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::Result<()>;
    fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetClusterMetrics(&self, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows::core::Result<()>;
    fn DetermineMinWidth(&self) -> ::windows::core::Result<f32>;
    fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>;
    fn HitTestTextPosition(&self, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>;
    fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextLayout {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>() -> IDWriteTextLayout_Vtbl {
        unsafe extern "system" fn SetMaxWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxWidth(::core::mem::transmute_copy(&maxwidth)).into()
        }
        unsafe extern "system" fn SetMaxHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxHeight(::core::mem::transmute_copy(&maxheight)).into()
        }
        unsafe extern "system" fn SetFontCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontCollection(::windows::core::from_raw_borrowed(&fontcollection), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows::core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontFamilyName(::core::mem::transmute(&fontfamilyname), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontWeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontWeight(::core::mem::transmute_copy(&fontweight), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontStyle(::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontStretch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontStretch(::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontSize(::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnderline(::core::mem::transmute_copy(&hasunderline), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrikethrough(::core::mem::transmute_copy(&hasstrikethrough), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetDrawingEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDrawingEffect(::windows::core::from_raw_borrowed(&drawingeffect), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetInlineObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inlineobject: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInlineObject(::windows::core::from_raw_borrowed(&inlineobject), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetTypography<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypography(::windows::core::from_raw_borrowed(&typography), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetLocaleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: ::windows::core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocaleName(::core::mem::transmute(&localename), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetMaxWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxWidth()
        }
        unsafe extern "system" fn GetMaxHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxHeight()
        }
        unsafe extern "system" fn GetFontCollection2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontcollection: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontCollection2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontFamilyNameLength2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyNameLength2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontFamilyName2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontfamilyname: ::windows::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFamilyName2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontfamilyname), ::core::mem::transmute_copy(&namesize), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontWeight2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontWeight2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontStyle2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontStyle2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontStretch2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontStretch2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontSize2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontSize2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetUnderline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUnderline(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&hasunderline), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetStrikethrough<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStrikethrough(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&hasstrikethrough), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetDrawingEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDrawingEffect(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&drawingeffect), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetInlineObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, inlineobject: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInlineObject(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&inlineobject), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetTypography<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, typography: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypography(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&typography), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetLocaleNameLength2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleNameLength2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetLocaleName2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, localename: ::windows::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocaleName2(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&namesize), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&clientdrawingcontext), ::windows::core::from_raw_borrowed(&renderer), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy)).into()
        }
        unsafe extern "system" fn GetLineMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineMetrics(::core::mem::transmute_copy(&linemetrics), ::core::mem::transmute_copy(&maxlinecount), ::core::mem::transmute_copy(&actuallinecount)).into()
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics(::core::mem::transmute_copy(&textmetrics)).into()
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOverhangMetrics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overhangs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClusterMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClusterMetrics(::core::mem::transmute_copy(&clustermetrics), ::core::mem::transmute_copy(&maxclustercount), ::core::mem::transmute_copy(&actualclustercount)).into()
        }
        unsafe extern "system" fn DetermineMinWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetermineMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HitTestPoint(::core::mem::transmute_copy(&pointx), ::core::mem::transmute_copy(&pointy), ::core::mem::transmute_copy(&istrailinghit), ::core::mem::transmute_copy(&isinside), ::core::mem::transmute_copy(&hittestmetrics)).into()
        }
        unsafe extern "system" fn HitTestTextPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HitTestTextPosition(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&istrailinghit), ::core::mem::transmute_copy(&pointx), ::core::mem::transmute_copy(&pointy), ::core::mem::transmute_copy(&hittestmetrics)).into()
        }
        unsafe extern "system" fn HitTestTextRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HitTestTextRange(::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&hittestmetrics), ::core::mem::transmute_copy(&maxhittestmetricscount), ::core::mem::transmute_copy(&actualhittestmetricscount)).into()
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMaxWidth: SetMaxWidth::<Identity, Impl, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, Impl, OFFSET>,
            SetFontCollection: SetFontCollection::<Identity, Impl, OFFSET>,
            SetFontFamilyName: SetFontFamilyName::<Identity, Impl, OFFSET>,
            SetFontWeight: SetFontWeight::<Identity, Impl, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, Impl, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, Impl, OFFSET>,
            SetFontSize: SetFontSize::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            SetDrawingEffect: SetDrawingEffect::<Identity, Impl, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, Impl, OFFSET>,
            SetTypography: SetTypography::<Identity, Impl, OFFSET>,
            SetLocaleName: SetLocaleName::<Identity, Impl, OFFSET>,
            GetMaxWidth: GetMaxWidth::<Identity, Impl, OFFSET>,
            GetMaxHeight: GetMaxHeight::<Identity, Impl, OFFSET>,
            GetFontCollection2: GetFontCollection2::<Identity, Impl, OFFSET>,
            GetFontFamilyNameLength2: GetFontFamilyNameLength2::<Identity, Impl, OFFSET>,
            GetFontFamilyName2: GetFontFamilyName2::<Identity, Impl, OFFSET>,
            GetFontWeight2: GetFontWeight2::<Identity, Impl, OFFSET>,
            GetFontStyle2: GetFontStyle2::<Identity, Impl, OFFSET>,
            GetFontStretch2: GetFontStretch2::<Identity, Impl, OFFSET>,
            GetFontSize2: GetFontSize2::<Identity, Impl, OFFSET>,
            GetUnderline: GetUnderline::<Identity, Impl, OFFSET>,
            GetStrikethrough: GetStrikethrough::<Identity, Impl, OFFSET>,
            GetDrawingEffect: GetDrawingEffect::<Identity, Impl, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, Impl, OFFSET>,
            GetTypography: GetTypography::<Identity, Impl, OFFSET>,
            GetLocaleNameLength2: GetLocaleNameLength2::<Identity, Impl, OFFSET>,
            GetLocaleName2: GetLocaleName2::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, Impl, OFFSET>,
            GetClusterMetrics: GetClusterMetrics::<Identity, Impl, OFFSET>,
            DetermineMinWidth: DetermineMinWidth::<Identity, Impl, OFFSET>,
            HitTestPoint: HitTestPoint::<Identity, Impl, OFFSET>,
            HitTestTextPosition: HitTestTextPosition::<Identity, Impl, OFFSET>,
            HitTestTextRange: HitTestTextRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout1_Impl: Sized + IDWriteTextLayout_Impl {
    fn SetPairKerning(&self, ispairkerningenabled: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextLayout1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: isize>() -> IDWriteTextLayout1_Vtbl {
        unsafe extern "system" fn SetPairKerning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPairKerning(::core::mem::transmute_copy(&ispairkerningenabled), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetPairKerning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPairKerning(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&ispairkerningenabled), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn SetCharacterSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharacterSpacing(::core::mem::transmute_copy(&leadingspacing), ::core::mem::transmute_copy(&trailingspacing), ::core::mem::transmute_copy(&minimumadvancewidth), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetCharacterSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCharacterSpacing(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&leadingspacing), ::core::mem::transmute_copy(&trailingspacing), ::core::mem::transmute_copy(&minimumadvancewidth), ::core::mem::transmute_copy(&textrange)).into()
        }
        Self {
            base__: IDWriteTextLayout_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPairKerning: SetPairKerning::<Identity, Impl, OFFSET>,
            GetPairKerning: GetPairKerning::<Identity, Impl, OFFSET>,
            SetCharacterSpacing: SetCharacterSpacing::<Identity, Impl, OFFSET>,
            GetCharacterSpacing: GetCharacterSpacing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout2_Impl: Sized + IDWriteTextLayout1_Impl {
    fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::Result<()>;
    fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows::core::Result<()>;
    fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextLayout2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>() -> IDWriteTextLayout2_Vtbl {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetrics2(::core::mem::transmute_copy(&textmetrics)).into()
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVerticalGlyphOrientation(::core::mem::transmute_copy(&glyphorientation)).into()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVerticalGlyphOrientation()
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastLineWrapping(::core::mem::transmute_copy(&islastlinewrappingenabled)).into()
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastLineWrapping()
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpticalAlignment(::core::mem::transmute_copy(&opticalalignment)).into()
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOpticalAlignment()
        }
        unsafe extern "system" fn SetFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontFallback(::windows::core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn GetFontFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteTextLayout1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, Impl, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, Impl, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, Impl, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, Impl, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, Impl, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout3_Impl: Sized + IDWriteTextLayout2_Impl {
    fn InvalidateLayout(&self) -> ::windows::core::Result<()>;
    fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::Result<()>;
    fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::Result<()>;
    fn GetLineMetrics2(&self, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextLayout3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: isize>() -> IDWriteTextLayout3_Vtbl {
        unsafe extern "system" fn InvalidateLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateLayout().into()
        }
        unsafe extern "system" fn SetLineSpacing2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineSpacing2(::core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineSpacing2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineSpacing2(::core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineMetrics2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineMetrics2(::core::mem::transmute_copy(&linemetrics), ::core::mem::transmute_copy(&maxlinecount), ::core::mem::transmute_copy(&actuallinecount)).into()
        }
        Self {
            base__: IDWriteTextLayout2_Vtbl::new::<Identity, Impl, OFFSET>(),
            InvalidateLayout: InvalidateLayout::<Identity, Impl, OFFSET>,
            SetLineSpacing2: SetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineSpacing2: GetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineMetrics2: GetLineMetrics2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout3 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout4_Impl: Sized + IDWriteTextLayout3_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: &DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetFontAxisValueCount(&self, currentposition: u32) -> u32;
    fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextLayout4 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>() -> IDWriteTextLayout4_Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValueCount(::core::mem::transmute_copy(&currentposition))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontAxisValues(::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAutomaticFontAxes()
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutomaticFontAxes(::core::mem::transmute_copy(&automaticfontaxes)).into()
        }
        Self {
            base__: IDWriteTextLayout3_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, Impl, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout4 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextFormat as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout1 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout2 as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextLayout3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRenderer_Impl: Sized + IDWritePixelSnapping_Impl {
    fn DrawGlyphRun(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawUnderline(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawStrikethrough(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawInlineObject(&self, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: ::core::option::Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextRenderer {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: isize>() -> IDWriteTextRenderer_Vtbl {
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawGlyphRun(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawUnderline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawUnderline(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&underline), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawStrikethrough(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&strikethrough), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawInlineObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawInlineObject(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::windows::core::from_raw_borrowed(&inlineobject), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        Self {
            base__: IDWritePixelSnapping_Vtbl::new::<Identity, Impl, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, Impl, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, Impl, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextRenderer as ::windows::core::ComInterface>::IID || iid == &<IDWritePixelSnapping as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRenderer1_Impl: Sized + IDWriteTextRenderer_Impl {
    fn DrawGlyphRun2(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawUnderline2(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawStrikethrough2(&self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DrawInlineObject2(&self, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: ::core::option::Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDWriteTextRenderer1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextRenderer1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: isize>() -> IDWriteTextRenderer1_Vtbl {
        unsafe extern "system" fn DrawGlyphRun2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawGlyphRun2(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawUnderline2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawUnderline2(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&underline), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawStrikethrough2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawStrikethrough2(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&strikethrough), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawInlineObject2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawInlineObject2(::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&orientationangle), ::windows::core::from_raw_borrowed(&inlineobject), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows::core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        Self {
            base__: IDWriteTextRenderer_Vtbl::new::<Identity, Impl, OFFSET>(),
            DrawGlyphRun2: DrawGlyphRun2::<Identity, Impl, OFFSET>,
            DrawUnderline2: DrawUnderline2::<Identity, Impl, OFFSET>,
            DrawStrikethrough2: DrawStrikethrough2::<Identity, Impl, OFFSET>,
            DrawInlineObject2: DrawInlineObject2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextRenderer1 as ::windows::core::ComInterface>::IID || iid == &<IDWritePixelSnapping as ::windows::core::ComInterface>::IID || iid == &<IDWriteTextRenderer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"implement\"`*"]
pub trait IDWriteTypography_Impl: Sized {
    fn AddFontFeature(&self, fontfeature: &DWRITE_FONT_FEATURE) -> ::windows::core::Result<()>;
    fn GetFontFeatureCount(&self) -> u32;
    fn GetFontFeature(&self, fontfeatureindex: u32) -> ::windows::core::Result<DWRITE_FONT_FEATURE>;
}
impl ::windows::core::RuntimeName for IDWriteTypography {}
impl IDWriteTypography_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: isize>() -> IDWriteTypography_Vtbl {
        unsafe extern "system" fn AddFontFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFontFeature(::core::mem::transmute(&fontfeature)).into()
        }
        unsafe extern "system" fn GetFontFeatureCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFontFeatureCount()
        }
        unsafe extern "system" fn GetFontFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFeature(::core::mem::transmute_copy(&fontfeatureindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfeature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFeature: AddFontFeature::<Identity, Impl, OFFSET>,
            GetFontFeatureCount: GetFontFeatureCount::<Identity, Impl, OFFSET>,
            GetFontFeature: GetFontFeature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTypography as ::windows::core::ComInterface>::IID
    }
}
