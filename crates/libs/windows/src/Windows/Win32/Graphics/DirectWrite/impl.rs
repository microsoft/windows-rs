#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteAsyncResultImpl: Sized {
    fn GetWaitHandle();
    fn GetResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteAsyncResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteAsyncResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteAsyncResultVtbl {
        unsafe extern "system" fn GetWaitHandle<Impl: IDWriteAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResult<Impl: IDWriteAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWaitHandle::<Impl, IMPL_OFFSET>, GetResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteAsyncResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTargetImpl: Sized {
    fn DrawGlyphRun();
    fn GetMemoryDC();
    fn GetPixelsPerDip();
    fn SetPixelsPerDip();
    fn GetCurrentTransform();
    fn SetCurrentTransform();
    fn GetSize();
    fn Resize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteBitmapRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteBitmapRenderTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteBitmapRenderTargetVtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: ::windows::core::RawPtr, textcolor: u32, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemoryDC<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelsPerDip<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPixelsPerDip<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelsperdip: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentTransform<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentTransform<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resize<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DrawGlyphRun::<Impl, IMPL_OFFSET>, GetMemoryDC::<Impl, IMPL_OFFSET>, GetPixelsPerDip::<Impl, IMPL_OFFSET>, SetPixelsPerDip::<Impl, IMPL_OFFSET>, GetCurrentTransform::<Impl, IMPL_OFFSET>, SetCurrentTransform::<Impl, IMPL_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, Resize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTarget1Impl: Sized + IDWriteBitmapRenderTargetImpl {
    fn GetTextAntialiasMode();
    fn SetTextAntialiasMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteBitmapRenderTarget1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteBitmapRenderTarget1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteBitmapRenderTarget1Vtbl {
        unsafe extern "system" fn GetTextAntialiasMode<Impl: IDWriteBitmapRenderTarget1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: IDWriteBitmapRenderTarget1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            GetMemoryDC::<Impl, IMPL_OFFSET>,
            GetPixelsPerDip::<Impl, IMPL_OFFSET>,
            SetPixelsPerDip::<Impl, IMPL_OFFSET>,
            GetCurrentTransform::<Impl, IMPL_OFFSET>,
            SetCurrentTransform::<Impl, IMPL_OFFSET>,
            GetSize::<Impl, IMPL_OFFSET>,
            Resize::<Impl, IMPL_OFFSET>,
            GetTextAntialiasMode::<Impl, IMPL_OFFSET>,
            SetTextAntialiasMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentRun();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteColorGlyphRunEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteColorGlyphRunEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteColorGlyphRunEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IDWriteColorGlyphRunEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentRun<Impl: IDWriteColorGlyphRunEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MoveNext::<Impl, IMPL_OFFSET>, GetCurrentRun::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumerator1Impl: Sized + IDWriteColorGlyphRunEnumeratorImpl {
    fn GetCurrentRun();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteColorGlyphRunEnumerator1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteColorGlyphRunEnumerator1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator1Vtbl {
        unsafe extern "system" fn GetCurrentRun<Impl: IDWriteColorGlyphRunEnumerator1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MoveNext::<Impl, IMPL_OFFSET>, GetCurrentRun::<Impl, IMPL_OFFSET>, GetCurrentRun::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactoryImpl: Sized {
    fn GetSystemFontCollection();
    fn CreateCustomFontCollection();
    fn RegisterFontCollectionLoader();
    fn UnregisterFontCollectionLoader();
    fn CreateFontFileReference();
    fn CreateCustomFontFileReference();
    fn CreateFontFace();
    fn CreateRenderingParams();
    fn CreateMonitorRenderingParams();
    fn CreateCustomRenderingParams();
    fn RegisterFontFileLoader();
    fn UnregisterFontFileLoader();
    fn CreateTextFormat();
    fn CreateTypography();
    fn GetGdiInterop();
    fn CreateTextLayout();
    fn CreateGdiCompatibleTextLayout();
    fn CreateEllipsisTrimmingSign();
    fn CreateTextAnalyzer();
    fn CreateNumberSubstitution();
    fn CreateGlyphRunAnalysis();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactoryVtbl {
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomFontCollection<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionloader: ::windows::core::RawPtr, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFileReference<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: ::windows::core::RawPtr, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const ::windows::core::RawPtr, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterFontFileLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTextFormat<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, fontcollection: ::windows::core::RawPtr, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: super::super::Foundation::PWSTR, textformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTypography<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiInterop<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdiinterop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTextLayout<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, maxwidth: f32, maxheight: f32, textlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textformat: ::windows::core::RawPtr, trimmingsign: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTextAnalyzer<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textanalyzer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNumberSubstitution<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: super::super::Foundation::PWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory1Impl: Sized + IDWriteFactoryImpl {
    fn GetEudcFontCollection();
    fn CreateCustomRenderingParams();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory1Vtbl {
        unsafe extern "system" fn GetEudcFontCollection<Impl: IDWriteFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory2Impl: Sized + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontFallback();
    fn CreateFontFallbackBuilder();
    fn TranslateColorGlyphRun();
    fn CreateCustomRenderingParams();
    fn CreateGlyphRunAnalysis();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory2Vtbl {
        unsafe extern "system" fn GetSystemFontFallback<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallbackbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory3Impl: Sized + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateGlyphRunAnalysis();
    fn CreateCustomRenderingParams();
    fn CreateFontFaceReference();
    fn CreateFontFaceReference();
    fn GetSystemFontSet();
    fn CreateFontSetBuilder();
    fn CreateFontCollectionFromFontSet();
    fn GetSystemFontCollection();
    fn GetFontDownloadQueue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory3Vtbl {
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontDownloadQueue<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontdownloadqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            GetFontDownloadQueue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory4Impl: Sized + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn TranslateColorGlyphRun();
    fn ComputeGlyphOrigins();
    fn ComputeGlyphOrigins();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory4Vtbl {
        unsafe extern "system" fn TranslateColorGlyphRun<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            GetFontDownloadQueue::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory5Impl: Sized + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontSetBuilder();
    fn CreateInMemoryFontFileLoader();
    fn CreateHttpFontFileLoader();
    fn AnalyzeContainerType();
    fn UnpackFontFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory5Vtbl {
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referrerurl: super::super::Foundation::PWSTR, extraheaders: super::super::Foundation::PWSTR, newloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeContainerType<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnpackFontFile<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            GetFontDownloadQueue::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateInMemoryFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateHttpFontFileLoader::<Impl, IMPL_OFFSET>,
            AnalyzeContainerType::<Impl, IMPL_OFFSET>,
            UnpackFontFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory6Impl: Sized + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontFaceReference();
    fn CreateFontResource();
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
    fn CreateFontCollectionFromFontSet();
    fn CreateFontSetBuilder();
    fn CreateTextFormat();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory6Vtbl {
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontResource<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTextFormat<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, fontcollection: ::windows::core::RawPtr, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: super::super::Foundation::PWSTR, textformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            GetFontDownloadQueue::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateInMemoryFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateHttpFontFileLoader::<Impl, IMPL_OFFSET>,
            AnalyzeContainerType::<Impl, IMPL_OFFSET>,
            UnpackFontFile::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory7Impl: Sized + IDWriteFactory6Impl + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFactory7Vtbl {
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomFontCollection::<Impl, IMPL_OFFSET>,
            RegisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontCollectionLoader::<Impl, IMPL_OFFSET>,
            CreateFontFileReference::<Impl, IMPL_OFFSET>,
            CreateCustomFontFileReference::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateRenderingParams::<Impl, IMPL_OFFSET>,
            CreateMonitorRenderingParams::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            RegisterFontFileLoader::<Impl, IMPL_OFFSET>,
            UnregisterFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            CreateTypography::<Impl, IMPL_OFFSET>,
            GetGdiInterop::<Impl, IMPL_OFFSET>,
            CreateTextLayout::<Impl, IMPL_OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, IMPL_OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, IMPL_OFFSET>,
            CreateTextAnalyzer::<Impl, IMPL_OFFSET>,
            CreateNumberSubstitution::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            GetEudcFontCollection::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            GetSystemFontFallback::<Impl, IMPL_OFFSET>,
            CreateFontFallbackBuilder::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateGlyphRunAnalysis::<Impl, IMPL_OFFSET>,
            CreateCustomRenderingParams::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            GetFontDownloadQueue::<Impl, IMPL_OFFSET>,
            TranslateColorGlyphRun::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            ComputeGlyphOrigins::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateInMemoryFontFileLoader::<Impl, IMPL_OFFSET>,
            CreateHttpFontFileLoader::<Impl, IMPL_OFFSET>,
            AnalyzeContainerType::<Impl, IMPL_OFFSET>,
            UnpackFontFile::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, IMPL_OFFSET>,
            CreateFontSetBuilder::<Impl, IMPL_OFFSET>,
            CreateTextFormat::<Impl, IMPL_OFFSET>,
            GetSystemFontSet::<Impl, IMPL_OFFSET>,
            GetSystemFontCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFactory7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontImpl: Sized {
    fn GetFontFamily();
    fn GetWeight();
    fn GetStretch();
    fn GetStyle();
    fn IsSymbolFont();
    fn GetFaceNames();
    fn GetInformationalStrings();
    fn GetSimulations();
    fn GetMetrics();
    fn HasCharacter();
    fn CreateFontFace();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontVtbl {
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWeight<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStretch<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyle<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSymbolFont<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInformationalStrings<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::windows::core::RawPtr, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont1Impl: Sized + IDWriteFontImpl {
    fn GetMetrics();
    fn GetPanose();
    fn GetUnicodeRanges();
    fn IsMonospacedFont();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFont1Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPanose<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnicodeRanges<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMonospacedFont<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont2Impl: Sized + IDWriteFont1Impl + IDWriteFontImpl {
    fn IsColorFont();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFont2Vtbl {
        unsafe extern "system" fn IsColorFont<Impl: IDWriteFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont3Impl: Sized + IDWriteFont2Impl + IDWriteFont1Impl + IDWriteFontImpl {
    fn CreateFontFace();
    fn Equals();
    fn GetFontFaceReference();
    fn HasCharacter();
    fn GetLocality();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFont3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFont3Vtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            GetLocality::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFont3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollectionImpl: Sized {
    fn GetFontFamilyCount();
    fn GetFontFamily();
    fn FindFamilyName();
    fn GetFontFromFontFace();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontCollectionVtbl {
        unsafe extern "system" fn GetFontFamilyCount<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFamilyName<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFromFontFace<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontFamilyCount::<Impl, IMPL_OFFSET>, GetFontFamily::<Impl, IMPL_OFFSET>, FindFamilyName::<Impl, IMPL_OFFSET>, GetFontFromFontFace::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection1Impl: Sized + IDWriteFontCollectionImpl {
    fn GetFontSet();
    fn GetFontFamily();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontCollection1Vtbl {
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontFamilyCount::<Impl, IMPL_OFFSET>, GetFontFamily::<Impl, IMPL_OFFSET>, FindFamilyName::<Impl, IMPL_OFFSET>, GetFontFromFontFace::<Impl, IMPL_OFFSET>, GetFontSet::<Impl, IMPL_OFFSET>, GetFontFamily::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection2Impl: Sized + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetFontFamily();
    fn GetMatchingFonts();
    fn GetFontFamilyModel();
    fn GetFontSet();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontCollection2Vtbl {
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamilyModel<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamilyCount::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            FindFamilyName::<Impl, IMPL_OFFSET>,
            GetFontFromFontFace::<Impl, IMPL_OFFSET>,
            GetFontSet::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFontFamilyModel::<Impl, IMPL_OFFSET>,
            GetFontSet::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection3Impl: Sized + IDWriteFontCollection2Impl + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetExpirationEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontCollection3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontCollection3Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Impl: IDWriteFontCollection3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFamilyCount::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            FindFamilyName::<Impl, IMPL_OFFSET>,
            GetFontFromFontFace::<Impl, IMPL_OFFSET>,
            GetFontSet::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetFontFamily::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFontFamilyModel::<Impl, IMPL_OFFSET>,
            GetFontSet::<Impl, IMPL_OFFSET>,
            GetExpirationEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollection3 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontCollectionLoaderImpl: Sized {
    fn CreateEnumeratorFromKey();
}
impl IDWriteFontCollectionLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollectionLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontCollectionLoaderVtbl {
        unsafe extern "system" fn CreateEnumeratorFromKey<Impl: IDWriteFontCollectionLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateEnumeratorFromKey::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontCollectionLoader as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontDownloadListenerImpl: Sized {
    fn DownloadCompleted();
}
impl IDWriteFontDownloadListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontDownloadListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontDownloadListenerVtbl {
        unsafe extern "system" fn DownloadCompleted<Impl: IDWriteFontDownloadListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadqueue: ::windows::core::RawPtr, context: *mut ::core::ffi::c_void, downloadresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DownloadCompleted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontDownloadListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontDownloadQueueImpl: Sized {
    fn AddListener();
    fn RemoveListener();
    fn IsEmpty();
    fn BeginDownload();
    fn CancelDownload();
    fn GetGenerationCount();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontDownloadQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontDownloadQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontDownloadQueueVtbl {
        unsafe extern "system" fn AddListener<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr, token: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveListener<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEmpty<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDownload<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelDownload<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGenerationCount<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddListener::<Impl, IMPL_OFFSET>, RemoveListener::<Impl, IMPL_OFFSET>, IsEmpty::<Impl, IMPL_OFFSET>, BeginDownload::<Impl, IMPL_OFFSET>, CancelDownload::<Impl, IMPL_OFFSET>, GetGenerationCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontDownloadQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFaceImpl: Sized {
    fn GetType();
    fn GetFiles();
    fn GetIndex();
    fn GetSimulations();
    fn IsSymbolFont();
    fn GetMetrics();
    fn GetGlyphCount();
    fn GetDesignGlyphMetrics();
    fn GetGlyphIndices();
    fn TryGetFontTable();
    fn ReleaseFontTable();
    fn GetGlyphRunOutline();
    fn GetRecommendedRenderingMode();
    fn GetGdiCompatibleMetrics();
    fn GetGdiCompatibleGlyphMetrics();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFaceVtbl {
        unsafe extern "system" fn GetType<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFiles<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndex<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSymbolFont<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphCount<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TryGetFontTable<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFontTable<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablecontext: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphRunOutline<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace1Impl: Sized + IDWriteFontFaceImpl {
    fn GetMetrics();
    fn GetGdiCompatibleMetrics();
    fn GetCaretMetrics();
    fn GetUnicodeRanges();
    fn IsMonospacedFont();
    fn GetDesignGlyphAdvances();
    fn GetGdiCompatibleGlyphAdvances();
    fn GetKerningPairAdjustments();
    fn HasKerningPairs();
    fn GetRecommendedRenderingMode();
    fn GetVerticalGlyphVariants();
    fn HasVerticalGlyphVariants();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace1Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaretMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnicodeRanges<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMonospacedFont<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasKerningPairs<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace2Impl: Sized + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn IsColorFont();
    fn GetColorPaletteCount();
    fn GetPaletteEntryCount();
    fn GetPaletteEntries();
    fn GetRecommendedRenderingMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace2Vtbl {
        unsafe extern "system" fn IsColorFont<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorPaletteCount<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPaletteEntryCount<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPaletteEntries<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            GetColorPaletteCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntryCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace3Impl: Sized + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFontFaceReference();
    fn GetPanose();
    fn GetWeight();
    fn GetStretch();
    fn GetStyle();
    fn GetFamilyNames();
    fn GetFaceNames();
    fn GetInformationalStrings();
    fn HasCharacter();
    fn GetRecommendedRenderingMode();
    fn IsCharacterLocal();
    fn IsGlyphLocal();
    fn AreCharactersLocal();
    fn AreGlyphsLocal();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace3Vtbl {
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPanose<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWeight<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStretch<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyle<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInformationalStrings<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::windows::core::RawPtr, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCharacterLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsGlyphLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AreCharactersLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: super::super::Foundation::PWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AreGlyphsLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            GetColorPaletteCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntryCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            IsCharacterLocal::<Impl, IMPL_OFFSET>,
            IsGlyphLocal::<Impl, IMPL_OFFSET>,
            AreCharactersLocal::<Impl, IMPL_OFFSET>,
            AreGlyphsLocal::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace4Impl: Sized + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetGlyphImageFormats();
    fn GetGlyphImageFormats();
    fn GetGlyphImageData();
    fn ReleaseGlyphImageData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace4Vtbl {
        unsafe extern "system" fn GetGlyphImageFormats<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphImageFormats<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphImageData<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphdatacontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            GetColorPaletteCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntryCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            IsCharacterLocal::<Impl, IMPL_OFFSET>,
            IsGlyphLocal::<Impl, IMPL_OFFSET>,
            AreCharactersLocal::<Impl, IMPL_OFFSET>,
            AreGlyphsLocal::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageData::<Impl, IMPL_OFFSET>,
            ReleaseGlyphImageData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace5Impl: Sized + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn HasVariations();
    fn GetFontResource();
    fn Equals();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace5Vtbl {
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasVariations<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontResource<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            GetColorPaletteCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntryCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            IsCharacterLocal::<Impl, IMPL_OFFSET>,
            IsGlyphLocal::<Impl, IMPL_OFFSET>,
            AreCharactersLocal::<Impl, IMPL_OFFSET>,
            AreGlyphsLocal::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageData::<Impl, IMPL_OFFSET>,
            ReleaseGlyphImageData::<Impl, IMPL_OFFSET>,
            GetFontAxisValueCount::<Impl, IMPL_OFFSET>,
            GetFontAxisValues::<Impl, IMPL_OFFSET>,
            HasVariations::<Impl, IMPL_OFFSET>,
            GetFontResource::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace6Impl: Sized + IDWriteFontFace5Impl + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFamilyNames();
    fn GetFaceNames();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDWriteFontFace6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFace6Vtbl {
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFace6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontFace6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFiles::<Impl, IMPL_OFFSET>,
            GetIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            IsSymbolFont::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphCount::<Impl, IMPL_OFFSET>,
            GetDesignGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            TryGetFontTable::<Impl, IMPL_OFFSET>,
            ReleaseFontTable::<Impl, IMPL_OFFSET>,
            GetGlyphRunOutline::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleMetrics::<Impl, IMPL_OFFSET>,
            GetCaretMetrics::<Impl, IMPL_OFFSET>,
            GetUnicodeRanges::<Impl, IMPL_OFFSET>,
            IsMonospacedFont::<Impl, IMPL_OFFSET>,
            GetDesignGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetKerningPairAdjustments::<Impl, IMPL_OFFSET>,
            HasKerningPairs::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            HasVerticalGlyphVariants::<Impl, IMPL_OFFSET>,
            IsColorFont::<Impl, IMPL_OFFSET>,
            GetColorPaletteCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntryCount::<Impl, IMPL_OFFSET>,
            GetPaletteEntries::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            GetPanose::<Impl, IMPL_OFFSET>,
            GetWeight::<Impl, IMPL_OFFSET>,
            GetStretch::<Impl, IMPL_OFFSET>,
            GetStyle::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
            GetInformationalStrings::<Impl, IMPL_OFFSET>,
            HasCharacter::<Impl, IMPL_OFFSET>,
            GetRecommendedRenderingMode::<Impl, IMPL_OFFSET>,
            IsCharacterLocal::<Impl, IMPL_OFFSET>,
            IsGlyphLocal::<Impl, IMPL_OFFSET>,
            AreCharactersLocal::<Impl, IMPL_OFFSET>,
            AreGlyphsLocal::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageFormats::<Impl, IMPL_OFFSET>,
            GetGlyphImageData::<Impl, IMPL_OFFSET>,
            ReleaseGlyphImageData::<Impl, IMPL_OFFSET>,
            GetFontAxisValueCount::<Impl, IMPL_OFFSET>,
            GetFontAxisValues::<Impl, IMPL_OFFSET>,
            HasVariations::<Impl, IMPL_OFFSET>,
            GetFontResource::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFaceNames::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFace6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReferenceImpl: Sized {
    fn CreateFontFace();
    fn CreateFontFaceWithSimulations();
    fn Equals();
    fn GetFontFaceIndex();
    fn GetSimulations();
    fn GetFontFile();
    fn GetLocalFileSize();
    fn GetFileSize();
    fn GetFileTime();
    fn GetLocality();
    fn EnqueueFontDownloadRequest();
    fn EnqueueCharacterDownloadRequest();
    fn EnqueueGlyphDownloadRequest();
    fn EnqueueFileFragmentDownloadRequest();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFaceReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFaceReferenceVtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFile<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalFileSize<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileTime<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: super::super::Foundation::PWSTR, charactercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateFontFaceWithSimulations::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
            GetFontFaceIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetFontFile::<Impl, IMPL_OFFSET>,
            GetLocalFileSize::<Impl, IMPL_OFFSET>,
            GetFileSize::<Impl, IMPL_OFFSET>,
            GetFileTime::<Impl, IMPL_OFFSET>,
            GetLocality::<Impl, IMPL_OFFSET>,
            EnqueueFontDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueCharacterDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueGlyphDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueFileFragmentDownloadRequest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReference1Impl: Sized + IDWriteFontFaceReferenceImpl {
    fn CreateFontFace();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFaceReference1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceReference1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFaceReference1Vtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateFontFaceWithSimulations::<Impl, IMPL_OFFSET>,
            Equals::<Impl, IMPL_OFFSET>,
            GetFontFaceIndex::<Impl, IMPL_OFFSET>,
            GetSimulations::<Impl, IMPL_OFFSET>,
            GetFontFile::<Impl, IMPL_OFFSET>,
            GetLocalFileSize::<Impl, IMPL_OFFSET>,
            GetFileSize::<Impl, IMPL_OFFSET>,
            GetFileTime::<Impl, IMPL_OFFSET>,
            GetLocality::<Impl, IMPL_OFFSET>,
            EnqueueFontDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueCharacterDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueGlyphDownloadRequest::<Impl, IMPL_OFFSET>,
            EnqueueFileFragmentDownloadRequest::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetFontAxisValueCount::<Impl, IMPL_OFFSET>,
            GetFontAxisValues::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFallbackImpl: Sized {
    fn MapCharacters();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFallbackVtbl {
        unsafe extern "system" fn MapCharacters<Impl: IDWriteFontFallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, basefontcollection: ::windows::core::RawPtr, basefamilyname: super::super::Foundation::PWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::windows::core::RawPtr, scale: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MapCharacters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFallback1Impl: Sized + IDWriteFontFallbackImpl {
    fn MapCharacters();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFallback1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallback1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFallback1Vtbl {
        unsafe extern "system" fn MapCharacters<Impl: IDWriteFontFallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, basefontcollection: ::windows::core::RawPtr, basefamilyname: super::super::Foundation::PWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MapCharacters::<Impl, IMPL_OFFSET>, MapCharacters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallback1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFallbackBuilderImpl: Sized {
    fn AddMapping();
    fn AddMappings();
    fn CreateFontFallback();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFallbackBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallbackBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFallbackBuilderVtbl {
        unsafe extern "system" fn AddMapping<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: ::windows::core::RawPtr, localename: super::super::Foundation::PWSTR, basefamilyname: super::super::Foundation::PWSTR, scale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMappings<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFallback<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddMapping::<Impl, IMPL_OFFSET>, AddMappings::<Impl, IMPL_OFFSET>, CreateFontFallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFallbackBuilder as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontFamilyImpl: Sized + IDWriteFontListImpl {
    fn GetFamilyNames();
    fn GetFirstMatchingFont();
    fn GetMatchingFonts();
}
impl IDWriteFontFamilyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamilyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFamilyVtbl {
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstMatchingFont<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontCollection::<Impl, IMPL_OFFSET>, GetFontCount::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>, GetFamilyNames::<Impl, IMPL_OFFSET>, GetFirstMatchingFont::<Impl, IMPL_OFFSET>, GetMatchingFonts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontFamily1Impl: Sized + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
impl IDWriteFontFamily1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamily1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFamily1Vtbl {
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFirstMatchingFont::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFontLocality::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily1 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontFamily2Impl: Sized + IDWriteFontFamily1Impl + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetMatchingFonts();
    fn GetFontSet();
}
impl IDWriteFontFamily2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamily2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFamily2Vtbl {
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontFamily2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontFamily2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            GetFamilyNames::<Impl, IMPL_OFFSET>,
            GetFirstMatchingFont::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFontLocality::<Impl, IMPL_OFFSET>,
            GetFont::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFontSet::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFamily2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFileImpl: Sized {
    fn GetReferenceKey();
    fn GetLoader();
    fn Analyze();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFileVtbl {
        unsafe extern "system" fn GetReferenceKey<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLoader<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Analyze<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetReferenceKey::<Impl, IMPL_OFFSET>, GetLoader::<Impl, IMPL_OFFSET>, Analyze::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFileEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentFontFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontFileEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFileEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IDWriteFontFileEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentFontFile<Impl: IDWriteFontFileEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MoveNext::<Impl, IMPL_OFFSET>, GetCurrentFontFile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontFileLoaderImpl: Sized {
    fn CreateStreamFromKey();
}
impl IDWriteFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFileLoaderVtbl {
        unsafe extern "system" fn CreateStreamFromKey<Impl: IDWriteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStreamFromKey::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileLoader as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontFileStreamImpl: Sized {
    fn ReadFileFragment();
    fn ReleaseFileFragment();
    fn GetFileSize();
    fn GetLastWriteTime();
}
impl IDWriteFontFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontFileStreamVtbl {
        unsafe extern "system" fn ReadFileFragment<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFileFragment<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentcontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastWriteTime<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReadFileFragment::<Impl, IMPL_OFFSET>, ReleaseFileFragment::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, GetLastWriteTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontFileStream as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontListImpl: Sized {
    fn GetFontCollection();
    fn GetFontCount();
    fn GetFont();
}
impl IDWriteFontListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontListVtbl {
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontCount<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontCollection::<Impl, IMPL_OFFSET>, GetFontCount::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontList1Impl: Sized + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
impl IDWriteFontList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontList1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontList1Vtbl {
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontCollection::<Impl, IMPL_OFFSET>, GetFontCount::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>, GetFontLocality::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>, GetFontFaceReference::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList1 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteFontList2Impl: Sized + IDWriteFontList1Impl + IDWriteFontListImpl {
    fn GetFontSet();
}
impl IDWriteFontList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontList2Vtbl {
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontCollection::<Impl, IMPL_OFFSET>, GetFontCount::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>, GetFontLocality::<Impl, IMPL_OFFSET>, GetFont::<Impl, IMPL_OFFSET>, GetFontFaceReference::<Impl, IMPL_OFFSET>, GetFontSet::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontResourceImpl: Sized {
    fn GetFontFile();
    fn GetFontFaceIndex();
    fn GetFontAxisCount();
    fn GetDefaultFontAxisValues();
    fn GetFontAxisRanges();
    fn GetFontAxisAttributes();
    fn GetAxisNames();
    fn GetAxisValueNameCount();
    fn GetAxisValueNames();
    fn HasVariations();
    fn CreateFontFace();
    fn CreateFontFaceReference();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontResourceVtbl {
        unsafe extern "system" fn GetFontFile<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisCount<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisAttributes<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAxisNames<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAxisValueNameCount<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAxisValueNames<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasVariations<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontFile::<Impl, IMPL_OFFSET>,
            GetFontFaceIndex::<Impl, IMPL_OFFSET>,
            GetFontAxisCount::<Impl, IMPL_OFFSET>,
            GetDefaultFontAxisValues::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontAxisAttributes::<Impl, IMPL_OFFSET>,
            GetAxisNames::<Impl, IMPL_OFFSET>,
            GetAxisValueNameCount::<Impl, IMPL_OFFSET>,
            GetAxisValueNames::<Impl, IMPL_OFFSET>,
            HasVariations::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            CreateFontFaceReference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSetImpl: Sized {
    fn GetFontCount();
    fn GetFontFaceReference();
    fn FindFontFaceReference();
    fn FindFontFace();
    fn GetPropertyValues();
    fn GetPropertyValues();
    fn GetPropertyValues();
    fn GetPropertyOccurrenceCount();
    fn GetMatchingFonts();
    fn GetMatchingFonts();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSetVtbl {
        unsafe extern "system" fn GetFontCount<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFontFaceReference<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFontFace<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: super::super::Foundation::PWSTR, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFace::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyOccurrenceCount::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet1Impl: Sized + IDWriteFontSetImpl {
    fn GetMatchingFonts();
    fn GetFirstFontResources();
    fn GetFilteredFonts();
    fn GetFilteredFonts();
    fn GetFilteredFonts();
    fn GetFilteredFontIndices();
    fn GetFilteredFontIndices();
    fn GetFontAxisRanges();
    fn GetFontAxisRanges();
    fn GetFontFaceReference();
    fn CreateFontResource();
    fn CreateFontFace();
    fn GetFontLocality();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSet1Vtbl {
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstFontResources<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredFontIndices<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredFontIndices<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontResource<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFace::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyOccurrenceCount::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFirstFontResources::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetFontLocality::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet2Impl: Sized + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetExpirationEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSet2Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Impl: IDWriteFontSet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFace::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyOccurrenceCount::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFirstFontResources::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetFontLocality::<Impl, IMPL_OFFSET>,
            GetExpirationEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet3Impl: Sized + IDWriteFontSet2Impl + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetFontSourceType();
    fn GetFontSourceNameLength();
    fn GetFontSourceName();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSet3Vtbl {
        unsafe extern "system" fn GetFontSourceType<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSourceNameLength<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSourceName<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: super::super::Foundation::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFontCount::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFaceReference::<Impl, IMPL_OFFSET>,
            FindFontFace::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyOccurrenceCount::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetMatchingFonts::<Impl, IMPL_OFFSET>,
            GetFirstFontResources::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFonts::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFilteredFontIndices::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontAxisRanges::<Impl, IMPL_OFFSET>,
            GetFontFaceReference::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateFontFace::<Impl, IMPL_OFFSET>,
            GetFontLocality::<Impl, IMPL_OFFSET>,
            GetExpirationEvent::<Impl, IMPL_OFFSET>,
            GetFontSourceType::<Impl, IMPL_OFFSET>,
            GetFontSourceNameLength::<Impl, IMPL_OFFSET>,
            GetFontSourceName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSet3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSetBuilderImpl: Sized {
    fn AddFontFaceReference();
    fn AddFontFaceReference();
    fn AddFontSet();
    fn CreateFontSet();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSetBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSetBuilderVtbl {
        unsafe extern "system" fn AddFontFaceReference<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFontFaceReference<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFontSet<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontSet<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontSet::<Impl, IMPL_OFFSET>, CreateFontSet::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSetBuilder1Impl: Sized + IDWriteFontSetBuilderImpl {
    fn AddFontFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSetBuilder1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilder1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSetBuilder1Vtbl {
        unsafe extern "system" fn AddFontFile<Impl: IDWriteFontSetBuilder1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontSet::<Impl, IMPL_OFFSET>, CreateFontSet::<Impl, IMPL_OFFSET>, AddFontFile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSetBuilder2Impl: Sized + IDWriteFontSetBuilder1Impl + IDWriteFontSetBuilderImpl {
    fn AddFont();
    fn AddFontFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteFontSetBuilder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilder2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteFontSetBuilder2Vtbl {
        unsafe extern "system" fn AddFont<Impl: IDWriteFontSetBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFontFile<Impl: IDWriteFontSetBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontFaceReference::<Impl, IMPL_OFFSET>, AddFontSet::<Impl, IMPL_OFFSET>, CreateFontSet::<Impl, IMPL_OFFSET>, AddFontFile::<Impl, IMPL_OFFSET>, AddFont::<Impl, IMPL_OFFSET>, AddFontFile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInteropImpl: Sized {
    fn CreateFontFromLOGFONT();
    fn ConvertFontToLOGFONT();
    fn ConvertFontFaceToLOGFONT();
    fn CreateFontFaceFromHdc();
    fn CreateBitmapRenderTarget();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDWriteGdiInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGdiInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteGdiInteropVtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, logfont: *mut super::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateFontFromLOGFONT::<Impl, IMPL_OFFSET>, ConvertFontToLOGFONT::<Impl, IMPL_OFFSET>, ConvertFontFaceToLOGFONT::<Impl, IMPL_OFFSET>, CreateFontFaceFromHdc::<Impl, IMPL_OFFSET>, CreateBitmapRenderTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGdiInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop1Impl: Sized + IDWriteGdiInteropImpl {
    fn CreateFontFromLOGFONT();
    fn GetFontSignature();
    fn GetFontSignature();
    fn GetMatchingFontsByLOGFONT();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl IDWriteGdiInterop1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGdiInterop1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteGdiInterop1Vtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: ::windows::core::RawPtr, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSignature<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSignature<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: ::windows::core::RawPtr, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateFontFromLOGFONT::<Impl, IMPL_OFFSET>,
            ConvertFontToLOGFONT::<Impl, IMPL_OFFSET>,
            ConvertFontFaceToLOGFONT::<Impl, IMPL_OFFSET>,
            CreateFontFaceFromHdc::<Impl, IMPL_OFFSET>,
            CreateBitmapRenderTarget::<Impl, IMPL_OFFSET>,
            CreateFontFromLOGFONT::<Impl, IMPL_OFFSET>,
            GetFontSignature::<Impl, IMPL_OFFSET>,
            GetFontSignature::<Impl, IMPL_OFFSET>,
            GetMatchingFontsByLOGFONT::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGdiInterop1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteGlyphRunAnalysisImpl: Sized {
    fn GetAlphaTextureBounds();
    fn CreateAlphaTexture();
    fn GetAlphaBlendParams();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteGlyphRunAnalysisVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGlyphRunAnalysisImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteGlyphRunAnalysisVtbl {
        unsafe extern "system" fn GetAlphaTextureBounds<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAlphaTexture<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlphaBlendParams<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: ::windows::core::RawPtr, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAlphaTextureBounds::<Impl, IMPL_OFFSET>, CreateAlphaTexture::<Impl, IMPL_OFFSET>, GetAlphaBlendParams::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteGlyphRunAnalysis as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteInMemoryFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateInMemoryFontFileReference();
    fn GetFileCount();
}
impl IDWriteInMemoryFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteInMemoryFontFileLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteInMemoryFontFileLoaderVtbl {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Impl: IDWriteInMemoryFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileCount<Impl: IDWriteInMemoryFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStreamFromKey::<Impl, IMPL_OFFSET>, CreateInMemoryFontFileReference::<Impl, IMPL_OFFSET>, GetFileCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteInMemoryFontFileLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteInlineObjectImpl: Sized {
    fn Draw();
    fn GetMetrics();
    fn GetOverhangMetrics();
    fn GetBreakConditions();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteInlineObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteInlineObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteInlineObjectVtbl {
        unsafe extern "system" fn Draw<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::windows::core::RawPtr, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverhangMetrics<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakConditions<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Draw::<Impl, IMPL_OFFSET>, GetMetrics::<Impl, IMPL_OFFSET>, GetOverhangMetrics::<Impl, IMPL_OFFSET>, GetBreakConditions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteInlineObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn GetFilePathLengthFromKey();
    fn GetFilePathFromKey();
    fn GetLastWriteTimeFromKey();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteLocalFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteLocalFontFileLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteLocalFontFileLoaderVtbl {
        unsafe extern "system" fn GetFilePathLengthFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilePathFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: super::super::Foundation::PWSTR, filepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStreamFromKey::<Impl, IMPL_OFFSET>, GetFilePathLengthFromKey::<Impl, IMPL_OFFSET>, GetFilePathFromKey::<Impl, IMPL_OFFSET>, GetLastWriteTimeFromKey::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteLocalFontFileLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalizedStringsImpl: Sized {
    fn GetCount();
    fn FindLocaleName();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteLocalizedStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteLocalizedStringsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteLocalizedStringsVtbl {
        unsafe extern "system" fn GetCount<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindLocaleName<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, localename: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringLength<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stringbuffer: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, FindLocaleName::<Impl, IMPL_OFFSET>, GetLocaleNameLength::<Impl, IMPL_OFFSET>, GetLocaleName::<Impl, IMPL_OFFSET>, GetStringLength::<Impl, IMPL_OFFSET>, GetString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteLocalizedStrings as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteNumberSubstitutionImpl: Sized {}
impl IDWriteNumberSubstitutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteNumberSubstitutionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteNumberSubstitutionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWritePixelSnappingImpl: Sized {
    fn IsPixelSnappingDisabled();
    fn GetCurrentTransform();
    fn GetPixelsPerDip();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWritePixelSnappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWritePixelSnappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWritePixelSnappingVtbl {
        unsafe extern "system" fn IsPixelSnappingDisabled<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentTransform<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelsPerDip<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPixelSnappingDisabled::<Impl, IMPL_OFFSET>, GetCurrentTransform::<Impl, IMPL_OFFSET>, GetPixelsPerDip::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWritePixelSnapping as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteRemoteFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateRemoteStreamFromKey();
    fn GetLocalityFromKey();
    fn CreateFontFileReferenceFromUrl();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteRemoteFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRemoteFontFileLoaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRemoteFontFileLoaderVtbl {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalityFromKey<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, baseurl: super::super::Foundation::PWSTR, fontfileurl: super::super::Foundation::PWSTR, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStreamFromKey::<Impl, IMPL_OFFSET>, CreateRemoteStreamFromKey::<Impl, IMPL_OFFSET>, GetLocalityFromKey::<Impl, IMPL_OFFSET>, CreateFontFileReferenceFromUrl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteRemoteFontFileStreamImpl: Sized + IDWriteFontFileStreamImpl {
    fn GetLocalFileSize();
    fn GetFileFragmentLocality();
    fn GetLocality();
    fn BeginDownload();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteRemoteFontFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRemoteFontFileStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRemoteFontFileStreamVtbl {
        unsafe extern "system" fn GetLocalFileSize<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localfilesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileFragmentLocality<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDownload<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadoperationid: *const ::windows::core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReadFileFragment::<Impl, IMPL_OFFSET>, ReleaseFileFragment::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, GetLastWriteTime::<Impl, IMPL_OFFSET>, GetLocalFileSize::<Impl, IMPL_OFFSET>, GetFileFragmentLocality::<Impl, IMPL_OFFSET>, GetLocality::<Impl, IMPL_OFFSET>, BeginDownload::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileStream as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteRenderingParamsImpl: Sized {
    fn GetGamma();
    fn GetEnhancedContrast();
    fn GetClearTypeLevel();
    fn GetPixelGeometry();
    fn GetRenderingMode();
}
impl IDWriteRenderingParamsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParamsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRenderingParamsVtbl {
        unsafe extern "system" fn GetGamma<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnhancedContrast<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClearTypeLevel<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPixelGeometry<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRenderingMode<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGamma::<Impl, IMPL_OFFSET>, GetEnhancedContrast::<Impl, IMPL_OFFSET>, GetClearTypeLevel::<Impl, IMPL_OFFSET>, GetPixelGeometry::<Impl, IMPL_OFFSET>, GetRenderingMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams1Impl: Sized + IDWriteRenderingParamsImpl {
    fn GetGrayscaleEnhancedContrast();
}
impl IDWriteRenderingParams1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRenderingParams1Vtbl {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Impl: IDWriteRenderingParams1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGamma::<Impl, IMPL_OFFSET>, GetEnhancedContrast::<Impl, IMPL_OFFSET>, GetClearTypeLevel::<Impl, IMPL_OFFSET>, GetPixelGeometry::<Impl, IMPL_OFFSET>, GetRenderingMode::<Impl, IMPL_OFFSET>, GetGrayscaleEnhancedContrast::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams1 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams2Impl: Sized + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetGridFitMode();
}
impl IDWriteRenderingParams2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRenderingParams2Vtbl {
        unsafe extern "system" fn GetGridFitMode<Impl: IDWriteRenderingParams2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GRID_FIT_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGamma::<Impl, IMPL_OFFSET>, GetEnhancedContrast::<Impl, IMPL_OFFSET>, GetClearTypeLevel::<Impl, IMPL_OFFSET>, GetPixelGeometry::<Impl, IMPL_OFFSET>, GetRenderingMode::<Impl, IMPL_OFFSET>, GetGrayscaleEnhancedContrast::<Impl, IMPL_OFFSET>, GetGridFitMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams2 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams3Impl: Sized + IDWriteRenderingParams2Impl + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetRenderingMode1();
}
impl IDWriteRenderingParams3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteRenderingParams3Vtbl {
        unsafe extern "system" fn GetRenderingMode1<Impl: IDWriteRenderingParams3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE1 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGamma::<Impl, IMPL_OFFSET>, GetEnhancedContrast::<Impl, IMPL_OFFSET>, GetClearTypeLevel::<Impl, IMPL_OFFSET>, GetPixelGeometry::<Impl, IMPL_OFFSET>, GetRenderingMode::<Impl, IMPL_OFFSET>, GetGrayscaleEnhancedContrast::<Impl, IMPL_OFFSET>, GetGridFitMode::<Impl, IMPL_OFFSET>, GetRenderingMode1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteRenderingParams3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteStringListImpl: Sized {
    fn GetCount();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteStringListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteStringListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteStringListVtbl {
        unsafe extern "system" fn GetCount<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, localename: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringLength<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: super::super::Foundation::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetLocaleNameLength::<Impl, IMPL_OFFSET>, GetLocaleName::<Impl, IMPL_OFFSET>, GetStringLength::<Impl, IMPL_OFFSET>, GetString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteStringList as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSinkImpl: Sized {
    fn SetScriptAnalysis();
    fn SetLineBreakpoints();
    fn SetBidiLevel();
    fn SetNumberSubstitution();
}
impl IDWriteTextAnalysisSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalysisSinkVtbl {
        unsafe extern "system" fn SetScriptAnalysis<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLineBreakpoints<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNumberSubstitution<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetScriptAnalysis::<Impl, IMPL_OFFSET>, SetLineBreakpoints::<Impl, IMPL_OFFSET>, SetBidiLevel::<Impl, IMPL_OFFSET>, SetNumberSubstitution::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalysisSink1Impl: Sized + IDWriteTextAnalysisSinkImpl {
    fn SetGlyphOrientation();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalysisSink1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSink1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalysisSink1Vtbl {
        unsafe extern "system" fn SetGlyphOrientation<Impl: IDWriteTextAnalysisSink1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetScriptAnalysis::<Impl, IMPL_OFFSET>, SetLineBreakpoints::<Impl, IMPL_OFFSET>, SetBidiLevel::<Impl, IMPL_OFFSET>, SetNumberSubstitution::<Impl, IMPL_OFFSET>, SetGlyphOrientation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink1 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSourceImpl: Sized {
    fn GetTextAtPosition();
    fn GetTextBeforePosition();
    fn GetParagraphReadingDirection();
    fn GetLocaleName();
    fn GetNumberSubstitution();
}
impl IDWriteTextAnalysisSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalysisSourceVtbl {
        unsafe extern "system" fn GetTextAtPosition<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextBeforePosition<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberSubstitution<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTextAtPosition::<Impl, IMPL_OFFSET>, GetTextBeforePosition::<Impl, IMPL_OFFSET>, GetParagraphReadingDirection::<Impl, IMPL_OFFSET>, GetLocaleName::<Impl, IMPL_OFFSET>, GetNumberSubstitution::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSource1Impl: Sized + IDWriteTextAnalysisSourceImpl {
    fn GetVerticalGlyphOrientation();
}
impl IDWriteTextAnalysisSource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSource1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalysisSource1Vtbl {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextAnalysisSource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTextAtPosition::<Impl, IMPL_OFFSET>, GetTextBeforePosition::<Impl, IMPL_OFFSET>, GetParagraphReadingDirection::<Impl, IMPL_OFFSET>, GetLocaleName::<Impl, IMPL_OFFSET>, GetNumberSubstitution::<Impl, IMPL_OFFSET>, GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzerImpl: Sized {
    fn AnalyzeScript();
    fn AnalyzeBidi();
    fn AnalyzeNumberSubstitution();
    fn AnalyzeLineBreakpoints();
    fn GetGlyphs();
    fn GetGlyphPlacements();
    fn GetGdiCompatibleGlyphPlacements();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalyzerVtbl {
        unsafe extern "system" fn AnalyzeScript<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeBidi<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphs<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textstring: super::super::Foundation::PWSTR, textlength: u32, fontface: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, numbersubstitution: ::windows::core::RawPtr, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphPlacements<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            textstring: super::super::Foundation::PWSTR,
            clustermap: *const u16,
            textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: ::windows::core::RawPtr,
            fontemsize: f32,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: super::super::Foundation::PWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphPlacements<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            textstring: super::super::Foundation::PWSTR,
            clustermap: *const u16,
            textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: ::windows::core::RawPtr,
            fontemsize: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: super::super::Foundation::BOOL,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: super::super::Foundation::PWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AnalyzeScript::<Impl, IMPL_OFFSET>, AnalyzeBidi::<Impl, IMPL_OFFSET>, AnalyzeNumberSubstitution::<Impl, IMPL_OFFSET>, AnalyzeLineBreakpoints::<Impl, IMPL_OFFSET>, GetGlyphs::<Impl, IMPL_OFFSET>, GetGlyphPlacements::<Impl, IMPL_OFFSET>, GetGdiCompatibleGlyphPlacements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer1Impl: Sized + IDWriteTextAnalyzerImpl {
    fn ApplyCharacterSpacing();
    fn GetBaseline();
    fn AnalyzeVerticalGlyphOrientation();
    fn GetGlyphOrientationTransform();
    fn GetScriptProperties();
    fn GetTextComplexity();
    fn GetJustificationOpportunities();
    fn JustifyGlyphAdvances();
    fn GetJustifiedGlyphs();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzer1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzer1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalyzer1Vtbl {
        unsafe extern "system" fn ApplyCharacterSpacing<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBaseline<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScriptProperties<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextComplexity<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textstring: super::super::Foundation::PWSTR, textlength: u32, fontface: ::windows::core::RawPtr, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJustificationOpportunities<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: super::super::Foundation::PWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AnalyzeScript::<Impl, IMPL_OFFSET>,
            AnalyzeBidi::<Impl, IMPL_OFFSET>,
            AnalyzeNumberSubstitution::<Impl, IMPL_OFFSET>,
            AnalyzeLineBreakpoints::<Impl, IMPL_OFFSET>,
            GetGlyphs::<Impl, IMPL_OFFSET>,
            GetGlyphPlacements::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphPlacements::<Impl, IMPL_OFFSET>,
            ApplyCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetBaseline::<Impl, IMPL_OFFSET>,
            AnalyzeVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetGlyphOrientationTransform::<Impl, IMPL_OFFSET>,
            GetScriptProperties::<Impl, IMPL_OFFSET>,
            GetTextComplexity::<Impl, IMPL_OFFSET>,
            GetJustificationOpportunities::<Impl, IMPL_OFFSET>,
            JustifyGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetJustifiedGlyphs::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer2Impl: Sized + IDWriteTextAnalyzer1Impl + IDWriteTextAnalyzerImpl {
    fn GetGlyphOrientationTransform();
    fn GetTypographicFeatures();
    fn CheckTypographicFeature();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextAnalyzer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextAnalyzer2Vtbl {
        unsafe extern "system" fn GetGlyphOrientationTransform<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypographicFeatures<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckTypographicFeature<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AnalyzeScript::<Impl, IMPL_OFFSET>,
            AnalyzeBidi::<Impl, IMPL_OFFSET>,
            AnalyzeNumberSubstitution::<Impl, IMPL_OFFSET>,
            AnalyzeLineBreakpoints::<Impl, IMPL_OFFSET>,
            GetGlyphs::<Impl, IMPL_OFFSET>,
            GetGlyphPlacements::<Impl, IMPL_OFFSET>,
            GetGdiCompatibleGlyphPlacements::<Impl, IMPL_OFFSET>,
            ApplyCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetBaseline::<Impl, IMPL_OFFSET>,
            AnalyzeVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetGlyphOrientationTransform::<Impl, IMPL_OFFSET>,
            GetScriptProperties::<Impl, IMPL_OFFSET>,
            GetTextComplexity::<Impl, IMPL_OFFSET>,
            GetJustificationOpportunities::<Impl, IMPL_OFFSET>,
            JustifyGlyphAdvances::<Impl, IMPL_OFFSET>,
            GetJustifiedGlyphs::<Impl, IMPL_OFFSET>,
            GetGlyphOrientationTransform::<Impl, IMPL_OFFSET>,
            GetTypographicFeatures::<Impl, IMPL_OFFSET>,
            CheckTypographicFeature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormatImpl: Sized {
    fn SetTextAlignment();
    fn SetParagraphAlignment();
    fn SetWordWrapping();
    fn SetReadingDirection();
    fn SetFlowDirection();
    fn SetIncrementalTabStop();
    fn SetTrimming();
    fn SetLineSpacing();
    fn GetTextAlignment();
    fn GetParagraphAlignment();
    fn GetWordWrapping();
    fn GetReadingDirection();
    fn GetFlowDirection();
    fn GetIncrementalTabStop();
    fn GetTrimming();
    fn GetLineSpacing();
    fn GetFontCollection();
    fn GetFontFamilyNameLength();
    fn GetFontFamilyName();
    fn GetFontWeight();
    fn GetFontStyle();
    fn GetFontStretch();
    fn GetFontSize();
    fn GetLocaleNameLength();
    fn GetLocaleName();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextFormatVtbl {
        unsafe extern "system" fn SetTextAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParagraphAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWordWrapping<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReadingDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlowDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIncrementalTabStop<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, incrementaltabstop: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrimming<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParagraphAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWordWrapping<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_WORD_WRAPPING {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadingDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlowDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FLOW_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIncrementalTabStop<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrimming<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamilyName<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontWeight<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontStyle<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontStretch<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSize<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat1Impl: Sized + IDWriteTextFormatImpl {
    fn SetVerticalGlyphOrientation();
    fn GetVerticalGlyphOrientation();
    fn SetLastLineWrapping();
    fn GetLastLineWrapping();
    fn SetOpticalAlignment();
    fn GetOpticalAlignment();
    fn SetFontFallback();
    fn GetFontFallback();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextFormat1Vtbl {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastLineWrapping<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastLineWrapping<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpticalAlignment<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpticalAlignment<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontFallback<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFallback<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat2Impl: Sized + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetLineSpacing();
    fn GetLineSpacing();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextFormat2Vtbl {
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat3Impl: Sized + IDWriteTextFormat2Impl + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextFormat3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextFormat3Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            SetFontAxisValues::<Impl, IMPL_OFFSET>,
            GetFontAxisValueCount::<Impl, IMPL_OFFSET>,
            GetFontAxisValues::<Impl, IMPL_OFFSET>,
            GetAutomaticFontAxes::<Impl, IMPL_OFFSET>,
            SetAutomaticFontAxes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextFormat3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayoutImpl: Sized + IDWriteTextFormatImpl {
    fn SetMaxWidth();
    fn SetMaxHeight();
    fn SetFontCollection();
    fn SetFontFamilyName();
    fn SetFontWeight();
    fn SetFontStyle();
    fn SetFontStretch();
    fn SetFontSize();
    fn SetUnderline();
    fn SetStrikethrough();
    fn SetDrawingEffect();
    fn SetInlineObject();
    fn SetTypography();
    fn SetLocaleName();
    fn GetMaxWidth();
    fn GetMaxHeight();
    fn GetFontCollection();
    fn GetFontFamilyNameLength();
    fn GetFontFamilyName();
    fn GetFontWeight();
    fn GetFontStyle();
    fn GetFontStretch();
    fn GetFontSize();
    fn GetUnderline();
    fn GetStrikethrough();
    fn GetDrawingEffect();
    fn GetInlineObject();
    fn GetTypography();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn Draw();
    fn GetLineMetrics();
    fn GetMetrics();
    fn GetOverhangMetrics();
    fn GetClusterMetrics();
    fn DetermineMinWidth();
    fn HitTestPoint();
    fn HitTestTextPosition();
    fn HitTestTextRange();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextLayoutVtbl {
        unsafe extern "system" fn SetMaxWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxHeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontCollection<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontFamilyName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontWeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontStyle<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontStretch<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontSize<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnderline<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDrawingEffect<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInlineObject<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inlineobject: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypography<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocaleName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxHeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontcollection: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFamilyName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontfamilyname: super::super::Foundation::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontWeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontStyle<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontStretch<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSize<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnderline<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrikethrough<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDrawingEffect<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInlineObject<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, inlineobject: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypography<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, typography: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, localename: super::super::Foundation::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::windows::core::RawPtr, originx: f32, originy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOverhangMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClusterMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetermineMinWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestPoint<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestTextPosition<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestTextRange<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetMaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxHeight::<Impl, IMPL_OFFSET>,
            SetFontCollection::<Impl, IMPL_OFFSET>,
            SetFontFamilyName::<Impl, IMPL_OFFSET>,
            SetFontWeight::<Impl, IMPL_OFFSET>,
            SetFontStyle::<Impl, IMPL_OFFSET>,
            SetFontStretch::<Impl, IMPL_OFFSET>,
            SetFontSize::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            SetDrawingEffect::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            SetTypography::<Impl, IMPL_OFFSET>,
            SetLocaleName::<Impl, IMPL_OFFSET>,
            GetMaxWidth::<Impl, IMPL_OFFSET>,
            GetMaxHeight::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            GetStrikethrough::<Impl, IMPL_OFFSET>,
            GetDrawingEffect::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetTypography::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetOverhangMetrics::<Impl, IMPL_OFFSET>,
            GetClusterMetrics::<Impl, IMPL_OFFSET>,
            DetermineMinWidth::<Impl, IMPL_OFFSET>,
            HitTestPoint::<Impl, IMPL_OFFSET>,
            HitTestTextPosition::<Impl, IMPL_OFFSET>,
            HitTestTextRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout1Impl: Sized + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetPairKerning();
    fn GetPairKerning();
    fn SetCharacterSpacing();
    fn GetCharacterSpacing();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextLayout1Vtbl {
        unsafe extern "system" fn SetPairKerning<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPairKerning<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCharacterSpacing<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharacterSpacing<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetMaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxHeight::<Impl, IMPL_OFFSET>,
            SetFontCollection::<Impl, IMPL_OFFSET>,
            SetFontFamilyName::<Impl, IMPL_OFFSET>,
            SetFontWeight::<Impl, IMPL_OFFSET>,
            SetFontStyle::<Impl, IMPL_OFFSET>,
            SetFontStretch::<Impl, IMPL_OFFSET>,
            SetFontSize::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            SetDrawingEffect::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            SetTypography::<Impl, IMPL_OFFSET>,
            SetLocaleName::<Impl, IMPL_OFFSET>,
            GetMaxWidth::<Impl, IMPL_OFFSET>,
            GetMaxHeight::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            GetStrikethrough::<Impl, IMPL_OFFSET>,
            GetDrawingEffect::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetTypography::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetOverhangMetrics::<Impl, IMPL_OFFSET>,
            GetClusterMetrics::<Impl, IMPL_OFFSET>,
            DetermineMinWidth::<Impl, IMPL_OFFSET>,
            HitTestPoint::<Impl, IMPL_OFFSET>,
            HitTestTextPosition::<Impl, IMPL_OFFSET>,
            HitTestTextRange::<Impl, IMPL_OFFSET>,
            SetPairKerning::<Impl, IMPL_OFFSET>,
            GetPairKerning::<Impl, IMPL_OFFSET>,
            SetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetCharacterSpacing::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout2Impl: Sized + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn GetMetrics();
    fn SetVerticalGlyphOrientation();
    fn GetVerticalGlyphOrientation();
    fn SetLastLineWrapping();
    fn GetLastLineWrapping();
    fn SetOpticalAlignment();
    fn GetOpticalAlignment();
    fn SetFontFallback();
    fn GetFontFallback();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextLayout2Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastLineWrapping<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastLineWrapping<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpticalAlignment<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpticalAlignment<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontFallback<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFallback<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetMaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxHeight::<Impl, IMPL_OFFSET>,
            SetFontCollection::<Impl, IMPL_OFFSET>,
            SetFontFamilyName::<Impl, IMPL_OFFSET>,
            SetFontWeight::<Impl, IMPL_OFFSET>,
            SetFontStyle::<Impl, IMPL_OFFSET>,
            SetFontStretch::<Impl, IMPL_OFFSET>,
            SetFontSize::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            SetDrawingEffect::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            SetTypography::<Impl, IMPL_OFFSET>,
            SetLocaleName::<Impl, IMPL_OFFSET>,
            GetMaxWidth::<Impl, IMPL_OFFSET>,
            GetMaxHeight::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            GetStrikethrough::<Impl, IMPL_OFFSET>,
            GetDrawingEffect::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetTypography::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetOverhangMetrics::<Impl, IMPL_OFFSET>,
            GetClusterMetrics::<Impl, IMPL_OFFSET>,
            DetermineMinWidth::<Impl, IMPL_OFFSET>,
            HitTestPoint::<Impl, IMPL_OFFSET>,
            HitTestTextPosition::<Impl, IMPL_OFFSET>,
            HitTestTextRange::<Impl, IMPL_OFFSET>,
            SetPairKerning::<Impl, IMPL_OFFSET>,
            GetPairKerning::<Impl, IMPL_OFFSET>,
            SetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout3Impl: Sized + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn InvalidateLayout();
    fn SetLineSpacing();
    fn GetLineSpacing();
    fn GetLineMetrics();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextLayout3Vtbl {
        unsafe extern "system" fn InvalidateLayout<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLineMetrics<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetMaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxHeight::<Impl, IMPL_OFFSET>,
            SetFontCollection::<Impl, IMPL_OFFSET>,
            SetFontFamilyName::<Impl, IMPL_OFFSET>,
            SetFontWeight::<Impl, IMPL_OFFSET>,
            SetFontStyle::<Impl, IMPL_OFFSET>,
            SetFontStretch::<Impl, IMPL_OFFSET>,
            SetFontSize::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            SetDrawingEffect::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            SetTypography::<Impl, IMPL_OFFSET>,
            SetLocaleName::<Impl, IMPL_OFFSET>,
            GetMaxWidth::<Impl, IMPL_OFFSET>,
            GetMaxHeight::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            GetStrikethrough::<Impl, IMPL_OFFSET>,
            GetDrawingEffect::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetTypography::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetOverhangMetrics::<Impl, IMPL_OFFSET>,
            GetClusterMetrics::<Impl, IMPL_OFFSET>,
            DetermineMinWidth::<Impl, IMPL_OFFSET>,
            HitTestPoint::<Impl, IMPL_OFFSET>,
            HitTestTextPosition::<Impl, IMPL_OFFSET>,
            HitTestTextRange::<Impl, IMPL_OFFSET>,
            SetPairKerning::<Impl, IMPL_OFFSET>,
            GetPairKerning::<Impl, IMPL_OFFSET>,
            SetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
            InvalidateLayout::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout4Impl: Sized + IDWriteTextLayout3Impl + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextLayout4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextLayout4Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTextAlignment::<Impl, IMPL_OFFSET>,
            SetParagraphAlignment::<Impl, IMPL_OFFSET>,
            SetWordWrapping::<Impl, IMPL_OFFSET>,
            SetReadingDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection::<Impl, IMPL_OFFSET>,
            SetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            SetTrimming::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetTextAlignment::<Impl, IMPL_OFFSET>,
            GetParagraphAlignment::<Impl, IMPL_OFFSET>,
            GetWordWrapping::<Impl, IMPL_OFFSET>,
            GetReadingDirection::<Impl, IMPL_OFFSET>,
            GetFlowDirection::<Impl, IMPL_OFFSET>,
            GetIncrementalTabStop::<Impl, IMPL_OFFSET>,
            GetTrimming::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            SetMaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxHeight::<Impl, IMPL_OFFSET>,
            SetFontCollection::<Impl, IMPL_OFFSET>,
            SetFontFamilyName::<Impl, IMPL_OFFSET>,
            SetFontWeight::<Impl, IMPL_OFFSET>,
            SetFontStyle::<Impl, IMPL_OFFSET>,
            SetFontStretch::<Impl, IMPL_OFFSET>,
            SetFontSize::<Impl, IMPL_OFFSET>,
            SetUnderline::<Impl, IMPL_OFFSET>,
            SetStrikethrough::<Impl, IMPL_OFFSET>,
            SetDrawingEffect::<Impl, IMPL_OFFSET>,
            SetInlineObject::<Impl, IMPL_OFFSET>,
            SetTypography::<Impl, IMPL_OFFSET>,
            SetLocaleName::<Impl, IMPL_OFFSET>,
            GetMaxWidth::<Impl, IMPL_OFFSET>,
            GetMaxHeight::<Impl, IMPL_OFFSET>,
            GetFontCollection::<Impl, IMPL_OFFSET>,
            GetFontFamilyNameLength::<Impl, IMPL_OFFSET>,
            GetFontFamilyName::<Impl, IMPL_OFFSET>,
            GetFontWeight::<Impl, IMPL_OFFSET>,
            GetFontStyle::<Impl, IMPL_OFFSET>,
            GetFontStretch::<Impl, IMPL_OFFSET>,
            GetFontSize::<Impl, IMPL_OFFSET>,
            GetUnderline::<Impl, IMPL_OFFSET>,
            GetStrikethrough::<Impl, IMPL_OFFSET>,
            GetDrawingEffect::<Impl, IMPL_OFFSET>,
            GetInlineObject::<Impl, IMPL_OFFSET>,
            GetTypography::<Impl, IMPL_OFFSET>,
            GetLocaleNameLength::<Impl, IMPL_OFFSET>,
            GetLocaleName::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            GetOverhangMetrics::<Impl, IMPL_OFFSET>,
            GetClusterMetrics::<Impl, IMPL_OFFSET>,
            DetermineMinWidth::<Impl, IMPL_OFFSET>,
            HitTestPoint::<Impl, IMPL_OFFSET>,
            HitTestTextPosition::<Impl, IMPL_OFFSET>,
            HitTestTextRange::<Impl, IMPL_OFFSET>,
            SetPairKerning::<Impl, IMPL_OFFSET>,
            GetPairKerning::<Impl, IMPL_OFFSET>,
            SetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetCharacterSpacing::<Impl, IMPL_OFFSET>,
            GetMetrics::<Impl, IMPL_OFFSET>,
            SetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            GetVerticalGlyphOrientation::<Impl, IMPL_OFFSET>,
            SetLastLineWrapping::<Impl, IMPL_OFFSET>,
            GetLastLineWrapping::<Impl, IMPL_OFFSET>,
            SetOpticalAlignment::<Impl, IMPL_OFFSET>,
            GetOpticalAlignment::<Impl, IMPL_OFFSET>,
            SetFontFallback::<Impl, IMPL_OFFSET>,
            GetFontFallback::<Impl, IMPL_OFFSET>,
            InvalidateLayout::<Impl, IMPL_OFFSET>,
            SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineMetrics::<Impl, IMPL_OFFSET>,
            SetFontAxisValues::<Impl, IMPL_OFFSET>,
            GetFontAxisValueCount::<Impl, IMPL_OFFSET>,
            GetFontAxisValues::<Impl, IMPL_OFFSET>,
            GetAutomaticFontAxes::<Impl, IMPL_OFFSET>,
            SetAutomaticFontAxes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextLayout4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRendererImpl: Sized + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextRendererVtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawUnderline<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawStrikethrough<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInlineObject<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPixelSnappingDisabled::<Impl, IMPL_OFFSET>, GetCurrentTransform::<Impl, IMPL_OFFSET>, GetPixelsPerDip::<Impl, IMPL_OFFSET>, DrawGlyphRun::<Impl, IMPL_OFFSET>, DrawUnderline::<Impl, IMPL_OFFSET>, DrawStrikethrough::<Impl, IMPL_OFFSET>, DrawInlineObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRenderer1Impl: Sized + IDWriteTextRendererImpl + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IDWriteTextRenderer1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextRenderer1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTextRenderer1Vtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawUnderline<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawStrikethrough<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInlineObject<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            IsPixelSnappingDisabled::<Impl, IMPL_OFFSET>,
            GetCurrentTransform::<Impl, IMPL_OFFSET>,
            GetPixelsPerDip::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawUnderline::<Impl, IMPL_OFFSET>,
            DrawStrikethrough::<Impl, IMPL_OFFSET>,
            DrawInlineObject::<Impl, IMPL_OFFSET>,
            DrawGlyphRun::<Impl, IMPL_OFFSET>,
            DrawUnderline::<Impl, IMPL_OFFSET>,
            DrawStrikethrough::<Impl, IMPL_OFFSET>,
            DrawInlineObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTextRenderer1 as ::windows::core::Interface>::IID
    }
}
pub trait IDWriteTypographyImpl: Sized {
    fn AddFontFeature();
    fn GetFontFeatureCount();
    fn GetFontFeature();
}
impl IDWriteTypographyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTypographyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDWriteTypographyVtbl {
        unsafe extern "system" fn AddFontFeature<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFeatureCount<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFeature<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFontFeature::<Impl, IMPL_OFFSET>, GetFontFeatureCount::<Impl, IMPL_OFFSET>, GetFontFeature::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDWriteTypography as ::windows::core::Interface>::IID
    }
}
