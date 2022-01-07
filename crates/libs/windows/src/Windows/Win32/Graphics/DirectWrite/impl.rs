pub trait IDWriteAsyncResultImpl: Sized {
    fn GetWaitHandle();
    fn GetResult();
}
impl ::windows::core::RuntimeName for IDWriteAsyncResult {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteAsyncResult";
}
impl IDWriteAsyncResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteAsyncResultImpl, const OFFSET: isize>() -> IDWriteAsyncResultVtbl {
        unsafe extern "system" fn GetWaitHandle<Impl: IDWriteAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaitHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Impl: IDWriteAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteAsyncResult>, ::windows::core::GetTrustLevel, GetWaitHandle::<Impl, OFFSET>, GetResult::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteBitmapRenderTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteBitmapRenderTarget";
}
impl IDWriteBitmapRenderTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>() -> IDWriteBitmapRenderTargetVtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: ::windows::core::RawPtr, textcolor: u32, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawGlyphRun(baselineoriginx, baselineoriginy, measuringmode, &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), &*(&renderingparams as *const <IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType), textcolor, ::core::mem::transmute_copy(&blackboxrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemoryDC<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemoryDC() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelsPerDip<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelsPerDip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelsPerDip<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixelsperdip: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPixelsPerDip(pixelsperdip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentTransform(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentTransform<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentTransform(&*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: IDWriteBitmapRenderTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteBitmapRenderTarget>,
            ::windows::core::GetTrustLevel,
            DrawGlyphRun::<Impl, OFFSET>,
            GetMemoryDC::<Impl, OFFSET>,
            GetPixelsPerDip::<Impl, OFFSET>,
            SetPixelsPerDip::<Impl, OFFSET>,
            GetCurrentTransform::<Impl, OFFSET>,
            SetCurrentTransform::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            Resize::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteBitmapRenderTarget1Impl: Sized + IDWriteBitmapRenderTargetImpl {
    fn GetTextAntialiasMode();
    fn SetTextAntialiasMode();
}
impl ::windows::core::RuntimeName for IDWriteBitmapRenderTarget1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteBitmapRenderTarget1";
}
impl IDWriteBitmapRenderTarget1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteBitmapRenderTarget1Impl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget1Vtbl {
        unsafe extern "system" fn GetTextAntialiasMode<Impl: IDWriteBitmapRenderTarget1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAntialiasMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Impl: IDWriteBitmapRenderTarget1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextAntialiasMode(antialiasmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteBitmapRenderTarget1>, ::windows::core::GetTrustLevel, GetTextAntialiasMode::<Impl, OFFSET>, SetTextAntialiasMode::<Impl, OFFSET>)
    }
}
pub trait IDWriteColorGlyphRunEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentRun();
}
impl ::windows::core::RuntimeName for IDWriteColorGlyphRunEnumerator {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteColorGlyphRunEnumerator";
}
impl IDWriteColorGlyphRunEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteColorGlyphRunEnumeratorImpl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IDWriteColorGlyphRunEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasrun)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentRun<Impl: IDWriteColorGlyphRunEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRun(::core::mem::transmute_copy(&colorglyphrun)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteColorGlyphRunEnumerator>, ::windows::core::GetTrustLevel, MoveNext::<Impl, OFFSET>, GetCurrentRun::<Impl, OFFSET>)
    }
}
pub trait IDWriteColorGlyphRunEnumerator1Impl: Sized + IDWriteColorGlyphRunEnumeratorImpl {
    fn GetCurrentRun();
}
impl ::windows::core::RuntimeName for IDWriteColorGlyphRunEnumerator1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteColorGlyphRunEnumerator1";
}
impl IDWriteColorGlyphRunEnumerator1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteColorGlyphRunEnumerator1Impl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator1Vtbl {
        unsafe extern "system" fn GetCurrentRun<Impl: IDWriteColorGlyphRunEnumerator1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRun(::core::mem::transmute_copy(&colorglyphrun)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteColorGlyphRunEnumerator1>, ::windows::core::GetTrustLevel, GetCurrentRun::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory";
}
impl IDWriteFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactoryImpl, const OFFSET: isize>() -> IDWriteFactoryVtbl {
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontCollection(::core::mem::transmute_copy(&fontcollection), &*(&checkforupdates as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomFontCollection<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionloader: ::windows::core::RawPtr, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomFontCollection(&*(&collectionloader as *const <IDWriteFontCollectionLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontCollectionLoader as ::windows::core::DefaultType>::DefaultType), &*(&collectionkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), collectionkeysize, ::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterFontCollectionLoader(&*(&fontcollectionloader as *const <IDWriteFontCollectionLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontCollectionLoader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollectionloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterFontCollectionLoader(&*(&fontcollectionloader as *const <IDWriteFontCollectionLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontCollectionLoader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFileReference<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFileReference(&*(&filepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lastwritetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: ::windows::core::RawPtr, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomFontFileReference(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, &*(&fontfileloader as *const <IDWriteFontFileLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontFileLoader as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const ::windows::core::RawPtr, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(fontfacetype, numberoffiles, &*(&fontfiles as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType), faceindex, fontfacesimulationflags, ::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderingParams(::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMonitorRenderingParams(&*(&monitor as *const <super::Gdi::HMONITOR as ::windows::core::Abi>::Abi as *const <super::Gdi::HMONITOR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, ::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterFontFileLoader(&*(&fontfileloader as *const <IDWriteFontFileLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontFileLoader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterFontFileLoader(&*(&fontfileloader as *const <IDWriteFontFileLoader as ::windows::core::Abi>::Abi as *const <IDWriteFontFileLoader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextFormat<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, fontcollection: ::windows::core::RawPtr, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: super::super::Foundation::PWSTR, textformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextFormat(
                &*(&fontfamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType),
                fontweight,
                fontstyle,
                fontstretch,
                fontsize,
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&textformat),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTypography<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTypography(::core::mem::transmute_copy(&typography)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiInterop<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdiinterop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGdiInterop(::core::mem::transmute_copy(&gdiinterop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextLayout<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, maxwidth: f32, maxheight: f32, textlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextLayout(&*(&string as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), stringlength, &*(&textformat as *const <IDWriteTextFormat as ::windows::core::Abi>::Abi as *const <IDWriteTextFormat as ::windows::core::DefaultType>::DefaultType), maxwidth, maxheight, ::core::mem::transmute_copy(&textlayout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: super::super::Foundation::PWSTR, stringlength: u32, textformat: ::windows::core::RawPtr, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGdiCompatibleTextLayout(
                &*(&string as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                stringlength,
                &*(&textformat as *const <IDWriteTextFormat as ::windows::core::Abi>::Abi as *const <IDWriteTextFormat as ::windows::core::DefaultType>::DefaultType),
                layoutwidth,
                layoutheight,
                pixelsperdip,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&usegdinatural as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&textlayout),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textformat: ::windows::core::RawPtr, trimmingsign: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEllipsisTrimmingSign(&*(&textformat as *const <IDWriteTextFormat as ::windows::core::Abi>::Abi as *const <IDWriteTextFormat as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&trimmingsign)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textanalyzer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextAnalyzer(::core::mem::transmute_copy(&textanalyzer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: super::super::Foundation::PWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNumberSubstitution(substitutionmethod, &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&ignoreuseroverride as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&numbersubstitution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGlyphRunAnalysis(&*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), pixelsperdip, &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), renderingmode, measuringmode, baselineoriginx, baselineoriginy, ::core::mem::transmute_copy(&glyphrunanalysis)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFactory>,
            ::windows::core::GetTrustLevel,
            GetSystemFontCollection::<Impl, OFFSET>,
            CreateCustomFontCollection::<Impl, OFFSET>,
            RegisterFontCollectionLoader::<Impl, OFFSET>,
            UnregisterFontCollectionLoader::<Impl, OFFSET>,
            CreateFontFileReference::<Impl, OFFSET>,
            CreateCustomFontFileReference::<Impl, OFFSET>,
            CreateFontFace::<Impl, OFFSET>,
            CreateRenderingParams::<Impl, OFFSET>,
            CreateMonitorRenderingParams::<Impl, OFFSET>,
            CreateCustomRenderingParams::<Impl, OFFSET>,
            RegisterFontFileLoader::<Impl, OFFSET>,
            UnregisterFontFileLoader::<Impl, OFFSET>,
            CreateTextFormat::<Impl, OFFSET>,
            CreateTypography::<Impl, OFFSET>,
            GetGdiInterop::<Impl, OFFSET>,
            CreateTextLayout::<Impl, OFFSET>,
            CreateGdiCompatibleTextLayout::<Impl, OFFSET>,
            CreateEllipsisTrimmingSign::<Impl, OFFSET>,
            CreateTextAnalyzer::<Impl, OFFSET>,
            CreateNumberSubstitution::<Impl, OFFSET>,
            CreateGlyphRunAnalysis::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFactory1Impl: Sized + IDWriteFactoryImpl {
    fn GetEudcFontCollection();
    fn CreateCustomRenderingParams();
}
impl ::windows::core::RuntimeName for IDWriteFactory1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory1";
}
impl IDWriteFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory1Impl, const OFFSET: isize>() -> IDWriteFactory1Vtbl {
        unsafe extern "system" fn GetEudcFontCollection<Impl: IDWriteFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEudcFontCollection(::core::mem::transmute_copy(&fontcollection), &*(&checkforupdates as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomRenderingParams(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, ::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFactory1>, ::windows::core::GetTrustLevel, GetEudcFontCollection::<Impl, OFFSET>, CreateCustomRenderingParams::<Impl, OFFSET>)
    }
}
pub trait IDWriteFactory2Impl: Sized + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontFallback();
    fn CreateFontFallbackBuilder();
    fn TranslateColorGlyphRun();
    fn CreateCustomRenderingParams();
    fn CreateGlyphRunAnalysis();
}
impl ::windows::core::RuntimeName for IDWriteFactory2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory2";
}
impl IDWriteFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory2Impl, const OFFSET: isize>() -> IDWriteFactory2Vtbl {
        unsafe extern "system" fn GetSystemFontFallback<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontFallback(::core::mem::transmute_copy(&fontfallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallbackbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFallbackBuilder(::core::mem::transmute_copy(&fontfallbackbuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateColorGlyphRun(
                baselineoriginx,
                baselineoriginy,
                &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrundescription as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                measuringmode,
                &*(&worldtodevicetransform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                colorpaletteindex,
                ::core::mem::transmute_copy(&colorlayers),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomRenderingParams(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, ::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGlyphRunAnalysis(&*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, ::core::mem::transmute_copy(&glyphrunanalysis)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFactory2>, ::windows::core::GetTrustLevel, GetSystemFontFallback::<Impl, OFFSET>, CreateFontFallbackBuilder::<Impl, OFFSET>, TranslateColorGlyphRun::<Impl, OFFSET>, CreateCustomRenderingParams::<Impl, OFFSET>, CreateGlyphRunAnalysis::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFactory3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory3";
}
impl IDWriteFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory3Impl, const OFFSET: isize>() -> IDWriteFactory3Vtbl {
        unsafe extern "system" fn CreateGlyphRunAnalysis<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGlyphRunAnalysis(&*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, ::core::mem::transmute_copy(&glyphrunanalysis)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomRenderingParams(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, ::core::mem::transmute_copy(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceReference(&*(&fontfile as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType), faceindex, fontsimulations, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceReference(&*(&filepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lastwritetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType), faceindex, fontsimulations, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontSetBuilder(::core::mem::transmute_copy(&fontsetbuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontCollectionFromFontSet(&*(&fontset as *const <IDWriteFontSet as ::windows::core::Abi>::Abi as *const <IDWriteFontSet as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut ::windows::core::RawPtr, checkforupdates: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontCollection(&*(&includedownloadablefonts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontcollection), &*(&checkforupdates as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontDownloadQueue<Impl: IDWriteFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontdownloadqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontDownloadQueue(::core::mem::transmute_copy(&fontdownloadqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFactory3>,
            ::windows::core::GetTrustLevel,
            CreateGlyphRunAnalysis::<Impl, OFFSET>,
            CreateCustomRenderingParams::<Impl, OFFSET>,
            CreateFontFaceReference::<Impl, OFFSET>,
            CreateFontFaceReference::<Impl, OFFSET>,
            GetSystemFontSet::<Impl, OFFSET>,
            CreateFontSetBuilder::<Impl, OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, OFFSET>,
            GetSystemFontCollection::<Impl, OFFSET>,
            GetFontDownloadQueue::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFactory4Impl: Sized + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn TranslateColorGlyphRun();
    fn ComputeGlyphOrigins();
    fn ComputeGlyphOrigins();
}
impl ::windows::core::RuntimeName for IDWriteFactory4 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory4";
}
impl IDWriteFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory4Impl, const OFFSET: isize>() -> IDWriteFactory4Vtbl {
        unsafe extern "system" fn TranslateColorGlyphRun<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateColorGlyphRun(
                &*(&baselineorigin as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrundescription as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                desiredglyphimageformats,
                measuringmode,
                &*(&worldanddpitransform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                colorpaletteindex,
                ::core::mem::transmute_copy(&colorlayers),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeGlyphOrigins(&*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType), &*(&baselineorigin as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&glyphorigins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Impl: IDWriteFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeGlyphOrigins(
                &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                measuringmode,
                &*(&baselineorigin as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::D2D_POINT_2F as ::windows::core::DefaultType>::DefaultType),
                &*(&worldanddpitransform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&glyphorigins),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFactory4>, ::windows::core::GetTrustLevel, TranslateColorGlyphRun::<Impl, OFFSET>, ComputeGlyphOrigins::<Impl, OFFSET>, ComputeGlyphOrigins::<Impl, OFFSET>)
    }
}
pub trait IDWriteFactory5Impl: Sized + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontSetBuilder();
    fn CreateInMemoryFontFileLoader();
    fn CreateHttpFontFileLoader();
    fn AnalyzeContainerType();
    fn UnpackFontFile();
}
impl ::windows::core::RuntimeName for IDWriteFactory5 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory5";
}
impl IDWriteFactory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory5Impl, const OFFSET: isize>() -> IDWriteFactory5Vtbl {
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontSetBuilder(::core::mem::transmute_copy(&fontsetbuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInMemoryFontFileLoader(::core::mem::transmute_copy(&newloader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referrerurl: super::super::Foundation::PWSTR, extraheaders: super::super::Foundation::PWSTR, newloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHttpFontFileLoader(&*(&referrerurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&extraheaders as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newloader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeContainerType<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeContainerType(&*(&filedata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), filedatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnpackFontFile<Impl: IDWriteFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnpackFontFile(containertype, &*(&filedata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), filedatasize, ::core::mem::transmute_copy(&unpackedfontstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFactory5>, ::windows::core::GetTrustLevel, CreateFontSetBuilder::<Impl, OFFSET>, CreateInMemoryFontFileLoader::<Impl, OFFSET>, CreateHttpFontFileLoader::<Impl, OFFSET>, AnalyzeContainerType::<Impl, OFFSET>, UnpackFontFile::<Impl, OFFSET>)
    }
}
pub trait IDWriteFactory6Impl: Sized + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontFaceReference();
    fn CreateFontResource();
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
    fn CreateFontCollectionFromFontSet();
    fn CreateFontSetBuilder();
    fn CreateTextFormat();
}
impl ::windows::core::RuntimeName for IDWriteFactory6 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory6";
}
impl IDWriteFactory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory6Impl, const OFFSET: isize>() -> IDWriteFactory6Vtbl {
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceReference(&*(&fontfile as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType), faceindex, fontsimulations, &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, faceindex: u32, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontResource(&*(&fontfile as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType), faceindex, ::core::mem::transmute_copy(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontSet(&*(&includedownloadablefonts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontCollection(&*(&includedownloadablefonts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), fontfamilymodel, ::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontCollectionFromFontSet(&*(&fontset as *const <IDWriteFontSet as ::windows::core::Abi>::Abi as *const <IDWriteFontSet as ::windows::core::DefaultType>::DefaultType), fontfamilymodel, ::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontSetBuilder(::core::mem::transmute_copy(&fontsetbuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextFormat<Impl: IDWriteFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, fontcollection: ::windows::core::RawPtr, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: super::super::Foundation::PWSTR, textformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextFormat(
                &*(&fontfamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType),
                fontaxisvaluecount,
                fontsize,
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&textformat),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFactory6>,
            ::windows::core::GetTrustLevel,
            CreateFontFaceReference::<Impl, OFFSET>,
            CreateFontResource::<Impl, OFFSET>,
            GetSystemFontSet::<Impl, OFFSET>,
            GetSystemFontCollection::<Impl, OFFSET>,
            CreateFontCollectionFromFontSet::<Impl, OFFSET>,
            CreateFontSetBuilder::<Impl, OFFSET>,
            CreateTextFormat::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFactory7Impl: Sized + IDWriteFactory6Impl + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
}
impl ::windows::core::RuntimeName for IDWriteFactory7 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFactory7";
}
impl IDWriteFactory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFactory7Impl, const OFFSET: isize>() -> IDWriteFactory7Vtbl {
        unsafe extern "system" fn GetSystemFontSet<Impl: IDWriteFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontSet(&*(&includedownloadablefonts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Impl: IDWriteFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemFontCollection(&*(&includedownloadablefonts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), fontfamilymodel, ::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFactory7>, ::windows::core::GetTrustLevel, GetSystemFontSet::<Impl, OFFSET>, GetSystemFontCollection::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFont {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFont";
}
impl IDWriteFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontImpl, const OFFSET: isize>() -> IDWriteFontVtbl {
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamily(::core::mem::transmute_copy(&fontfamily)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWeight<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStretch<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyle<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSymbolFont<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSymbolFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFaceNames(::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::windows::core::RawPtr, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInformationalStrings(informationalstringid, ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetrics(::core::mem::transmute_copy(&fontmetrics)).into()
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCharacter(unicodevalue, ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFont>,
            ::windows::core::GetTrustLevel,
            GetFontFamily::<Impl, OFFSET>,
            GetWeight::<Impl, OFFSET>,
            GetStretch::<Impl, OFFSET>,
            GetStyle::<Impl, OFFSET>,
            IsSymbolFont::<Impl, OFFSET>,
            GetFaceNames::<Impl, OFFSET>,
            GetInformationalStrings::<Impl, OFFSET>,
            GetSimulations::<Impl, OFFSET>,
            GetMetrics::<Impl, OFFSET>,
            HasCharacter::<Impl, OFFSET>,
            CreateFontFace::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFont1Impl: Sized + IDWriteFontImpl {
    fn GetMetrics();
    fn GetPanose();
    fn GetUnicodeRanges();
    fn IsMonospacedFont();
}
impl ::windows::core::RuntimeName for IDWriteFont1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFont1";
}
impl IDWriteFont1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont1Impl, const OFFSET: isize>() -> IDWriteFont1Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetrics(::core::mem::transmute_copy(&fontmetrics)).into()
        }
        unsafe extern "system" fn GetPanose<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPanose(::core::mem::transmute_copy(&panose)).into()
        }
        unsafe extern "system" fn GetUnicodeRanges<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeRanges(maxrangecount, ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMonospacedFont<Impl: IDWriteFont1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMonospacedFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFont1>, ::windows::core::GetTrustLevel, GetMetrics::<Impl, OFFSET>, GetPanose::<Impl, OFFSET>, GetUnicodeRanges::<Impl, OFFSET>, IsMonospacedFont::<Impl, OFFSET>)
    }
}
pub trait IDWriteFont2Impl: Sized + IDWriteFont1Impl + IDWriteFontImpl {
    fn IsColorFont();
}
impl ::windows::core::RuntimeName for IDWriteFont2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFont2";
}
impl IDWriteFont2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont2Impl, const OFFSET: isize>() -> IDWriteFont2Vtbl {
        unsafe extern "system" fn IsColorFont<Impl: IDWriteFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFont2>, ::windows::core::GetTrustLevel, IsColorFont::<Impl, OFFSET>)
    }
}
pub trait IDWriteFont3Impl: Sized + IDWriteFont2Impl + IDWriteFont1Impl + IDWriteFontImpl {
    fn CreateFontFace();
    fn Equals();
    fn GetFontFaceReference();
    fn HasCharacter();
    fn GetLocality();
}
impl ::windows::core::RuntimeName for IDWriteFont3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFont3";
}
impl IDWriteFont3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFont3Impl, const OFFSET: isize>() -> IDWriteFont3Vtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&font as *const <IDWriteFont as ::windows::core::Abi>::Abi as *const <IDWriteFont as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCharacter(unicodevalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteFont3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFont3>, ::windows::core::GetTrustLevel, CreateFontFace::<Impl, OFFSET>, Equals::<Impl, OFFSET>, GetFontFaceReference::<Impl, OFFSET>, HasCharacter::<Impl, OFFSET>, GetLocality::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontCollectionImpl: Sized {
    fn GetFontFamilyCount();
    fn GetFontFamily();
    fn FindFamilyName();
    fn GetFontFromFontFace();
}
impl ::windows::core::RuntimeName for IDWriteFontCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollection";
}
impl IDWriteFontCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollectionImpl, const OFFSET: isize>() -> IDWriteFontCollectionVtbl {
        unsafe extern "system" fn GetFontFamilyCount<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamily(index, ::core::mem::transmute_copy(&fontfamily)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFamilyName<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFamilyName(&*(&familyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFromFontFace<Impl: IDWriteFontCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFromFontFace(&*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontCollection>, ::windows::core::GetTrustLevel, GetFontFamilyCount::<Impl, OFFSET>, GetFontFamily::<Impl, OFFSET>, FindFamilyName::<Impl, OFFSET>, GetFontFromFontFace::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontCollection1Impl: Sized + IDWriteFontCollectionImpl {
    fn GetFontSet();
    fn GetFontFamily();
}
impl ::windows::core::RuntimeName for IDWriteFontCollection1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollection1";
}
impl IDWriteFontCollection1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection1Impl, const OFFSET: isize>() -> IDWriteFontCollection1Vtbl {
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollection1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamily(index, ::core::mem::transmute_copy(&fontfamily)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontCollection1>, ::windows::core::GetTrustLevel, GetFontSet::<Impl, OFFSET>, GetFontFamily::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontCollection2Impl: Sized + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetFontFamily();
    fn GetMatchingFonts();
    fn GetFontFamilyModel();
    fn GetFontSet();
}
impl ::windows::core::RuntimeName for IDWriteFontCollection2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollection2";
}
impl IDWriteFontCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection2Impl, const OFFSET: isize>() -> IDWriteFontCollection2Vtbl {
        unsafe extern "system" fn GetFontFamily<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamily(index, ::core::mem::transmute_copy(&fontfamily)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(&*(&familyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&fontlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyModel<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontCollection2>, ::windows::core::GetTrustLevel, GetFontFamily::<Impl, OFFSET>, GetMatchingFonts::<Impl, OFFSET>, GetFontFamilyModel::<Impl, OFFSET>, GetFontSet::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontCollection3Impl: Sized + IDWriteFontCollection2Impl + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetExpirationEvent();
}
impl ::windows::core::RuntimeName for IDWriteFontCollection3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollection3";
}
impl IDWriteFontCollection3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollection3Impl, const OFFSET: isize>() -> IDWriteFontCollection3Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Impl: IDWriteFontCollection3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExpirationEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontCollection3>, ::windows::core::GetTrustLevel, GetExpirationEvent::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontCollectionLoaderImpl: Sized {
    fn CreateEnumeratorFromKey();
}
impl ::windows::core::RuntimeName for IDWriteFontCollectionLoader {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollectionLoader";
}
impl IDWriteFontCollectionLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontCollectionLoaderImpl, const OFFSET: isize>() -> IDWriteFontCollectionLoaderVtbl {
        unsafe extern "system" fn CreateEnumeratorFromKey<Impl: IDWriteFontCollectionLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnumeratorFromKey(&*(&factory as *const <IDWriteFactory as ::windows::core::Abi>::Abi as *const <IDWriteFactory as ::windows::core::DefaultType>::DefaultType), &*(&collectionkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), collectionkeysize, ::core::mem::transmute_copy(&fontfileenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontCollectionLoader>, ::windows::core::GetTrustLevel, CreateEnumeratorFromKey::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontDownloadListenerImpl: Sized {
    fn DownloadCompleted();
}
impl ::windows::core::RuntimeName for IDWriteFontDownloadListener {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontDownloadListener";
}
impl IDWriteFontDownloadListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontDownloadListenerImpl, const OFFSET: isize>() -> IDWriteFontDownloadListenerVtbl {
        unsafe extern "system" fn DownloadCompleted<Impl: IDWriteFontDownloadListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadqueue: ::windows::core::RawPtr, context: *mut ::core::ffi::c_void, downloadresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadCompleted(&*(&downloadqueue as *const <IDWriteFontDownloadQueue as ::windows::core::Abi>::Abi as *const <IDWriteFontDownloadQueue as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), downloadresult).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontDownloadListener>, ::windows::core::GetTrustLevel, DownloadCompleted::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontDownloadQueueImpl: Sized {
    fn AddListener();
    fn RemoveListener();
    fn IsEmpty();
    fn BeginDownload();
    fn CancelDownload();
    fn GetGenerationCount();
}
impl ::windows::core::RuntimeName for IDWriteFontDownloadQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontDownloadQueue";
}
impl IDWriteFontDownloadQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>() -> IDWriteFontDownloadQueueVtbl {
        unsafe extern "system" fn AddListener<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr, token: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddListener(&*(&listener as *const <IDWriteFontDownloadListener as ::windows::core::Abi>::Abi as *const <IDWriteFontDownloadListener as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListener<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveListener(token) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDownload<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDownload(&*(&context as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelDownload<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationCount<Impl: IDWriteFontDownloadQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontDownloadQueue>, ::windows::core::GetTrustLevel, AddListener::<Impl, OFFSET>, RemoveListener::<Impl, OFFSET>, IsEmpty::<Impl, OFFSET>, BeginDownload::<Impl, OFFSET>, CancelDownload::<Impl, OFFSET>, GetGenerationCount::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontFace {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace";
}
impl IDWriteFontFaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceImpl, const OFFSET: isize>() -> IDWriteFontFaceVtbl {
        unsafe extern "system" fn GetType<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFiles(numberoffiles, ::core::mem::transmute_copy(&fontfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSymbolFont<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSymbolFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetrics(::core::mem::transmute_copy(&fontfacemetrics)).into()
        }
        unsafe extern "system" fn GetGlyphCount<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u16 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesignGlyphMetrics(glyphindices, glyphcount, ::core::mem::transmute_copy(&glyphmetrics), &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndices(codepoints, codepointcount, ::core::mem::transmute_copy(&glyphindices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetFontTable<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetFontTable(opentypetabletag, ::core::mem::transmute_copy(&tabledata), ::core::mem::transmute_copy(&tablesize), ::core::mem::transmute_copy(&tablecontext), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFontTable<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablecontext: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFontTable(&*(&tablecontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetGlyphRunOutline<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphRunOutline(
                emsize,
                glyphindices,
                glyphadvances,
                &*(&glyphoffsets as *const <DWRITE_GLYPH_OFFSET as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_OFFSET as ::windows::core::DefaultType>::DefaultType),
                glyphcount,
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&geometrysink as *const <super::Direct2D::Common::ID2D1SimplifiedGeometrySink as ::windows::core::Abi>::Abi as *const <super::Direct2D::Common::ID2D1SimplifiedGeometrySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, &*(&renderingparams as *const <IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGdiCompatibleMetrics(emsize, pixelsperdip, &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontfacemetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Impl: IDWriteFontFaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGdiCompatibleGlyphMetrics(
                emsize,
                pixelsperdip,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&usegdinatural as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                glyphindices,
                glyphcount,
                ::core::mem::transmute_copy(&glyphmetrics),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontFace>,
            ::windows::core::GetTrustLevel,
            GetType::<Impl, OFFSET>,
            GetFiles::<Impl, OFFSET>,
            GetIndex::<Impl, OFFSET>,
            GetSimulations::<Impl, OFFSET>,
            IsSymbolFont::<Impl, OFFSET>,
            GetMetrics::<Impl, OFFSET>,
            GetGlyphCount::<Impl, OFFSET>,
            GetDesignGlyphMetrics::<Impl, OFFSET>,
            GetGlyphIndices::<Impl, OFFSET>,
            TryGetFontTable::<Impl, OFFSET>,
            ReleaseFontTable::<Impl, OFFSET>,
            GetGlyphRunOutline::<Impl, OFFSET>,
            GetRecommendedRenderingMode::<Impl, OFFSET>,
            GetGdiCompatibleMetrics::<Impl, OFFSET>,
            GetGdiCompatibleGlyphMetrics::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontFace1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace1";
}
impl IDWriteFontFace1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace1Impl, const OFFSET: isize>() -> IDWriteFontFace1Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetrics(::core::mem::transmute_copy(&fontmetrics)).into()
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGdiCompatibleMetrics(emsize, pixelsperdip, &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontmetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretMetrics<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaretMetrics(::core::mem::transmute_copy(&caretmetrics)).into()
        }
        unsafe extern "system" fn GetUnicodeRanges<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeRanges(maxrangecount, ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMonospacedFont<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMonospacedFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesignGlyphAdvances(glyphcount, glyphindices, ::core::mem::transmute_copy(&glyphadvances), &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGdiCompatibleGlyphAdvances(
                emsize,
                pixelsperdip,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&usegdinatural as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                glyphcount,
                glyphindices,
                ::core::mem::transmute_copy(&glyphadvances),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKerningPairAdjustments(glyphcount, glyphindices, ::core::mem::transmute_copy(&glyphadvanceadjustments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKerningPairs<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKerningPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecommendedRenderingMode(fontemsize, dpix, dpiy, &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType), &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), outlinethreshold, measuringmode, ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerticalGlyphVariants(glyphcount, nominalglyphindices, ::core::mem::transmute_copy(&verticalglyphindices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Impl: IDWriteFontFace1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasVerticalGlyphVariants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontFace1>,
            ::windows::core::GetTrustLevel,
            GetMetrics::<Impl, OFFSET>,
            GetGdiCompatibleMetrics::<Impl, OFFSET>,
            GetCaretMetrics::<Impl, OFFSET>,
            GetUnicodeRanges::<Impl, OFFSET>,
            IsMonospacedFont::<Impl, OFFSET>,
            GetDesignGlyphAdvances::<Impl, OFFSET>,
            GetGdiCompatibleGlyphAdvances::<Impl, OFFSET>,
            GetKerningPairAdjustments::<Impl, OFFSET>,
            HasKerningPairs::<Impl, OFFSET>,
            GetRecommendedRenderingMode::<Impl, OFFSET>,
            GetVerticalGlyphVariants::<Impl, OFFSET>,
            HasVerticalGlyphVariants::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFontFace2Impl: Sized + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn IsColorFont();
    fn GetColorPaletteCount();
    fn GetPaletteEntryCount();
    fn GetPaletteEntries();
    fn GetRecommendedRenderingMode();
}
impl ::windows::core::RuntimeName for IDWriteFontFace2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace2";
}
impl IDWriteFontFace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace2Impl, const OFFSET: isize>() -> IDWriteFontFace2Vtbl {
        unsafe extern "system" fn IsColorFont<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorPaletteCount<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorPaletteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPaletteEntryCount<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPaletteEntryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPaletteEntries(colorpaletteindex, firstentryindex, entrycount, ::core::mem::transmute_copy(&paletteentries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecommendedRenderingMode(
                fontemsize,
                dpix,
                dpiy,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                outlinethreshold,
                measuringmode,
                &*(&renderingparams as *const <IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&renderingmode),
                ::core::mem::transmute_copy(&gridfitmode),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFace2>, ::windows::core::GetTrustLevel, IsColorFont::<Impl, OFFSET>, GetColorPaletteCount::<Impl, OFFSET>, GetPaletteEntryCount::<Impl, OFFSET>, GetPaletteEntries::<Impl, OFFSET>, GetRecommendedRenderingMode::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontFace3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace3";
}
impl IDWriteFontFace3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace3Impl, const OFFSET: isize>() -> IDWriteFontFace3Vtbl {
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPanose<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPanose(::core::mem::transmute_copy(&panose)).into()
        }
        unsafe extern "system" fn GetWeight<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStretch<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyle<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyNames(::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFaceNames(::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::windows::core::RawPtr, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInformationalStrings(informationalstringid, ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCharacter<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCharacter(unicodevalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::windows::core::RawPtr, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecommendedRenderingMode(
                fontemsize,
                dpix,
                dpiy,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                outlinethreshold,
                measuringmode,
                &*(&renderingparams as *const <IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&renderingmode),
                ::core::mem::transmute_copy(&gridfitmode),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCharacterLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCharacterLocal(unicodevalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGlyphLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGlyphLocal(glyphid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreCharactersLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: super::super::Foundation::PWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreCharactersLocal(&*(&characters as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), charactercount, &*(&enqueueifnotlocal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&islocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreGlyphsLocal<Impl: IDWriteFontFace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreGlyphsLocal(glyphindices, glyphcount, &*(&enqueueifnotlocal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&islocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontFace3>,
            ::windows::core::GetTrustLevel,
            GetFontFaceReference::<Impl, OFFSET>,
            GetPanose::<Impl, OFFSET>,
            GetWeight::<Impl, OFFSET>,
            GetStretch::<Impl, OFFSET>,
            GetStyle::<Impl, OFFSET>,
            GetFamilyNames::<Impl, OFFSET>,
            GetFaceNames::<Impl, OFFSET>,
            GetInformationalStrings::<Impl, OFFSET>,
            HasCharacter::<Impl, OFFSET>,
            GetRecommendedRenderingMode::<Impl, OFFSET>,
            IsCharacterLocal::<Impl, OFFSET>,
            IsGlyphLocal::<Impl, OFFSET>,
            AreCharactersLocal::<Impl, OFFSET>,
            AreGlyphsLocal::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFontFace4Impl: Sized + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetGlyphImageFormats();
    fn GetGlyphImageFormats();
    fn GetGlyphImageData();
    fn ReleaseGlyphImageData();
}
impl ::windows::core::RuntimeName for IDWriteFontFace4 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace4";
}
impl IDWriteFontFace4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace4Impl, const OFFSET: isize>() -> IDWriteFontFace4Vtbl {
        unsafe extern "system" fn GetGlyphImageFormats<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphImageFormats(glyphid, pixelsperemfirst, pixelsperemlast, ::core::mem::transmute_copy(&glyphimageformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphImageFormats<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphImageFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphImageData<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphImageData(glyphid, pixelsperem, glyphimageformat, ::core::mem::transmute_copy(&glyphdata), ::core::mem::transmute_copy(&glyphdatacontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Impl: IDWriteFontFace4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphdatacontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseGlyphImageData(&*(&glyphdatacontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFace4>, ::windows::core::GetTrustLevel, GetGlyphImageFormats::<Impl, OFFSET>, GetGlyphImageFormats::<Impl, OFFSET>, GetGlyphImageData::<Impl, OFFSET>, ReleaseGlyphImageData::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFace5Impl: Sized + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn HasVariations();
    fn GetFontResource();
    fn Equals();
}
impl ::windows::core::RuntimeName for IDWriteFontFace5 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace5";
}
impl IDWriteFontFace5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace5Impl, const OFFSET: isize>() -> IDWriteFontFace5Vtbl {
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValueCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), fontaxisvaluecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasVariations<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasVariations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontResource<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontResource(::core::mem::transmute_copy(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFontFace5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFace5>, ::windows::core::GetTrustLevel, GetFontAxisValueCount::<Impl, OFFSET>, GetFontAxisValues::<Impl, OFFSET>, HasVariations::<Impl, OFFSET>, GetFontResource::<Impl, OFFSET>, Equals::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFace6Impl: Sized + IDWriteFontFace5Impl + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFamilyNames();
    fn GetFaceNames();
}
impl ::windows::core::RuntimeName for IDWriteFontFace6 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFace6";
}
impl IDWriteFontFace6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFace6Impl, const OFFSET: isize>() -> IDWriteFontFace6Vtbl {
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFace6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyNames(fontfamilymodel, ::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Impl: IDWriteFontFace6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFaceNames(fontfamilymodel, ::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFace6>, ::windows::core::GetTrustLevel, GetFamilyNames::<Impl, OFFSET>, GetFaceNames::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontFaceReference {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFaceReference";
}
impl IDWriteFontFaceReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>() -> IDWriteFontFaceReferenceVtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceWithSimulations(fontfacesimulationflags, ::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&fontfacereference as *const <IDWriteFontFaceReference as ::windows::core::Abi>::Abi as *const <IDWriteFontFaceReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimulations<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFile<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFile(::core::mem::transmute_copy(&fontfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileSize<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTime<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileTime(::core::mem::transmute_copy(&lastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueFontDownloadRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characters: super::super::Foundation::PWSTR, charactercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueCharacterDownloadRequest(&*(&characters as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), charactercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueGlyphDownloadRequest(glyphindices, glyphcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Impl: IDWriteFontFaceReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueFileFragmentDownloadRequest(fileoffset, fragmentsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontFaceReference>,
            ::windows::core::GetTrustLevel,
            CreateFontFace::<Impl, OFFSET>,
            CreateFontFaceWithSimulations::<Impl, OFFSET>,
            Equals::<Impl, OFFSET>,
            GetFontFaceIndex::<Impl, OFFSET>,
            GetSimulations::<Impl, OFFSET>,
            GetFontFile::<Impl, OFFSET>,
            GetLocalFileSize::<Impl, OFFSET>,
            GetFileSize::<Impl, OFFSET>,
            GetFileTime::<Impl, OFFSET>,
            GetLocality::<Impl, OFFSET>,
            EnqueueFontDownloadRequest::<Impl, OFFSET>,
            EnqueueCharacterDownloadRequest::<Impl, OFFSET>,
            EnqueueGlyphDownloadRequest::<Impl, OFFSET>,
            EnqueueFileFragmentDownloadRequest::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFontFaceReference1Impl: Sized + IDWriteFontFaceReferenceImpl {
    fn CreateFontFace();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
}
impl ::windows::core::RuntimeName for IDWriteFontFaceReference1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFaceReference1";
}
impl IDWriteFontFaceReference1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>() -> IDWriteFontFaceReference1Vtbl {
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValueCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteFontFaceReference1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), fontaxisvaluecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFaceReference1>, ::windows::core::GetTrustLevel, CreateFontFace::<Impl, OFFSET>, GetFontAxisValueCount::<Impl, OFFSET>, GetFontAxisValues::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFallbackImpl: Sized {
    fn MapCharacters();
}
impl ::windows::core::RuntimeName for IDWriteFontFallback {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFallback";
}
impl IDWriteFontFallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallbackImpl, const OFFSET: isize>() -> IDWriteFontFallbackVtbl {
        unsafe extern "system" fn MapCharacters<Impl: IDWriteFontFallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, basefontcollection: ::windows::core::RawPtr, basefamilyname: super::super::Foundation::PWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::windows::core::RawPtr, scale: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapCharacters(
                &*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType),
                textposition,
                textlength,
                &*(&basefontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&basefamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                baseweight,
                basestyle,
                basestretch,
                mappedlength,
                ::core::mem::transmute_copy(&mappedfont),
                ::core::mem::transmute_copy(&scale),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFallback>, ::windows::core::GetTrustLevel, MapCharacters::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFallback1Impl: Sized + IDWriteFontFallbackImpl {
    fn MapCharacters();
}
impl ::windows::core::RuntimeName for IDWriteFontFallback1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFallback1";
}
impl IDWriteFontFallback1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallback1Impl, const OFFSET: isize>() -> IDWriteFontFallback1Vtbl {
        unsafe extern "system" fn MapCharacters<Impl: IDWriteFontFallback1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, basefontcollection: ::windows::core::RawPtr, basefamilyname: super::super::Foundation::PWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapCharacters(
                &*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType),
                textposition,
                textlength,
                &*(&basefontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&basefamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType),
                fontaxisvaluecount,
                mappedlength,
                ::core::mem::transmute_copy(&scale),
                ::core::mem::transmute_copy(&mappedfontface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFallback1>, ::windows::core::GetTrustLevel, MapCharacters::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFallbackBuilderImpl: Sized {
    fn AddMapping();
    fn AddMappings();
    fn CreateFontFallback();
}
impl ::windows::core::RuntimeName for IDWriteFontFallbackBuilder {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFallbackBuilder";
}
impl IDWriteFontFallbackBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>() -> IDWriteFontFallbackBuilderVtbl {
        unsafe extern "system" fn AddMapping<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: ::windows::core::RawPtr, localename: super::super::Foundation::PWSTR, basefamilyname: super::super::Foundation::PWSTR, scale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMapping(
                &*(&ranges as *const <DWRITE_UNICODE_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_UNICODE_RANGE as ::windows::core::DefaultType>::DefaultType),
                rangescount,
                targetfamilynames,
                targetfamilynamescount,
                &*(&fontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&basefamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                scale,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMappings<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMappings(&*(&fontfallback as *const <IDWriteFontFallback as ::windows::core::Abi>::Abi as *const <IDWriteFontFallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFallback<Impl: IDWriteFontFallbackBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFallback(::core::mem::transmute_copy(&fontfallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFallbackBuilder>, ::windows::core::GetTrustLevel, AddMapping::<Impl, OFFSET>, AddMappings::<Impl, OFFSET>, CreateFontFallback::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFamilyImpl: Sized + IDWriteFontListImpl {
    fn GetFamilyNames();
    fn GetFirstMatchingFont();
    fn GetMatchingFonts();
}
impl ::windows::core::RuntimeName for IDWriteFontFamily {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFamily";
}
impl IDWriteFontFamilyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamilyImpl, const OFFSET: isize>() -> IDWriteFontFamilyVtbl {
        unsafe extern "system" fn GetFamilyNames<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyNames(::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstMatchingFont<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstMatchingFont(weight, stretch, style, ::core::mem::transmute_copy(&matchingfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontFamilyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(weight, stretch, style, ::core::mem::transmute_copy(&matchingfonts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFamily>, ::windows::core::GetTrustLevel, GetFamilyNames::<Impl, OFFSET>, GetFirstMatchingFont::<Impl, OFFSET>, GetMatchingFonts::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFamily1Impl: Sized + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
impl ::windows::core::RuntimeName for IDWriteFontFamily1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFamily1";
}
impl IDWriteFontFamily1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamily1Impl, const OFFSET: isize>() -> IDWriteFontFamily1Vtbl {
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontLocality(listindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont(listindex, ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontFamily1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(listindex, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFamily1>, ::windows::core::GetTrustLevel, GetFontLocality::<Impl, OFFSET>, GetFont::<Impl, OFFSET>, GetFontFaceReference::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFamily2Impl: Sized + IDWriteFontFamily1Impl + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetMatchingFonts();
    fn GetFontSet();
}
impl ::windows::core::RuntimeName for IDWriteFontFamily2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFamily2";
}
impl IDWriteFontFamily2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFamily2Impl, const OFFSET: isize>() -> IDWriteFontFamily2Vtbl {
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontFamily2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(&*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&matchingfonts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontFamily2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFamily2>, ::windows::core::GetTrustLevel, GetMatchingFonts::<Impl, OFFSET>, GetFontSet::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFileImpl: Sized {
    fn GetReferenceKey();
    fn GetLoader();
    fn Analyze();
}
impl ::windows::core::RuntimeName for IDWriteFontFile {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFile";
}
impl IDWriteFontFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileImpl, const OFFSET: isize>() -> IDWriteFontFileVtbl {
        unsafe extern "system" fn GetReferenceKey<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReferenceKey(::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLoader<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLoader(::core::mem::transmute_copy(&fontfileloader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Analyze<Impl: IDWriteFontFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Analyze(::core::mem::transmute_copy(&issupportedfonttype), ::core::mem::transmute_copy(&fontfiletype), ::core::mem::transmute_copy(&fontfacetype), ::core::mem::transmute_copy(&numberoffaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFile>, ::windows::core::GetTrustLevel, GetReferenceKey::<Impl, OFFSET>, GetLoader::<Impl, OFFSET>, Analyze::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFileEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentFontFile();
}
impl ::windows::core::RuntimeName for IDWriteFontFileEnumerator {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFileEnumerator";
}
impl IDWriteFontFileEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileEnumeratorImpl, const OFFSET: isize>() -> IDWriteFontFileEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IDWriteFontFileEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hascurrentfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFontFile<Impl: IDWriteFontFileEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentFontFile(::core::mem::transmute_copy(&fontfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFileEnumerator>, ::windows::core::GetTrustLevel, MoveNext::<Impl, OFFSET>, GetCurrentFontFile::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFileLoaderImpl: Sized {
    fn CreateStreamFromKey();
}
impl ::windows::core::RuntimeName for IDWriteFontFileLoader {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFileLoader";
}
impl IDWriteFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileLoaderImpl, const OFFSET: isize>() -> IDWriteFontFileLoaderVtbl {
        unsafe extern "system" fn CreateStreamFromKey<Impl: IDWriteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStreamFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&fontfilestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFileLoader>, ::windows::core::GetTrustLevel, CreateStreamFromKey::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontFileStreamImpl: Sized {
    fn ReadFileFragment();
    fn ReleaseFileFragment();
    fn GetFileSize();
    fn GetLastWriteTime();
}
impl ::windows::core::RuntimeName for IDWriteFontFileStream {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontFileStream";
}
impl IDWriteFontFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>() -> IDWriteFontFileStreamVtbl {
        unsafe extern "system" fn ReadFileFragment<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadFileFragment(::core::mem::transmute_copy(&fragmentstart), fileoffset, fragmentsize, ::core::mem::transmute_copy(&fragmentcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFileFragment<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fragmentcontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFileFragment(&*(&fragmentcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetFileSize<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastWriteTime<Impl: IDWriteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastWriteTime(::core::mem::transmute_copy(&lastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontFileStream>, ::windows::core::GetTrustLevel, ReadFileFragment::<Impl, OFFSET>, ReleaseFileFragment::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>, GetLastWriteTime::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontListImpl: Sized {
    fn GetFontCollection();
    fn GetFontCount();
    fn GetFont();
}
impl ::windows::core::RuntimeName for IDWriteFontList {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontList";
}
impl IDWriteFontListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontListImpl, const OFFSET: isize>() -> IDWriteFontListVtbl {
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCollection(::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontCount<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont(index, ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontList>, ::windows::core::GetTrustLevel, GetFontCollection::<Impl, OFFSET>, GetFontCount::<Impl, OFFSET>, GetFont::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontList1Impl: Sized + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
impl ::windows::core::RuntimeName for IDWriteFontList1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontList1";
}
impl IDWriteFontList1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontList1Impl, const OFFSET: isize>() -> IDWriteFontList1Vtbl {
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontLocality(listindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont(listindex, ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontList1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(listindex, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontList1>, ::windows::core::GetTrustLevel, GetFontLocality::<Impl, OFFSET>, GetFont::<Impl, OFFSET>, GetFontFaceReference::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontList2Impl: Sized + IDWriteFontList1Impl + IDWriteFontListImpl {
    fn GetFontSet();
}
impl ::windows::core::RuntimeName for IDWriteFontList2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontList2";
}
impl IDWriteFontList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontList2Impl, const OFFSET: isize>() -> IDWriteFontList2Vtbl {
        unsafe extern "system" fn GetFontSet<Impl: IDWriteFontList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontList2>, ::windows::core::GetTrustLevel, GetFontSet::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontResource {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontResource";
}
impl IDWriteFontResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontResourceImpl, const OFFSET: isize>() -> IDWriteFontResourceVtbl {
        unsafe extern "system" fn GetFontFile<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFile(::core::mem::transmute_copy(&fontfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisCount<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), fontaxisvaluecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisRanges(::core::mem::transmute_copy(&fontaxisranges), fontaxisrangecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisAttributes<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisAttributes(axisindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisNames<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAxisNames(axisindex, ::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisValueNameCount<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAxisValueNameCount(axisindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisValueNames<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAxisValueNames(axisindex, axisvalueindex, ::core::mem::transmute_copy(&fontaxisrange), ::core::mem::transmute_copy(&names)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasVariations<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasVariations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(fontsimulations, &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Impl: IDWriteFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceReference(fontsimulations, &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontResource>,
            ::windows::core::GetTrustLevel,
            GetFontFile::<Impl, OFFSET>,
            GetFontFaceIndex::<Impl, OFFSET>,
            GetFontAxisCount::<Impl, OFFSET>,
            GetDefaultFontAxisValues::<Impl, OFFSET>,
            GetFontAxisRanges::<Impl, OFFSET>,
            GetFontAxisAttributes::<Impl, OFFSET>,
            GetAxisNames::<Impl, OFFSET>,
            GetAxisValueNameCount::<Impl, OFFSET>,
            GetAxisValueNames::<Impl, OFFSET>,
            HasVariations::<Impl, OFFSET>,
            CreateFontFace::<Impl, OFFSET>,
            CreateFontFaceReference::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontSet {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSet";
}
impl IDWriteFontSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetImpl, const OFFSET: isize>() -> IDWriteFontSetVtbl {
        unsafe extern "system" fn GetFontCount<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(listindex, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFontFaceReference<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFontFaceReference(&*(&fontfacereference as *const <IDWriteFontFaceReference as ::windows::core::Abi>::Abi as *const <IDWriteFontFaceReference as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFontFace<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFontFace(&*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValues(propertyid, ::core::mem::transmute_copy(&values)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: super::super::Foundation::PWSTR, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValues(propertyid, &*(&preferredlocalenames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&values)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValues(listindex, propertyid, ::core::mem::transmute_copy(&exists), ::core::mem::transmute_copy(&values)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyOccurrenceCount(&*(&property as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyoccurrencecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, familyname: super::super::Foundation::PWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(&*(&familyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fontweight, fontstretch, fontstyle, ::core::mem::transmute_copy(&filteredset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(&*(&properties as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), propertycount, ::core::mem::transmute_copy(&filteredset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontSet>,
            ::windows::core::GetTrustLevel,
            GetFontCount::<Impl, OFFSET>,
            GetFontFaceReference::<Impl, OFFSET>,
            FindFontFaceReference::<Impl, OFFSET>,
            FindFontFace::<Impl, OFFSET>,
            GetPropertyValues::<Impl, OFFSET>,
            GetPropertyValues::<Impl, OFFSET>,
            GetPropertyValues::<Impl, OFFSET>,
            GetPropertyOccurrenceCount::<Impl, OFFSET>,
            GetMatchingFonts::<Impl, OFFSET>,
            GetMatchingFonts::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDWriteFontSet1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSet1";
}
impl IDWriteFontSet1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet1Impl, const OFFSET: isize>() -> IDWriteFontSet1Vtbl {
        unsafe extern "system" fn GetMatchingFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFonts(&*(&fontproperty as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, ::core::mem::transmute_copy(&matchingfonts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstFontResources<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstFontResources(::core::mem::transmute_copy(&filteredfontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredFonts(indices, indexcount, ::core::mem::transmute_copy(&filteredfontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredFonts(&*(&fontaxisranges as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::DefaultType>::DefaultType), fontaxisrangecount, &*(&selectanyrange as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&filteredfontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredFonts(&*(&properties as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), propertycount, &*(&selectanyproperty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&filteredfontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredFontIndices(&*(&fontaxisranges as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::DefaultType>::DefaultType), fontaxisrangecount, &*(&selectanyrange as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&indices), maxindexcount, ::core::mem::transmute_copy(&actualindexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredFontIndices(&*(&properties as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), propertycount, &*(&selectanyproperty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&indices), maxindexcount, ::core::mem::transmute_copy(&actualindexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisRanges(listindex, ::core::mem::transmute_copy(&fontaxisranges), maxfontaxisrangecount, ::core::mem::transmute_copy(&actualfontaxisrangecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisRanges<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisRanges(::core::mem::transmute_copy(&fontaxisranges), maxfontaxisrangecount, ::core::mem::transmute_copy(&actualfontaxisrangecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceReference(listindex, ::core::mem::transmute_copy(&fontfacereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontResource(listindex, ::core::mem::transmute_copy(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFace(listindex, ::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontLocality<Impl: IDWriteFontSet1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontLocality(listindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteFontSet1>,
            ::windows::core::GetTrustLevel,
            GetMatchingFonts::<Impl, OFFSET>,
            GetFirstFontResources::<Impl, OFFSET>,
            GetFilteredFonts::<Impl, OFFSET>,
            GetFilteredFonts::<Impl, OFFSET>,
            GetFilteredFonts::<Impl, OFFSET>,
            GetFilteredFontIndices::<Impl, OFFSET>,
            GetFilteredFontIndices::<Impl, OFFSET>,
            GetFontAxisRanges::<Impl, OFFSET>,
            GetFontAxisRanges::<Impl, OFFSET>,
            GetFontFaceReference::<Impl, OFFSET>,
            CreateFontResource::<Impl, OFFSET>,
            CreateFontFace::<Impl, OFFSET>,
            GetFontLocality::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteFontSet2Impl: Sized + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetExpirationEvent();
}
impl ::windows::core::RuntimeName for IDWriteFontSet2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSet2";
}
impl IDWriteFontSet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet2Impl, const OFFSET: isize>() -> IDWriteFontSet2Vtbl {
        unsafe extern "system" fn GetExpirationEvent<Impl: IDWriteFontSet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExpirationEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontSet2>, ::windows::core::GetTrustLevel, GetExpirationEvent::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontSet3Impl: Sized + IDWriteFontSet2Impl + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetFontSourceType();
    fn GetFontSourceNameLength();
    fn GetFontSourceName();
}
impl ::windows::core::RuntimeName for IDWriteFontSet3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSet3";
}
impl IDWriteFontSet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSet3Impl, const OFFSET: isize>() -> IDWriteFontSet3Vtbl {
        unsafe extern "system" fn GetFontSourceType<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSourceType(fontindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSourceNameLength<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSourceNameLength(listindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSourceName<Impl: IDWriteFontSet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: super::super::Foundation::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSourceName(listindex, ::core::mem::transmute_copy(&stringbuffer), stringbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontSet3>, ::windows::core::GetTrustLevel, GetFontSourceType::<Impl, OFFSET>, GetFontSourceNameLength::<Impl, OFFSET>, GetFontSourceName::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontSetBuilderImpl: Sized {
    fn AddFontFaceReference();
    fn AddFontFaceReference();
    fn AddFontSet();
    fn CreateFontSet();
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSetBuilder";
}
impl IDWriteFontSetBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>() -> IDWriteFontSetBuilderVtbl {
        unsafe extern "system" fn AddFontFaceReference<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontFaceReference(&*(&fontfacereference as *const <IDWriteFontFaceReference as ::windows::core::Abi>::Abi as *const <IDWriteFontFaceReference as ::windows::core::DefaultType>::DefaultType), &*(&properties as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType), propertycount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFontFaceReference<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfacereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontFaceReference(&*(&fontfacereference as *const <IDWriteFontFaceReference as ::windows::core::Abi>::Abi as *const <IDWriteFontFaceReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFontSet<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontSet(&*(&fontset as *const <IDWriteFontSet as ::windows::core::Abi>::Abi as *const <IDWriteFontSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSet<Impl: IDWriteFontSetBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontSet(::core::mem::transmute_copy(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontSetBuilder>, ::windows::core::GetTrustLevel, AddFontFaceReference::<Impl, OFFSET>, AddFontFaceReference::<Impl, OFFSET>, AddFontSet::<Impl, OFFSET>, CreateFontSet::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontSetBuilder1Impl: Sized + IDWriteFontSetBuilderImpl {
    fn AddFontFile();
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSetBuilder1";
}
impl IDWriteFontSetBuilder1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilder1Impl, const OFFSET: isize>() -> IDWriteFontSetBuilder1Vtbl {
        unsafe extern "system" fn AddFontFile<Impl: IDWriteFontSetBuilder1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontFile(&*(&fontfile as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontSetBuilder1>, ::windows::core::GetTrustLevel, AddFontFile::<Impl, OFFSET>)
    }
}
pub trait IDWriteFontSetBuilder2Impl: Sized + IDWriteFontSetBuilder1Impl + IDWriteFontSetBuilderImpl {
    fn AddFont();
    fn AddFontFile();
}
impl ::windows::core::RuntimeName for IDWriteFontSetBuilder2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteFontSetBuilder2";
}
impl IDWriteFontSetBuilder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteFontSetBuilder2Impl, const OFFSET: isize>() -> IDWriteFontSetBuilder2Vtbl {
        unsafe extern "system" fn AddFont<Impl: IDWriteFontSetBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfile: ::windows::core::RawPtr, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFont(
                &*(&fontfile as *const <IDWriteFontFile as ::windows::core::Abi>::Abi as *const <IDWriteFontFile as ::windows::core::DefaultType>::DefaultType),
                fontfaceindex,
                fontsimulations,
                &*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType),
                fontaxisvaluecount,
                &*(&fontaxisranges as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_RANGE as ::windows::core::DefaultType>::DefaultType),
                fontaxisrangecount,
                &*(&properties as *const <DWRITE_FONT_PROPERTY as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_PROPERTY as ::windows::core::DefaultType>::DefaultType),
                propertycount,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFontFile<Impl: IDWriteFontSetBuilder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontFile(&*(&filepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteFontSetBuilder2>, ::windows::core::GetTrustLevel, AddFont::<Impl, OFFSET>, AddFontFile::<Impl, OFFSET>)
    }
}
pub trait IDWriteGdiInteropImpl: Sized {
    fn CreateFontFromLOGFONT();
    fn ConvertFontToLOGFONT();
    fn ConvertFontFaceToLOGFONT();
    fn CreateFontFaceFromHdc();
    fn CreateBitmapRenderTarget();
}
impl ::windows::core::RuntimeName for IDWriteGdiInterop {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteGdiInterop";
}
impl IDWriteGdiInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGdiInteropImpl, const OFFSET: isize>() -> IDWriteGdiInteropVtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFromLOGFONT(&*(&logfont as *const <super::Gdi::LOGFONTW as ::windows::core::Abi>::Abi as *const <super::Gdi::LOGFONTW as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFontToLOGFONT(&*(&font as *const <IDWriteFont as ::windows::core::Abi>::Abi as *const <IDWriteFont as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&logfont), ::core::mem::transmute_copy(&issystemfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, logfont: *mut super::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFontFaceToLOGFONT(&*(&font as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&logfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFaceFromHdc(&*(&hdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Impl: IDWriteGdiInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBitmapRenderTarget(&*(&hdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), width, height, ::core::mem::transmute_copy(&rendertarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteGdiInterop>, ::windows::core::GetTrustLevel, CreateFontFromLOGFONT::<Impl, OFFSET>, ConvertFontToLOGFONT::<Impl, OFFSET>, ConvertFontFaceToLOGFONT::<Impl, OFFSET>, CreateFontFaceFromHdc::<Impl, OFFSET>, CreateBitmapRenderTarget::<Impl, OFFSET>)
    }
}
pub trait IDWriteGdiInterop1Impl: Sized + IDWriteGdiInteropImpl {
    fn CreateFontFromLOGFONT();
    fn GetFontSignature();
    fn GetFontSignature();
    fn GetMatchingFontsByLOGFONT();
}
impl ::windows::core::RuntimeName for IDWriteGdiInterop1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteGdiInterop1";
}
impl IDWriteGdiInterop1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>() -> IDWriteGdiInterop1Vtbl {
        unsafe extern "system" fn CreateFontFromLOGFONT<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: ::windows::core::RawPtr, font: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFromLOGFONT(&*(&logfont as *const <super::Gdi::LOGFONTW as ::windows::core::Abi>::Abi as *const <super::Gdi::LOGFONTW as ::windows::core::DefaultType>::DefaultType), &*(&fontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&font)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSignature<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSignature(&*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSignature<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, font: ::windows::core::RawPtr, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSignature(&*(&font as *const <IDWriteFont as ::windows::core::Abi>::Abi as *const <IDWriteFont as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fontsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Impl: IDWriteGdiInterop1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: ::windows::core::RawPtr, filteredset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingFontsByLOGFONT(&*(&logfont as *const <super::Gdi::LOGFONTA as ::windows::core::Abi>::Abi as *const <super::Gdi::LOGFONTA as ::windows::core::DefaultType>::DefaultType), &*(&fontset as *const <IDWriteFontSet as ::windows::core::Abi>::Abi as *const <IDWriteFontSet as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&filteredset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteGdiInterop1>, ::windows::core::GetTrustLevel, CreateFontFromLOGFONT::<Impl, OFFSET>, GetFontSignature::<Impl, OFFSET>, GetFontSignature::<Impl, OFFSET>, GetMatchingFontsByLOGFONT::<Impl, OFFSET>)
    }
}
pub trait IDWriteGlyphRunAnalysisImpl: Sized {
    fn GetAlphaTextureBounds();
    fn CreateAlphaTexture();
    fn GetAlphaBlendParams();
}
impl ::windows::core::RuntimeName for IDWriteGlyphRunAnalysis {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteGlyphRunAnalysis";
}
impl IDWriteGlyphRunAnalysisVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>() -> IDWriteGlyphRunAnalysisVtbl {
        unsafe extern "system" fn GetAlphaTextureBounds<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlphaTextureBounds(texturetype, ::core::mem::transmute_copy(&texturebounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAlphaTexture<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAlphaTexture(texturetype, &*(&texturebounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&alphavalues), buffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlphaBlendParams<Impl: IDWriteGlyphRunAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, renderingparams: ::windows::core::RawPtr, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlphaBlendParams(&*(&renderingparams as *const <IDWriteRenderingParams as ::windows::core::Abi>::Abi as *const <IDWriteRenderingParams as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&blendgamma), ::core::mem::transmute_copy(&blendenhancedcontrast), ::core::mem::transmute_copy(&blendcleartypelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteGlyphRunAnalysis>, ::windows::core::GetTrustLevel, GetAlphaTextureBounds::<Impl, OFFSET>, CreateAlphaTexture::<Impl, OFFSET>, GetAlphaBlendParams::<Impl, OFFSET>)
    }
}
pub trait IDWriteInMemoryFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateInMemoryFontFileReference();
    fn GetFileCount();
}
impl ::windows::core::RuntimeName for IDWriteInMemoryFontFileLoader {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteInMemoryFontFileLoader";
}
impl IDWriteInMemoryFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteInMemoryFontFileLoaderImpl, const OFFSET: isize>() -> IDWriteInMemoryFontFileLoaderVtbl {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Impl: IDWriteInMemoryFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInMemoryFontFileReference(
                &*(&factory as *const <IDWriteFactory as ::windows::core::Abi>::Abi as *const <IDWriteFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&fontdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                fontdatasize,
                &*(&ownerobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&fontfile),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Impl: IDWriteInMemoryFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteInMemoryFontFileLoader>, ::windows::core::GetTrustLevel, CreateInMemoryFontFileReference::<Impl, OFFSET>, GetFileCount::<Impl, OFFSET>)
    }
}
pub trait IDWriteInlineObjectImpl: Sized {
    fn Draw();
    fn GetMetrics();
    fn GetOverhangMetrics();
    fn GetBreakConditions();
}
impl ::windows::core::RuntimeName for IDWriteInlineObject {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteInlineObject";
}
impl IDWriteInlineObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteInlineObjectImpl, const OFFSET: isize>() -> IDWriteInlineObjectVtbl {
        unsafe extern "system" fn Draw<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::windows::core::RawPtr, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&renderer as *const <IDWriteTextRenderer as ::windows::core::Abi>::Abi as *const <IDWriteTextRenderer as ::windows::core::DefaultType>::DefaultType),
                originx,
                originy,
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetrics(::core::mem::transmute_copy(&metrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverhangMetrics(::core::mem::transmute_copy(&overhangs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakConditions<Impl: IDWriteInlineObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakConditions(::core::mem::transmute_copy(&breakconditionbefore), ::core::mem::transmute_copy(&breakconditionafter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteInlineObject>, ::windows::core::GetTrustLevel, Draw::<Impl, OFFSET>, GetMetrics::<Impl, OFFSET>, GetOverhangMetrics::<Impl, OFFSET>, GetBreakConditions::<Impl, OFFSET>)
    }
}
pub trait IDWriteLocalFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn GetFilePathLengthFromKey();
    fn GetFilePathFromKey();
    fn GetLastWriteTimeFromKey();
}
impl ::windows::core::RuntimeName for IDWriteLocalFontFileLoader {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteLocalFontFileLoader";
}
impl IDWriteLocalFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>() -> IDWriteLocalFontFileLoaderVtbl {
        unsafe extern "system" fn GetFilePathLengthFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilePathLengthFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&filepathlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilePathFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: super::super::Foundation::PWSTR, filepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilePathFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&filepath), filepathsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Impl: IDWriteLocalFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastWriteTimeFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&lastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteLocalFontFileLoader>, ::windows::core::GetTrustLevel, GetFilePathLengthFromKey::<Impl, OFFSET>, GetFilePathFromKey::<Impl, OFFSET>, GetLastWriteTimeFromKey::<Impl, OFFSET>)
    }
}
pub trait IDWriteLocalizedStringsImpl: Sized {
    fn GetCount();
    fn FindLocaleName();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
impl ::windows::core::RuntimeName for IDWriteLocalizedStrings {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteLocalizedStrings";
}
impl IDWriteLocalizedStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>() -> IDWriteLocalizedStringsVtbl {
        unsafe extern "system" fn GetCount<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLocaleName<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindLocaleName(&*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleNameLength(index, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, localename: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleName(index, ::core::mem::transmute_copy(&localename), size) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringLength<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringLength(index, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IDWriteLocalizedStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stringbuffer: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(index, ::core::mem::transmute_copy(&stringbuffer), size) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteLocalizedStrings>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, FindLocaleName::<Impl, OFFSET>, GetLocaleNameLength::<Impl, OFFSET>, GetLocaleName::<Impl, OFFSET>, GetStringLength::<Impl, OFFSET>, GetString::<Impl, OFFSET>)
    }
}
pub trait IDWriteNumberSubstitutionImpl: Sized {}
impl ::windows::core::RuntimeName for IDWriteNumberSubstitution {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteNumberSubstitution";
}
impl IDWriteNumberSubstitutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteNumberSubstitutionImpl, const OFFSET: isize>() -> IDWriteNumberSubstitutionVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteNumberSubstitution>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDWritePixelSnappingImpl: Sized {
    fn IsPixelSnappingDisabled();
    fn GetCurrentTransform();
    fn GetPixelsPerDip();
}
impl ::windows::core::RuntimeName for IDWritePixelSnapping {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWritePixelSnapping";
}
impl IDWritePixelSnappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWritePixelSnappingImpl, const OFFSET: isize>() -> IDWritePixelSnappingVtbl {
        unsafe extern "system" fn IsPixelSnappingDisabled<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPixelSnappingDisabled(&*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isdisabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentTransform(&*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelsPerDip<Impl: IDWritePixelSnappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelsPerDip(&*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pixelsperdip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWritePixelSnapping>, ::windows::core::GetTrustLevel, IsPixelSnappingDisabled::<Impl, OFFSET>, GetCurrentTransform::<Impl, OFFSET>, GetPixelsPerDip::<Impl, OFFSET>)
    }
}
pub trait IDWriteRemoteFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateRemoteStreamFromKey();
    fn GetLocalityFromKey();
    fn CreateFontFileReferenceFromUrl();
}
impl ::windows::core::RuntimeName for IDWriteRemoteFontFileLoader {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRemoteFontFileLoader";
}
impl IDWriteRemoteFontFileLoaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>() -> IDWriteRemoteFontFileLoaderVtbl {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteStreamFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&fontfilestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalityFromKey<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalityFromKey(&*(&fontfilereferencekey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), fontfilereferencekeysize, ::core::mem::transmute_copy(&locality)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Impl: IDWriteRemoteFontFileLoaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, baseurl: super::super::Foundation::PWSTR, fontfileurl: super::super::Foundation::PWSTR, fontfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontFileReferenceFromUrl(
                &*(&factory as *const <IDWriteFactory as ::windows::core::Abi>::Abi as *const <IDWriteFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&baseurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fontfileurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&fontfile),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRemoteFontFileLoader>, ::windows::core::GetTrustLevel, CreateRemoteStreamFromKey::<Impl, OFFSET>, GetLocalityFromKey::<Impl, OFFSET>, CreateFontFileReferenceFromUrl::<Impl, OFFSET>)
    }
}
pub trait IDWriteRemoteFontFileStreamImpl: Sized + IDWriteFontFileStreamImpl {
    fn GetLocalFileSize();
    fn GetFileFragmentLocality();
    fn GetLocality();
    fn BeginDownload();
}
impl ::windows::core::RuntimeName for IDWriteRemoteFontFileStream {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRemoteFontFileStream";
}
impl IDWriteRemoteFontFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>() -> IDWriteRemoteFontFileStreamVtbl {
        unsafe extern "system" fn GetLocalFileSize<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localfilesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalFileSize(::core::mem::transmute_copy(&localfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileFragmentLocality<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileFragmentLocality(fileoffset, fragmentsize, ::core::mem::transmute_copy(&islocal), partialsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocality<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDownload<Impl: IDWriteRemoteFontFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadoperationid: *const ::windows::core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDownload(&*(&downloadoperationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&filefragments as *const <DWRITE_FILE_FRAGMENT as ::windows::core::Abi>::Abi as *const <DWRITE_FILE_FRAGMENT as ::windows::core::DefaultType>::DefaultType), fragmentcount, ::core::mem::transmute_copy(&asyncresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRemoteFontFileStream>, ::windows::core::GetTrustLevel, GetLocalFileSize::<Impl, OFFSET>, GetFileFragmentLocality::<Impl, OFFSET>, GetLocality::<Impl, OFFSET>, BeginDownload::<Impl, OFFSET>)
    }
}
pub trait IDWriteRenderingParamsImpl: Sized {
    fn GetGamma();
    fn GetEnhancedContrast();
    fn GetClearTypeLevel();
    fn GetPixelGeometry();
    fn GetRenderingMode();
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRenderingParams";
}
impl IDWriteRenderingParamsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>() -> IDWriteRenderingParamsVtbl {
        unsafe extern "system" fn GetGamma<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnhancedContrast<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnhancedContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClearTypeLevel<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClearTypeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelGeometry<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPixelGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderingMode<Impl: IDWriteRenderingParamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRenderingParams>, ::windows::core::GetTrustLevel, GetGamma::<Impl, OFFSET>, GetEnhancedContrast::<Impl, OFFSET>, GetClearTypeLevel::<Impl, OFFSET>, GetPixelGeometry::<Impl, OFFSET>, GetRenderingMode::<Impl, OFFSET>)
    }
}
pub trait IDWriteRenderingParams1Impl: Sized + IDWriteRenderingParamsImpl {
    fn GetGrayscaleEnhancedContrast();
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRenderingParams1";
}
impl IDWriteRenderingParams1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams1Impl, const OFFSET: isize>() -> IDWriteRenderingParams1Vtbl {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Impl: IDWriteRenderingParams1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrayscaleEnhancedContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRenderingParams1>, ::windows::core::GetTrustLevel, GetGrayscaleEnhancedContrast::<Impl, OFFSET>)
    }
}
pub trait IDWriteRenderingParams2Impl: Sized + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetGridFitMode();
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRenderingParams2";
}
impl IDWriteRenderingParams2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams2Impl, const OFFSET: isize>() -> IDWriteRenderingParams2Vtbl {
        unsafe extern "system" fn GetGridFitMode<Impl: IDWriteRenderingParams2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_GRID_FIT_MODE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGridFitMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRenderingParams2>, ::windows::core::GetTrustLevel, GetGridFitMode::<Impl, OFFSET>)
    }
}
pub trait IDWriteRenderingParams3Impl: Sized + IDWriteRenderingParams2Impl + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetRenderingMode1();
}
impl ::windows::core::RuntimeName for IDWriteRenderingParams3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteRenderingParams3";
}
impl IDWriteRenderingParams3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteRenderingParams3Impl, const OFFSET: isize>() -> IDWriteRenderingParams3Vtbl {
        unsafe extern "system" fn GetRenderingMode1<Impl: IDWriteRenderingParams3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE1 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderingMode1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteRenderingParams3>, ::windows::core::GetTrustLevel, GetRenderingMode1::<Impl, OFFSET>)
    }
}
pub trait IDWriteStringListImpl: Sized {
    fn GetCount();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
impl ::windows::core::RuntimeName for IDWriteStringList {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteStringList";
}
impl IDWriteStringListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteStringListImpl, const OFFSET: isize>() -> IDWriteStringListVtbl {
        unsafe extern "system" fn GetCount<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleNameLength(listindex, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, localename: super::super::Foundation::PWSTR, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleName(listindex, ::core::mem::transmute_copy(&localename), size) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringLength<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringLength(listindex, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IDWriteStringListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: super::super::Foundation::PWSTR, stringbuffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(listindex, ::core::mem::transmute_copy(&stringbuffer), stringbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteStringList>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetLocaleNameLength::<Impl, OFFSET>, GetLocaleName::<Impl, OFFSET>, GetStringLength::<Impl, OFFSET>, GetString::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextAnalysisSinkImpl: Sized {
    fn SetScriptAnalysis();
    fn SetLineBreakpoints();
    fn SetBidiLevel();
    fn SetNumberSubstitution();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSink {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalysisSink";
}
impl IDWriteTextAnalysisSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSinkVtbl {
        unsafe extern "system" fn SetScriptAnalysis<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScriptAnalysis(textposition, textlength, &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineBreakpoints<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineBreakpoints(textposition, textlength, &*(&linebreakpoints as *const <DWRITE_LINE_BREAKPOINT as ::windows::core::Abi>::Abi as *const <DWRITE_LINE_BREAKPOINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBidiLevel(textposition, textlength, explicitlevel, resolvedlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberSubstitution<Impl: IDWriteTextAnalysisSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNumberSubstitution(textposition, textlength, &*(&numbersubstitution as *const <IDWriteNumberSubstitution as ::windows::core::Abi>::Abi as *const <IDWriteNumberSubstitution as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextAnalysisSink>, ::windows::core::GetTrustLevel, SetScriptAnalysis::<Impl, OFFSET>, SetLineBreakpoints::<Impl, OFFSET>, SetBidiLevel::<Impl, OFFSET>, SetNumberSubstitution::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextAnalysisSink1Impl: Sized + IDWriteTextAnalysisSinkImpl {
    fn SetGlyphOrientation();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSink1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalysisSink1";
}
impl IDWriteTextAnalysisSink1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSink1Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSink1Vtbl {
        unsafe extern "system" fn SetGlyphOrientation<Impl: IDWriteTextAnalysisSink1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGlyphOrientation(textposition, textlength, glyphorientationangle, adjustedbidilevel, &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextAnalysisSink1>, ::windows::core::GetTrustLevel, SetGlyphOrientation::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextAnalysisSourceImpl: Sized {
    fn GetTextAtPosition();
    fn GetTextBeforePosition();
    fn GetParagraphReadingDirection();
    fn GetLocaleName();
    fn GetNumberSubstitution();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSource {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalysisSource";
}
impl IDWriteTextAnalysisSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSourceVtbl {
        unsafe extern "system" fn GetTextAtPosition<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAtPosition(textposition, ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextBeforePosition<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextBeforePosition(textposition, ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParagraphReadingDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleName(textposition, ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&localename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberSubstitution<Impl: IDWriteTextAnalysisSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberSubstitution(textposition, ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&numbersubstitution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextAnalysisSource>, ::windows::core::GetTrustLevel, GetTextAtPosition::<Impl, OFFSET>, GetTextBeforePosition::<Impl, OFFSET>, GetParagraphReadingDirection::<Impl, OFFSET>, GetLocaleName::<Impl, OFFSET>, GetNumberSubstitution::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextAnalysisSource1Impl: Sized + IDWriteTextAnalysisSourceImpl {
    fn GetVerticalGlyphOrientation();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalysisSource1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalysisSource1";
}
impl IDWriteTextAnalysisSource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalysisSource1Impl, const OFFSET: isize>() -> IDWriteTextAnalysisSource1Vtbl {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextAnalysisSource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerticalGlyphOrientation(textposition, ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphorientation), ::core::mem::transmute_copy(&bidilevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextAnalysisSource1>, ::windows::core::GetTrustLevel, GetVerticalGlyphOrientation::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextAnalyzerImpl: Sized {
    fn AnalyzeScript();
    fn AnalyzeBidi();
    fn AnalyzeNumberSubstitution();
    fn AnalyzeLineBreakpoints();
    fn GetGlyphs();
    fn GetGlyphPlacements();
    fn GetGdiCompatibleGlyphPlacements();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalyzer";
}
impl IDWriteTextAnalyzerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>() -> IDWriteTextAnalyzerVtbl {
        unsafe extern "system" fn AnalyzeScript<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeScript(&*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType), textposition, textlength, &*(&analysissink as *const <IDWriteTextAnalysisSink as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeBidi<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeBidi(&*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType), textposition, textlength, &*(&analysissink as *const <IDWriteTextAnalysisSink as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeNumberSubstitution(&*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType), textposition, textlength, &*(&analysissink as *const <IDWriteTextAnalysisSink as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeLineBreakpoints(&*(&analysissource as *const <IDWriteTextAnalysisSource as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource as ::windows::core::DefaultType>::DefaultType), textposition, textlength, &*(&analysissink as *const <IDWriteTextAnalysisSink as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphs<Impl: IDWriteTextAnalyzerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textstring: super::super::Foundation::PWSTR, textlength: u32, fontface: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, numbersubstitution: ::windows::core::RawPtr, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphs(
                &*(&textstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                textlength,
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&numbersubstitution as *const <IDWriteNumberSubstitution as ::windows::core::Abi>::Abi as *const <IDWriteNumberSubstitution as ::windows::core::DefaultType>::DefaultType),
                &*(&features as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::Abi>::Abi as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::DefaultType>::DefaultType),
                featurerangelengths,
                featureranges,
                maxglyphcount,
                ::core::mem::transmute_copy(&clustermap),
                ::core::mem::transmute_copy(&textprops),
                ::core::mem::transmute_copy(&glyphindices),
                ::core::mem::transmute_copy(&glyphprops),
                ::core::mem::transmute_copy(&actualglyphcount),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
            match (*this).GetGlyphPlacements(
                &*(&textstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                clustermap,
                &*(&textprops as *const <DWRITE_SHAPING_TEXT_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_TEXT_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                textlength,
                glyphindices,
                &*(&glyphprops as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                glyphcount,
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&features as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::Abi>::Abi as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::DefaultType>::DefaultType),
                featurerangelengths,
                featureranges,
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&glyphoffsets),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
            match (*this).GetGdiCompatibleGlyphPlacements(
                &*(&textstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                clustermap,
                &*(&textprops as *const <DWRITE_SHAPING_TEXT_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_TEXT_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                textlength,
                glyphindices,
                &*(&glyphprops as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                glyphcount,
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                pixelsperdip,
                &*(&transform as *const <DWRITE_MATRIX as ::windows::core::Abi>::Abi as *const <DWRITE_MATRIX as ::windows::core::DefaultType>::DefaultType),
                &*(&usegdinatural as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&features as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::Abi>::Abi as *const <DWRITE_TYPOGRAPHIC_FEATURES as ::windows::core::DefaultType>::DefaultType),
                featurerangelengths,
                featureranges,
                ::core::mem::transmute_copy(&glyphadvances),
                ::core::mem::transmute_copy(&glyphoffsets),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextAnalyzer>,
            ::windows::core::GetTrustLevel,
            AnalyzeScript::<Impl, OFFSET>,
            AnalyzeBidi::<Impl, OFFSET>,
            AnalyzeNumberSubstitution::<Impl, OFFSET>,
            AnalyzeLineBreakpoints::<Impl, OFFSET>,
            GetGlyphs::<Impl, OFFSET>,
            GetGlyphPlacements::<Impl, OFFSET>,
            GetGdiCompatibleGlyphPlacements::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalyzer1";
}
impl IDWriteTextAnalyzer1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>() -> IDWriteTextAnalyzer1Vtbl {
        unsafe extern "system" fn ApplyCharacterSpacing<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyCharacterSpacing(
                leadingspacing,
                trailingspacing,
                minimumadvancewidth,
                textlength,
                glyphcount,
                clustermap,
                glyphadvances,
                &*(&glyphoffsets as *const <DWRITE_GLYPH_OFFSET as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_OFFSET as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphproperties as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&modifiedglyphadvances),
                ::core::mem::transmute_copy(&modifiedglyphoffsets),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBaseline<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseline(
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                baseline,
                &*(&isvertical as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&issimulationallowed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&baselinecoordinate),
                ::core::mem::transmute_copy(&exists),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, analysissource: ::windows::core::RawPtr, textposition: u32, textlength: u32, analysissink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalyzeVerticalGlyphOrientation(&*(&analysissource as *const <IDWriteTextAnalysisSource1 as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSource1 as ::windows::core::DefaultType>::DefaultType), textposition, textlength, &*(&analysissink as *const <IDWriteTextAnalysisSink1 as ::windows::core::Abi>::Abi as *const <IDWriteTextAnalysisSink1 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphOrientationTransform(glyphorientationangle, &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptProperties<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScriptProperties(&*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&scriptproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextComplexity<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textstring: super::super::Foundation::PWSTR, textlength: u32, fontface: ::windows::core::RawPtr, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextComplexity(&*(&textstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), textlength, &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&istextsimple), textlengthread, ::core::mem::transmute_copy(&glyphindices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJustificationOpportunities<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: super::super::Foundation::PWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJustificationOpportunities(
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                textlength,
                glyphcount,
                &*(&textstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                clustermap,
                &*(&glyphproperties as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&justificationopportunities),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JustifyGlyphAdvances(
                linewidth,
                glyphcount,
                &*(&justificationopportunities as *const <DWRITE_JUSTIFICATION_OPPORTUNITY as ::windows::core::Abi>::Abi as *const <DWRITE_JUSTIFICATION_OPPORTUNITY as ::windows::core::DefaultType>::DefaultType),
                glyphadvances,
                &*(&glyphoffsets as *const <DWRITE_GLYPH_OFFSET as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_OFFSET as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&justifiedglyphadvances),
                ::core::mem::transmute_copy(&justifiedglyphoffsets),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Impl: IDWriteTextAnalyzer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJustifiedGlyphs(
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                fontemsize,
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                textlength,
                glyphcount,
                maxglyphcount,
                clustermap,
                glyphindices,
                glyphadvances,
                justifiedglyphadvances,
                &*(&justifiedglyphoffsets as *const <DWRITE_GLYPH_OFFSET as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_OFFSET as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphproperties as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::Abi>::Abi as *const <DWRITE_SHAPING_GLYPH_PROPERTIES as ::windows::core::DefaultType>::DefaultType),
                actualglyphcount,
                ::core::mem::transmute_copy(&modifiedclustermap),
                ::core::mem::transmute_copy(&modifiedglyphindices),
                ::core::mem::transmute_copy(&modifiedglyphadvances),
                ::core::mem::transmute_copy(&modifiedglyphoffsets),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextAnalyzer1>,
            ::windows::core::GetTrustLevel,
            ApplyCharacterSpacing::<Impl, OFFSET>,
            GetBaseline::<Impl, OFFSET>,
            AnalyzeVerticalGlyphOrientation::<Impl, OFFSET>,
            GetGlyphOrientationTransform::<Impl, OFFSET>,
            GetScriptProperties::<Impl, OFFSET>,
            GetTextComplexity::<Impl, OFFSET>,
            GetJustificationOpportunities::<Impl, OFFSET>,
            JustifyGlyphAdvances::<Impl, OFFSET>,
            GetJustifiedGlyphs::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteTextAnalyzer2Impl: Sized + IDWriteTextAnalyzer1Impl + IDWriteTextAnalyzerImpl {
    fn GetGlyphOrientationTransform();
    fn GetTypographicFeatures();
    fn CheckTypographicFeature();
}
impl ::windows::core::RuntimeName for IDWriteTextAnalyzer2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextAnalyzer2";
}
impl IDWriteTextAnalyzer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>() -> IDWriteTextAnalyzer2Vtbl {
        unsafe extern "system" fn GetGlyphOrientationTransform<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphOrientationTransform(glyphorientationangle, &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), originx, originy, ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypographicFeatures<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypographicFeatures(
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                maxtagcount,
                ::core::mem::transmute_copy(&actualtagcount),
                ::core::mem::transmute_copy(&tags),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckTypographicFeature<Impl: IDWriteTextAnalyzer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontface: ::windows::core::RawPtr, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: super::super::Foundation::PWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckTypographicFeature(
                &*(&fontface as *const <IDWriteFontFace as ::windows::core::Abi>::Abi as *const <IDWriteFontFace as ::windows::core::DefaultType>::DefaultType),
                &*(&scriptanalysis as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::Abi>::Abi as *const <DWRITE_SCRIPT_ANALYSIS as ::windows::core::DefaultType>::DefaultType),
                &*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                featuretag,
                glyphcount,
                glyphindices,
                ::core::mem::transmute_copy(&featureapplies),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextAnalyzer2>, ::windows::core::GetTrustLevel, GetGlyphOrientationTransform::<Impl, OFFSET>, GetTypographicFeatures::<Impl, OFFSET>, CheckTypographicFeature::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteTextFormat {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextFormat";
}
impl IDWriteTextFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormatImpl, const OFFSET: isize>() -> IDWriteTextFormatVtbl {
        unsafe extern "system" fn SetTextAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextAlignment(textalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParagraphAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParagraphAlignment(paragraphalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWordWrapping<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWordWrapping(wordwrapping) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadingDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReadingDirection(readingdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlowDirection(flowdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncrementalTabStop<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, incrementaltabstop: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIncrementalTabStop(incrementaltabstop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimming<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrimming(&*(&trimmingoptions as *const <DWRITE_TRIMMING as ::windows::core::Abi>::Abi as *const <DWRITE_TRIMMING as ::windows::core::DefaultType>::DefaultType), &*(&trimmingsign as *const <IDWriteInlineObject as ::windows::core::Abi>::Abi as *const <IDWriteInlineObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineSpacing(linespacingmethod, linespacing, baseline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParagraphAlignment<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParagraphAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordWrapping<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_WORD_WRAPPING {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWordWrapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadingDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadingDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowDirection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FLOW_DIRECTION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncrementalTabStop<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncrementalTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrimming<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrimming(::core::mem::transmute_copy(&trimmingoptions), ::core::mem::transmute_copy(&trimmingsign)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacing(::core::mem::transmute_copy(&linespacingmethod), ::core::mem::transmute_copy(&linespacing), ::core::mem::transmute_copy(&baseline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCollection(::core::mem::transmute_copy(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyName<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyName(::core::mem::transmute_copy(&fontfamilyname), namesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontWeight<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontStyle<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontStretch<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSize<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, namesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleName(::core::mem::transmute_copy(&localename), namesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextFormat>,
            ::windows::core::GetTrustLevel,
            SetTextAlignment::<Impl, OFFSET>,
            SetParagraphAlignment::<Impl, OFFSET>,
            SetWordWrapping::<Impl, OFFSET>,
            SetReadingDirection::<Impl, OFFSET>,
            SetFlowDirection::<Impl, OFFSET>,
            SetIncrementalTabStop::<Impl, OFFSET>,
            SetTrimming::<Impl, OFFSET>,
            SetLineSpacing::<Impl, OFFSET>,
            GetTextAlignment::<Impl, OFFSET>,
            GetParagraphAlignment::<Impl, OFFSET>,
            GetWordWrapping::<Impl, OFFSET>,
            GetReadingDirection::<Impl, OFFSET>,
            GetFlowDirection::<Impl, OFFSET>,
            GetIncrementalTabStop::<Impl, OFFSET>,
            GetTrimming::<Impl, OFFSET>,
            GetLineSpacing::<Impl, OFFSET>,
            GetFontCollection::<Impl, OFFSET>,
            GetFontFamilyNameLength::<Impl, OFFSET>,
            GetFontFamilyName::<Impl, OFFSET>,
            GetFontWeight::<Impl, OFFSET>,
            GetFontStyle::<Impl, OFFSET>,
            GetFontStretch::<Impl, OFFSET>,
            GetFontSize::<Impl, OFFSET>,
            GetLocaleNameLength::<Impl, OFFSET>,
            GetLocaleName::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDWriteTextFormat1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextFormat1";
}
impl IDWriteTextFormat1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat1Impl, const OFFSET: isize>() -> IDWriteTextFormat1Vtbl {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVerticalGlyphOrientation(glyphorientation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerticalGlyphOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastLineWrapping<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastLineWrapping(&*(&islastlinewrappingenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastLineWrapping<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastLineWrapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpticalAlignment<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpticalAlignment(opticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpticalAlignment<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpticalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFallback<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontFallback(&*(&fontfallback as *const <IDWriteFontFallback as ::windows::core::Abi>::Abi as *const <IDWriteFontFallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFallback<Impl: IDWriteTextFormat1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFallback(::core::mem::transmute_copy(&fontfallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextFormat1>,
            ::windows::core::GetTrustLevel,
            SetVerticalGlyphOrientation::<Impl, OFFSET>,
            GetVerticalGlyphOrientation::<Impl, OFFSET>,
            SetLastLineWrapping::<Impl, OFFSET>,
            GetLastLineWrapping::<Impl, OFFSET>,
            SetOpticalAlignment::<Impl, OFFSET>,
            GetOpticalAlignment::<Impl, OFFSET>,
            SetFontFallback::<Impl, OFFSET>,
            GetFontFallback::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteTextFormat2Impl: Sized + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetLineSpacing();
    fn GetLineSpacing();
}
impl ::windows::core::RuntimeName for IDWriteTextFormat2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextFormat2";
}
impl IDWriteTextFormat2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat2Impl, const OFFSET: isize>() -> IDWriteTextFormat2Vtbl {
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineSpacing(&*(&linespacingoptions as *const <DWRITE_LINE_SPACING as ::windows::core::Abi>::Abi as *const <DWRITE_LINE_SPACING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacing(::core::mem::transmute_copy(&linespacingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextFormat2>, ::windows::core::GetTrustLevel, SetLineSpacing::<Impl, OFFSET>, GetLineSpacing::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextFormat3Impl: Sized + IDWriteTextFormat2Impl + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
impl ::windows::core::RuntimeName for IDWriteTextFormat3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextFormat3";
}
impl IDWriteTextFormat3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextFormat3Impl, const OFFSET: isize>() -> IDWriteTextFormat3Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontAxisValues(&*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValueCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValues(::core::mem::transmute_copy(&fontaxisvalues), fontaxisvaluecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticFontAxes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Impl: IDWriteTextFormat3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutomaticFontAxes(automaticfontaxes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextFormat3>, ::windows::core::GetTrustLevel, SetFontAxisValues::<Impl, OFFSET>, GetFontAxisValueCount::<Impl, OFFSET>, GetFontAxisValues::<Impl, OFFSET>, GetAutomaticFontAxes::<Impl, OFFSET>, SetAutomaticFontAxes::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteTextLayout {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout";
}
impl IDWriteTextLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayoutImpl, const OFFSET: isize>() -> IDWriteTextLayoutVtbl {
        unsafe extern "system" fn SetMaxWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxWidth(maxwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxHeight(maxheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontCollection<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontcollection: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontCollection(&*(&fontcollection as *const <IDWriteFontCollection as ::windows::core::Abi>::Abi as *const <IDWriteFontCollection as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFamilyName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfamilyname: super::super::Foundation::PWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontFamilyName(&*(&fontfamilyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontWeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontWeight(fontweight, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontStyle(fontstyle, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStretch<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontStretch(fontstretch, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontSize<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontSize(fontsize, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnderline(&*(&hasunderline as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrikethrough(&*(&hasstrikethrough as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawingEffect<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDrawingEffect(&*(&drawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInlineObject<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inlineobject: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInlineObject(&*(&inlineobject as *const <IDWriteInlineObject as ::windows::core::Abi>::Abi as *const <IDWriteInlineObject as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypography<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typography: ::windows::core::RawPtr, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypography(&*(&typography as *const <IDWriteTypography as ::windows::core::Abi>::Abi as *const <IDWriteTypography as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocaleName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localename: super::super::Foundation::PWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocaleName(&*(&localename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxHeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontCollection<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontcollection: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCollection(currentposition, ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyNameLength(currentposition, ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontfamilyname: super::super::Foundation::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFamilyName(currentposition, ::core::mem::transmute_copy(&fontfamilyname), namesize, ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontWeight<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontWeight(currentposition, ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontStyle<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontStyle(currentposition, ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontStretch<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontStretch(currentposition, ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSize<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontSize(currentposition, ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnderline<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderline(currentposition, ::core::mem::transmute_copy(&hasunderline), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrikethrough<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrikethrough(currentposition, ::core::mem::transmute_copy(&hasstrikethrough), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrawingEffect<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDrawingEffect(currentposition, ::core::mem::transmute_copy(&drawingeffect), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInlineObject<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, inlineobject: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInlineObject(currentposition, ::core::mem::transmute_copy(&inlineobject), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypography<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, typography: *mut ::windows::core::RawPtr, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypography(currentposition, ::core::mem::transmute_copy(&typography), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleNameLength(currentposition, ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, localename: super::super::Foundation::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocaleName(currentposition, ::core::mem::transmute_copy(&localename), namesize, ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::windows::core::RawPtr, originx: f32, originy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&renderer as *const <IDWriteTextRenderer as ::windows::core::Abi>::Abi as *const <IDWriteTextRenderer as ::windows::core::DefaultType>::DefaultType), originx, originy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineMetrics(::core::mem::transmute_copy(&linemetrics), maxlinecount, ::core::mem::transmute_copy(&actuallinecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetrics(::core::mem::transmute_copy(&textmetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverhangMetrics(::core::mem::transmute_copy(&overhangs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClusterMetrics<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClusterMetrics(::core::mem::transmute_copy(&clustermetrics), maxclustercount, ::core::mem::transmute_copy(&actualclustercount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetermineMinWidth<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetermineMinWidth(::core::mem::transmute_copy(&minwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestPoint<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestPoint(pointx, pointy, ::core::mem::transmute_copy(&istrailinghit), ::core::mem::transmute_copy(&isinside), ::core::mem::transmute_copy(&hittestmetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestTextPosition<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestTextPosition(textposition, &*(&istrailinghit as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pointx), ::core::mem::transmute_copy(&pointy), ::core::mem::transmute_copy(&hittestmetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestTextRange<Impl: IDWriteTextLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute_copy(&hittestmetrics), maxhittestmetricscount, ::core::mem::transmute_copy(&actualhittestmetricscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextLayout>,
            ::windows::core::GetTrustLevel,
            SetMaxWidth::<Impl, OFFSET>,
            SetMaxHeight::<Impl, OFFSET>,
            SetFontCollection::<Impl, OFFSET>,
            SetFontFamilyName::<Impl, OFFSET>,
            SetFontWeight::<Impl, OFFSET>,
            SetFontStyle::<Impl, OFFSET>,
            SetFontStretch::<Impl, OFFSET>,
            SetFontSize::<Impl, OFFSET>,
            SetUnderline::<Impl, OFFSET>,
            SetStrikethrough::<Impl, OFFSET>,
            SetDrawingEffect::<Impl, OFFSET>,
            SetInlineObject::<Impl, OFFSET>,
            SetTypography::<Impl, OFFSET>,
            SetLocaleName::<Impl, OFFSET>,
            GetMaxWidth::<Impl, OFFSET>,
            GetMaxHeight::<Impl, OFFSET>,
            GetFontCollection::<Impl, OFFSET>,
            GetFontFamilyNameLength::<Impl, OFFSET>,
            GetFontFamilyName::<Impl, OFFSET>,
            GetFontWeight::<Impl, OFFSET>,
            GetFontStyle::<Impl, OFFSET>,
            GetFontStretch::<Impl, OFFSET>,
            GetFontSize::<Impl, OFFSET>,
            GetUnderline::<Impl, OFFSET>,
            GetStrikethrough::<Impl, OFFSET>,
            GetDrawingEffect::<Impl, OFFSET>,
            GetInlineObject::<Impl, OFFSET>,
            GetTypography::<Impl, OFFSET>,
            GetLocaleNameLength::<Impl, OFFSET>,
            GetLocaleName::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            GetLineMetrics::<Impl, OFFSET>,
            GetMetrics::<Impl, OFFSET>,
            GetOverhangMetrics::<Impl, OFFSET>,
            GetClusterMetrics::<Impl, OFFSET>,
            DetermineMinWidth::<Impl, OFFSET>,
            HitTestPoint::<Impl, OFFSET>,
            HitTestTextPosition::<Impl, OFFSET>,
            HitTestTextRange::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteTextLayout1Impl: Sized + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetPairKerning();
    fn GetPairKerning();
    fn SetCharacterSpacing();
    fn GetCharacterSpacing();
}
impl ::windows::core::RuntimeName for IDWriteTextLayout1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout1";
}
impl IDWriteTextLayout1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout1Impl, const OFFSET: isize>() -> IDWriteTextLayout1Vtbl {
        unsafe extern "system" fn SetPairKerning<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPairKerning(&*(&ispairkerningenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPairKerning<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPairKerning(currentposition, ::core::mem::transmute_copy(&ispairkerningenabled), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterSpacing<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterSpacing<Impl: IDWriteTextLayout1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacterSpacing(currentposition, ::core::mem::transmute_copy(&leadingspacing), ::core::mem::transmute_copy(&trailingspacing), ::core::mem::transmute_copy(&minimumadvancewidth), ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextLayout1>, ::windows::core::GetTrustLevel, SetPairKerning::<Impl, OFFSET>, GetPairKerning::<Impl, OFFSET>, SetCharacterSpacing::<Impl, OFFSET>, GetCharacterSpacing::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDWriteTextLayout2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout2";
}
impl IDWriteTextLayout2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout2Impl, const OFFSET: isize>() -> IDWriteTextLayout2Vtbl {
        unsafe extern "system" fn GetMetrics<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetrics(::core::mem::transmute_copy(&textmetrics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVerticalGlyphOrientation(glyphorientation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVerticalGlyphOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastLineWrapping<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastLineWrapping(&*(&islastlinewrappingenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastLineWrapping<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastLineWrapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpticalAlignment<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpticalAlignment(opticalalignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpticalAlignment<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpticalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFallback<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontFallback(&*(&fontfallback as *const <IDWriteFontFallback as ::windows::core::Abi>::Abi as *const <IDWriteFontFallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFallback<Impl: IDWriteTextLayout2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFallback(::core::mem::transmute_copy(&fontfallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDWriteTextLayout2>,
            ::windows::core::GetTrustLevel,
            GetMetrics::<Impl, OFFSET>,
            SetVerticalGlyphOrientation::<Impl, OFFSET>,
            GetVerticalGlyphOrientation::<Impl, OFFSET>,
            SetLastLineWrapping::<Impl, OFFSET>,
            GetLastLineWrapping::<Impl, OFFSET>,
            SetOpticalAlignment::<Impl, OFFSET>,
            GetOpticalAlignment::<Impl, OFFSET>,
            SetFontFallback::<Impl, OFFSET>,
            GetFontFallback::<Impl, OFFSET>,
        )
    }
}
pub trait IDWriteTextLayout3Impl: Sized + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn InvalidateLayout();
    fn SetLineSpacing();
    fn GetLineSpacing();
    fn GetLineMetrics();
}
impl ::windows::core::RuntimeName for IDWriteTextLayout3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout3";
}
impl IDWriteTextLayout3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout3Impl, const OFFSET: isize>() -> IDWriteTextLayout3Vtbl {
        unsafe extern "system" fn InvalidateLayout<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateLayout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineSpacing(&*(&linespacingoptions as *const <DWRITE_LINE_SPACING as ::windows::core::Abi>::Abi as *const <DWRITE_LINE_SPACING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacing(::core::mem::transmute_copy(&linespacingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineMetrics<Impl: IDWriteTextLayout3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineMetrics(::core::mem::transmute_copy(&linemetrics), maxlinecount, ::core::mem::transmute_copy(&actuallinecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextLayout3>, ::windows::core::GetTrustLevel, InvalidateLayout::<Impl, OFFSET>, SetLineSpacing::<Impl, OFFSET>, GetLineSpacing::<Impl, OFFSET>, GetLineMetrics::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextLayout4Impl: Sized + IDWriteTextLayout3Impl + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
impl ::windows::core::RuntimeName for IDWriteTextLayout4 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout4";
}
impl IDWriteTextLayout4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextLayout4Impl, const OFFSET: isize>() -> IDWriteTextLayout4Vtbl {
        unsafe extern "system" fn SetFontAxisValues<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontAxisValues(&*(&fontaxisvalues as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_AXIS_VALUE as ::windows::core::DefaultType>::DefaultType), fontaxisvaluecount, &*(&textrange as *const <DWRITE_TEXT_RANGE as ::windows::core::Abi>::Abi as *const <DWRITE_TEXT_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValueCount(currentposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAxisValues(currentposition, ::core::mem::transmute_copy(&fontaxisvalues), fontaxisvaluecount, ::core::mem::transmute_copy(&textrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticFontAxes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Impl: IDWriteTextLayout4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutomaticFontAxes(automaticfontaxes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextLayout4>, ::windows::core::GetTrustLevel, SetFontAxisValues::<Impl, OFFSET>, GetFontAxisValueCount::<Impl, OFFSET>, GetFontAxisValues::<Impl, OFFSET>, GetAutomaticFontAxes::<Impl, OFFSET>, SetAutomaticFontAxes::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextRendererImpl: Sized + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
impl ::windows::core::RuntimeName for IDWriteTextRenderer {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextRenderer";
}
impl IDWriteTextRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextRendererImpl, const OFFSET: isize>() -> IDWriteTextRendererVtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawGlyphRun(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                measuringmode,
                &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrundescription as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawUnderline<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawUnderline(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                &*(&underline as *const <DWRITE_UNDERLINE as ::windows::core::Abi>::Abi as *const <DWRITE_UNDERLINE as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawStrikethrough<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawStrikethrough(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                &*(&strikethrough as *const <DWRITE_STRIKETHROUGH as ::windows::core::Abi>::Abi as *const <DWRITE_STRIKETHROUGH as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawInlineObject<Impl: IDWriteTextRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawInlineObject(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                originx,
                originy,
                &*(&inlineobject as *const <IDWriteInlineObject as ::windows::core::Abi>::Abi as *const <IDWriteInlineObject as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextRenderer>, ::windows::core::GetTrustLevel, DrawGlyphRun::<Impl, OFFSET>, DrawUnderline::<Impl, OFFSET>, DrawStrikethrough::<Impl, OFFSET>, DrawInlineObject::<Impl, OFFSET>)
    }
}
pub trait IDWriteTextRenderer1Impl: Sized + IDWriteTextRendererImpl + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
impl ::windows::core::RuntimeName for IDWriteTextRenderer1 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTextRenderer1";
}
impl IDWriteTextRenderer1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>() -> IDWriteTextRenderer1Vtbl {
        unsafe extern "system" fn DrawGlyphRun<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawGlyphRun(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                orientationangle,
                measuringmode,
                &*(&glyphrun as *const <DWRITE_GLYPH_RUN as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN as ::windows::core::DefaultType>::DefaultType),
                &*(&glyphrundescription as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::Abi>::Abi as *const <DWRITE_GLYPH_RUN_DESCRIPTION as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawUnderline<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawUnderline(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                orientationangle,
                &*(&underline as *const <DWRITE_UNDERLINE as ::windows::core::Abi>::Abi as *const <DWRITE_UNDERLINE as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawStrikethrough<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawStrikethrough(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                baselineoriginx,
                baselineoriginy,
                orientationangle,
                &*(&strikethrough as *const <DWRITE_STRIKETHROUGH as ::windows::core::Abi>::Abi as *const <DWRITE_STRIKETHROUGH as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawInlineObject<Impl: IDWriteTextRenderer1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: ::windows::core::RawPtr, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawInlineObject(
                &*(&clientdrawingcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                originx,
                originy,
                orientationangle,
                &*(&inlineobject as *const <IDWriteInlineObject as ::windows::core::Abi>::Abi as *const <IDWriteInlineObject as ::windows::core::DefaultType>::DefaultType),
                &*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&isrighttoleft as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&clientdrawingeffect as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTextRenderer1>, ::windows::core::GetTrustLevel, DrawGlyphRun::<Impl, OFFSET>, DrawUnderline::<Impl, OFFSET>, DrawStrikethrough::<Impl, OFFSET>, DrawInlineObject::<Impl, OFFSET>)
    }
}
pub trait IDWriteTypographyImpl: Sized {
    fn AddFontFeature();
    fn GetFontFeatureCount();
    fn GetFontFeature();
}
impl ::windows::core::RuntimeName for IDWriteTypography {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectWrite.IDWriteTypography";
}
impl IDWriteTypographyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDWriteTypographyImpl, const OFFSET: isize>() -> IDWriteTypographyVtbl {
        unsafe extern "system" fn AddFontFeature<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFontFeature(&*(&fontfeature as *const <DWRITE_FONT_FEATURE as ::windows::core::Abi>::Abi as *const <DWRITE_FONT_FEATURE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFeatureCount<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFeatureCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFeature<Impl: IDWriteTypographyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFeature(fontfeatureindex, ::core::mem::transmute_copy(&fontfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDWriteTypography>, ::windows::core::GetTrustLevel, AddFontFeature::<Impl, OFFSET>, GetFontFeatureCount::<Impl, OFFSET>, GetFontFeature::<Impl, OFFSET>)
    }
}
