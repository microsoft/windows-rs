pub trait IDynamicRendererImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn HWND();
    fn SetHWND();
    fn ClipRectangle();
    fn SetClipRectangle();
    fn ClipRegion();
    fn SetClipRegion();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn DataCacheEnabled();
    fn SetDataCacheEnabled();
    fn ReleaseCachedData();
    fn Refresh();
    fn Draw();
}
impl ::windows::core::RuntimeName for IDynamicRenderer {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IDynamicRenderer";
}
impl IDynamicRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRendererImpl, const OFFSET: isize>() -> IDynamicRendererVtbl {
        unsafe extern "system" fn Enabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&benabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(&*(&benabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HWND<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HWND(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHWND(&*(&hwnd as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipRectangle<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipRectangle(::core::mem::transmute_copy(&prccliprect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRectangle<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClipRectangle(&*(&prccliprect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipRegion<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipRegion(::core::mem::transmute_copy(&phcliprgn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRegion<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClipRegion(&*(&hcliprgn as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppida: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingAttributes(::core::mem::transmute_copy(&ppida)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pida: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DrawingAttributes(&*(&pida as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCacheEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCacheEnabled(::core::mem::transmute_copy(&pfcachedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCacheEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDataCacheEnabled(&*(&fcachedata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCachedData<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseCachedData(strokeid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&hdc as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IDynamicRenderer>,
            ::windows::core::GetTrustLevel,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            HWND::<Impl, OFFSET>,
            SetHWND::<Impl, OFFSET>,
            ClipRectangle::<Impl, OFFSET>,
            SetClipRectangle::<Impl, OFFSET>,
            ClipRegion::<Impl, OFFSET>,
            SetClipRegion::<Impl, OFFSET>,
            DrawingAttributes::<Impl, OFFSET>,
            putref_DrawingAttributes::<Impl, OFFSET>,
            DataCacheEnabled::<Impl, OFFSET>,
            SetDataCacheEnabled::<Impl, OFFSET>,
            ReleaseCachedData::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
        )
    }
}
pub trait IGestureRecognizerImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn MaxStrokeCount();
    fn SetMaxStrokeCount();
    fn EnableGestures();
    fn Reset();
}
impl ::windows::core::RuntimeName for IGestureRecognizer {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IGestureRecognizer";
}
impl IGestureRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizerImpl, const OFFSET: isize>() -> IGestureRecognizerVtbl {
        unsafe extern "system" fn Enabled<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(&*(&fenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxStrokeCount<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstrokes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxStrokeCount(::core::mem::transmute_copy(&pcstrokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStrokeCount<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstrokes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxStrokeCount(cstrokes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableGestures<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableGestures(cgestures, pgestures) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGestureRecognizer>, ::windows::core::GetTrustLevel, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, MaxStrokeCount::<Impl, OFFSET>, SetMaxStrokeCount::<Impl, OFFSET>, EnableGestures::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IHandwrittenTextInsertionImpl: Sized {
    fn InsertRecognitionResultsArray();
    fn InsertInkRecognitionResult();
}
impl ::windows::core::RuntimeName for IHandwrittenTextInsertion {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IHandwrittenTextInsertion";
}
impl IHandwrittenTextInsertionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandwrittenTextInsertionImpl, const OFFSET: isize>() -> IHandwrittenTextInsertionVtbl {
        unsafe extern "system" fn InsertRecognitionResultsArray<Impl: IHandwrittenTextInsertionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertRecognitionResultsArray(&*(&psaalternates as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), locale, &*(&falternatecontainsautospacinginformation as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Impl: IHandwrittenTextInsertionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkrecoresult: ::windows::core::RawPtr, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertInkRecognitionResult(&*(&piinkrecoresult as *const <IInkRecognitionResult as ::windows::core::Abi>::Abi as *const <IInkRecognitionResult as ::windows::core::DefaultType>::DefaultType), locale, &*(&falternatecontainsautospacinginformation as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHandwrittenTextInsertion>, ::windows::core::GetTrustLevel, InsertRecognitionResultsArray::<Impl, OFFSET>, InsertInkRecognitionResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInk {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInk";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkImpl, const OFFSET: isize>() -> IInkVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInk>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCollectorImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn Enabled();
    fn SetEnabled();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCollector {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCollector";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCollectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollectorImpl, const OFFSET: isize>() -> IInkCollectorVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hWnd(::core::mem::transmute_copy(&currentwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SethWnd(newwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(collecting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDrawingAttributes(::core::mem::transmute_copy(&currentattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DefaultDrawingAttributes(&*(&newattributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Renderer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Renderer(::core::mem::transmute_copy(&currentinkrenderer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Renderer(&*(&newinkrenderer as *const <IInkRenderer as ::windows::core::Abi>::Abi as *const <IInkRenderer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Ink(&*(&newink as *const <IInkDisp as ::windows::core::Abi>::Abi as *const <IInkDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRedraw(::core::mem::transmute_copy(&autoredraw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoRedraw(autoredraw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectingInk(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCollectionMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicRendering(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDynamicRendering(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPacketDescription(::core::mem::transmute_copy(&packetguids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredPacketDescription(&*(&packetguids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseIcon(::core::mem::transmute_copy(&mouseicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_MouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MousePointer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MousePointer(::core::mem::transmute_copy(&mousepointer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMousePointer(mousepointer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cursors(::core::mem::transmute_copy(&cursors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginX(::core::mem::transmute_copy(&marginx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginX(marginx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginY<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginY(::core::mem::transmute_copy(&marginy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginY(marginy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tablet<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tablet(::core::mem::transmute_copy(&singletablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportHighContrastInk(::core::mem::transmute_copy(&support)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportHighContrastInk(support) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGestureStatus(gesture, listen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGestureStatus(gesture, ::core::mem::transmute_copy(&listening)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllTabletsMode(usemouseforinput) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSingleTabletIntegratedMode(&*(&tablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventInterest(eventid, ::core::mem::transmute_copy(&listen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterest(eventid, listen) {
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
            ::windows::core::GetRuntimeClassName::<IInkCollector>,
            ::windows::core::GetTrustLevel,
            hWnd::<Impl, OFFSET>,
            SethWnd::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            DefaultDrawingAttributes::<Impl, OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, OFFSET>,
            Renderer::<Impl, OFFSET>,
            putref_Renderer::<Impl, OFFSET>,
            Ink::<Impl, OFFSET>,
            putref_Ink::<Impl, OFFSET>,
            AutoRedraw::<Impl, OFFSET>,
            SetAutoRedraw::<Impl, OFFSET>,
            CollectingInk::<Impl, OFFSET>,
            CollectionMode::<Impl, OFFSET>,
            SetCollectionMode::<Impl, OFFSET>,
            DynamicRendering::<Impl, OFFSET>,
            SetDynamicRendering::<Impl, OFFSET>,
            DesiredPacketDescription::<Impl, OFFSET>,
            SetDesiredPacketDescription::<Impl, OFFSET>,
            MouseIcon::<Impl, OFFSET>,
            SetMouseIcon::<Impl, OFFSET>,
            putref_MouseIcon::<Impl, OFFSET>,
            MousePointer::<Impl, OFFSET>,
            SetMousePointer::<Impl, OFFSET>,
            Cursors::<Impl, OFFSET>,
            MarginX::<Impl, OFFSET>,
            SetMarginX::<Impl, OFFSET>,
            MarginY::<Impl, OFFSET>,
            SetMarginY::<Impl, OFFSET>,
            Tablet::<Impl, OFFSET>,
            SupportHighContrastInk::<Impl, OFFSET>,
            SetSupportHighContrastInk::<Impl, OFFSET>,
            SetGestureStatus::<Impl, OFFSET>,
            GetGestureStatus::<Impl, OFFSET>,
            GetWindowInputRectangle::<Impl, OFFSET>,
            SetWindowInputRectangle::<Impl, OFFSET>,
            SetAllTabletsMode::<Impl, OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, OFFSET>,
            GetEventInterest::<Impl, OFFSET>,
            SetEventInterest::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn Inverted();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn Tablet();
    fn Buttons();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCursor {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCursor";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorImpl, const OFFSET: isize>() -> IInkCursorVtbl {
        unsafe extern "system" fn Name<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverted<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inverted(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingAttributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DrawingAttributes(&*(&attributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tablet<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tablet(::core::mem::transmute_copy(&tablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buttons<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttons: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buttons(::core::mem::transmute_copy(&buttons)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCursor>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Id::<Impl, OFFSET>, Inverted::<Impl, OFFSET>, DrawingAttributes::<Impl, OFFSET>, putref_DrawingAttributes::<Impl, OFFSET>, Tablet::<Impl, OFFSET>, Buttons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButtonImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCursorButton {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCursorButton";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtonImpl, const OFFSET: isize>() -> IInkCursorButtonVtbl {
        unsafe extern "system" fn Name<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&currentstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCursorButton>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Id::<Impl, OFFSET>, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButtonsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCursorButtons {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCursorButtons";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorButtonsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtonsImpl, const OFFSET: isize>() -> IInkCursorButtonsVtbl {
        unsafe extern "system" fn Count<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&identifier as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&button)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCursorButtons>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCursors {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCursors";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorsImpl, const OFFSET: isize>() -> IInkCursorsVtbl {
        unsafe extern "system" fn Count<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&cursor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCursors>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCustomStrokesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkCustomStrokes {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkCustomStrokes";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkCustomStrokesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokesImpl, const OFFSET: isize>() -> IInkCustomStrokesVtbl {
        unsafe extern "system" fn Count<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&identifier as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&identifier as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCustomStrokes>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDispImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn ExtendedProperties();
    fn Dirty();
    fn SetDirty();
    fn CustomStrokes();
    fn GetBoundingBox();
    fn DeleteStrokes();
    fn DeleteStroke();
    fn ExtractStrokes();
    fn ExtractWithRectangle();
    fn Clip();
    fn Clone();
    fn HitTestCircle();
    fn HitTestWithRectangle();
    fn HitTestWithLasso();
    fn NearestPoint();
    fn CreateStrokes();
    fn AddStrokesAtRectangle();
    fn Save();
    fn Load();
    fn CreateStroke();
    fn ClipboardCopyWithRectangle();
    fn ClipboardCopy();
    fn CanPaste();
    fn ClipboardPaste();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDisp {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDisp";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDispImpl, const OFFSET: isize>() -> IInkDispVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dirty<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dirty(::core::mem::transmute_copy(&dirty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirty<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirty(dirty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkinkcustomstrokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomStrokes(::core::mem::transmute_copy(&ppunkinkcustomstrokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingBox(boundingboxmode, ::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteStrokes(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStroke<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteStroke(&*(&stroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtractStrokes(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), extractflags, ::core::mem::transmute_copy(&extractedink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtractWithRectangle(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType), extractflags, ::core::mem::transmute_copy(&extractedink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&newink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestCircle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestCircle(x, y, radius, ::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionrectangle: ::windows::core::RawPtr, intersectpercent: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestWithRectangle(&*(&selectionrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType), intersectpercent, ::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithLasso<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestWithLasso(&*(&points as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), intersectpercent, &*(&lassopoints as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearestPoint<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearestPoint(x, y, pointonstroke, distancefrompacket, ::core::mem::transmute_copy(&stroke)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStrokes(&*(&strokeids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStrokesAtRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestrokes: ::windows::core::RawPtr, targetrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStrokesAtRectangle(&*(&sourcestrokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), &*(&targetrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(persistenceformat, compressionmode, ::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&data as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStroke<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStroke(&*(&packetdata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&packetdescription as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&stroke)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopyWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipboardCopyWithRectangle(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType), clipboardformats, clipboardmodes, ::core::mem::transmute_copy(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopy<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipboardCopy(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), clipboardformats, clipboardmodes, ::core::mem::transmute_copy(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataobject: ::windows::core::RawPtr, canpaste: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPaste(&*(&dataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&canpaste)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardPaste<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, dataobject: ::windows::core::RawPtr, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipboardPaste(x, y, &*(&dataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strokes)) {
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
            ::windows::core::GetRuntimeClassName::<IInkDisp>,
            ::windows::core::GetTrustLevel,
            Strokes::<Impl, OFFSET>,
            ExtendedProperties::<Impl, OFFSET>,
            Dirty::<Impl, OFFSET>,
            SetDirty::<Impl, OFFSET>,
            CustomStrokes::<Impl, OFFSET>,
            GetBoundingBox::<Impl, OFFSET>,
            DeleteStrokes::<Impl, OFFSET>,
            DeleteStroke::<Impl, OFFSET>,
            ExtractStrokes::<Impl, OFFSET>,
            ExtractWithRectangle::<Impl, OFFSET>,
            Clip::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            HitTestCircle::<Impl, OFFSET>,
            HitTestWithRectangle::<Impl, OFFSET>,
            HitTestWithLasso::<Impl, OFFSET>,
            NearestPoint::<Impl, OFFSET>,
            CreateStrokes::<Impl, OFFSET>,
            AddStrokesAtRectangle::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            Load::<Impl, OFFSET>,
            CreateStroke::<Impl, OFFSET>,
            ClipboardCopyWithRectangle::<Impl, OFFSET>,
            ClipboardCopy::<Impl, OFFSET>,
            CanPaste::<Impl, OFFSET>,
            ClipboardPaste::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDividerImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn putref_Strokes();
    fn RecognizerContext();
    fn putref_RecognizerContext();
    fn LineHeight();
    fn SetLineHeight();
    fn Divide();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDivider {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDivider";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDividerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDividerImpl, const OFFSET: isize>() -> IInkDividerVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Strokes(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizerContext<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizerContext(::core::mem::transmute_copy(&recognizercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RecognizerContext<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_RecognizerContext(&*(&recognizercontext as *const <IInkRecognizerContext as ::windows::core::Abi>::Abi as *const <IInkRecognizerContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineHeight<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeight(::core::mem::transmute_copy(&lineheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineHeight(lineheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdivisionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Divide(::core::mem::transmute_copy(&inkdivisionresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkDivider>, ::windows::core::GetTrustLevel, Strokes::<Impl, OFFSET>, putref_Strokes::<Impl, OFFSET>, RecognizerContext::<Impl, OFFSET>, putref_RecognizerContext::<Impl, OFFSET>, LineHeight::<Impl, OFFSET>, SetLineHeight::<Impl, OFFSET>, Divide::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionResultImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn ResultByType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDivisionResult {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDivisionResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionResultImpl, const OFFSET: isize>() -> IInkDivisionResultVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDivisionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultByType<Impl: IInkDivisionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultByType(divisiontype, ::core::mem::transmute_copy(&inkdivisionunits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkDivisionResult>, ::windows::core::GetTrustLevel, Strokes::<Impl, OFFSET>, ResultByType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnitImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn DivisionType();
    fn RecognizedString();
    fn RotationTransform();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDivisionUnit {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDivisionUnit";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnitImpl, const OFFSET: isize>() -> IInkDivisionUnitVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DivisionType<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: *mut InkDivisionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DivisionType(::core::mem::transmute_copy(&divisiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizedString<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizedString(::core::mem::transmute_copy(&recostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTransform<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotationtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationTransform(::core::mem::transmute_copy(&rotationtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkDivisionUnit>, ::windows::core::GetTrustLevel, Strokes::<Impl, OFFSET>, DivisionType::<Impl, OFFSET>, RecognizedString::<Impl, OFFSET>, RotationTransform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnitsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDivisionUnits {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDivisionUnits";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnitsImpl, const OFFSET: isize>() -> IInkDivisionUnitsVtbl {
        unsafe extern "system" fn Count<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkdivisionunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&inkdivisionunit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkDivisionUnits>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDrawingAttributesImpl: Sized + IDispatchImpl {
    fn Color();
    fn SetColor();
    fn Width();
    fn SetWidth();
    fn Height();
    fn SetHeight();
    fn FitToCurve();
    fn SetFitToCurve();
    fn IgnorePressure();
    fn SetIgnorePressure();
    fn AntiAliased();
    fn SetAntiAliased();
    fn Transparency();
    fn SetTransparency();
    fn RasterOperation();
    fn SetRasterOperation();
    fn PenTip();
    fn SetPenTip();
    fn ExtendedProperties();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkDrawingAttributes {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkDrawingAttributes";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkDrawingAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesImpl, const OFFSET: isize>() -> IInkDrawingAttributesVtbl {
        unsafe extern "system" fn Color<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcolor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color(::core::mem::transmute_copy(&currentcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColor(newcolor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&currentwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWidth(newwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentheight: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&currentheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeight(newheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FitToCurve(::core::mem::transmute_copy(&flag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFitToCurve(flag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnorePressure(::core::mem::transmute_copy(&flag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIgnorePressure(flag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiAliased<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntiAliased(::core::mem::transmute_copy(&flag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiAliased<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAntiAliased(flag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transparency<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currenttransparency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transparency(::core::mem::transmute_copy(&currenttransparency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransparency<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newtransparency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransparency(newtransparency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RasterOperation<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterOperation(::core::mem::transmute_copy(&currentrasteroperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterOperation<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newrasteroperation: InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRasterOperation(newrasteroperation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpentip: *mut InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PenTip(::core::mem::transmute_copy(&currentpentip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpentip: InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPenTip(newpentip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&drawingattributes)) {
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
            ::windows::core::GetRuntimeClassName::<IInkDrawingAttributes>,
            ::windows::core::GetTrustLevel,
            Color::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            SetWidth::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            SetHeight::<Impl, OFFSET>,
            FitToCurve::<Impl, OFFSET>,
            SetFitToCurve::<Impl, OFFSET>,
            IgnorePressure::<Impl, OFFSET>,
            SetIgnorePressure::<Impl, OFFSET>,
            AntiAliased::<Impl, OFFSET>,
            SetAntiAliased::<Impl, OFFSET>,
            Transparency::<Impl, OFFSET>,
            SetTransparency::<Impl, OFFSET>,
            RasterOperation::<Impl, OFFSET>,
            SetRasterOperation::<Impl, OFFSET>,
            PenTip::<Impl, OFFSET>,
            SetPenTip::<Impl, OFFSET>,
            ExtendedProperties::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkEditImpl: Sized + IDispatchImpl {
    fn Status();
    fn UseMouseForInput();
    fn SetUseMouseForInput();
    fn InkMode();
    fn SetInkMode();
    fn InkInsertMode();
    fn SetInkInsertMode();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn RecognitionTimeout();
    fn SetRecognitionTimeout();
    fn Recognizer();
    fn putref_Recognizer();
    fn Factoid();
    fn SetFactoid();
    fn SelInks();
    fn SetSelInks();
    fn SelInksDisplayMode();
    fn SetSelInksDisplayMode();
    fn Recognize();
    fn GetGestureStatus();
    fn SetGestureStatus();
    fn SetBackColor();
    fn BackColor();
    fn Appearance();
    fn SetAppearance();
    fn BorderStyle();
    fn SetBorderStyle();
    fn Hwnd();
    fn Font();
    fn putref_Font();
    fn Text();
    fn SetText();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn Locked();
    fn SetLocked();
    fn Enabled();
    fn SetEnabled();
    fn MaxLength();
    fn SetMaxLength();
    fn MultiLine();
    fn SetMultiLine();
    fn ScrollBars();
    fn SetScrollBars();
    fn DisableNoScroll();
    fn SetDisableNoScroll();
    fn SelAlignment();
    fn SetSelAlignment();
    fn SelBold();
    fn SetSelBold();
    fn SelItalic();
    fn SetSelItalic();
    fn SelUnderline();
    fn SetSelUnderline();
    fn SelColor();
    fn SetSelColor();
    fn SelFontName();
    fn SetSelFontName();
    fn SelFontSize();
    fn SetSelFontSize();
    fn SelCharOffset();
    fn SetSelCharOffset();
    fn TextRTF();
    fn SetTextRTF();
    fn SelStart();
    fn SetSelStart();
    fn SelLength();
    fn SetSelLength();
    fn SelText();
    fn SetSelText();
    fn SelRTF();
    fn SetSelRTF();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkEdit {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkEdit";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkEditVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkEditImpl, const OFFSET: isize>() -> IInkEditVtbl {
        unsafe extern "system" fn Status<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut InkEditStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseMouseForInput<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseMouseForInput(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseMouseForInput<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseMouseForInput(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkMode(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInkMode(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkInsertMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkInsertMode(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkInsertMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInkInsertMode(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingAttributes(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DrawingAttributes(&*(&newval as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionTimeout<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognitionTimeout(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionTimeout<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecognitionTimeout(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognizer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Recognizer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Recognizer(&*(&newval as *const <IInkRecognizer as ::windows::core::Abi>::Abi as *const <IInkRecognizer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Factoid(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFactoid(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelInks<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselink: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelInks(::core::mem::transmute_copy(&pselink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInks<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selink: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelInks(&*(&selink as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelInksDisplayMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelInksDisplayMode(::core::mem::transmute_copy(&pinkdisplaymode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInksDisplayMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelInksDisplayMode(inkdisplaymode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGestureStatus(gesture, ::core::mem::transmute_copy(&plisten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGestureStatus(gesture, listen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackColor(clr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColor(::core::mem::transmute_copy(&pclr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: *mut AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance(::core::mem::transmute_copy(&pappearance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppearance(pappearance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BorderStyle<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderStyle(::core::mem::transmute_copy(&pborderstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBorderStyle(pborderstyle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hwnd<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pohhwnd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hwnd(::core::mem::transmute_copy(&pohhwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Font<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Font(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Font(&*(&ppfont as *const <super::super::System::Ole::IFontDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IFontDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text(::core::mem::transmute_copy(&pbstrtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(&*(&pbstrtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseIcon(::core::mem::transmute_copy(&mouseicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_MouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MousePointer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MousePointer(::core::mem::transmute_copy(&mousepointer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMousePointer(mousepointer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locked<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locked(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocked<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocked(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLength(::core::mem::transmute_copy(&plmaxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxLength(lmaxlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultiLine<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiLine(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiLine<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultiLine(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollBars<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollBars(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollBars<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScrollBars(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableNoScroll<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableNoScroll(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableNoScroll<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisableNoScroll(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelAlignment<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelAlignment(::core::mem::transmute_copy(&pvarselalignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelAlignment<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelAlignment(&*(&pvarselalignment as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelBold<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelBold(::core::mem::transmute_copy(&pvarselbold)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelBold<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelBold(&*(&pvarselbold as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelItalic<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelItalic(::core::mem::transmute_copy(&pvarselitalic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelItalic<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelItalic(&*(&pvarselitalic as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelUnderline<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelUnderline(::core::mem::transmute_copy(&pvarselunderline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelUnderline<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelUnderline(&*(&pvarselunderline as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelColor(::core::mem::transmute_copy(&pvarselcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelColor(&*(&pvarselcolor as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelFontName<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelFontName(::core::mem::transmute_copy(&pvarselfontname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontName<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelFontName(&*(&pvarselfontname as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelFontSize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelFontSize(::core::mem::transmute_copy(&pvarselfontsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontSize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelFontSize(&*(&pvarselfontsize as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelCharOffset<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelCharOffset(::core::mem::transmute_copy(&pvarselcharoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelCharOffset<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelCharOffset(&*(&pvarselcharoffset as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRTF(::core::mem::transmute_copy(&pbstrtextrtf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTextRTF(&*(&pbstrtextrtf as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelStart<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelStart(::core::mem::transmute_copy(&plselstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelStart<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelStart(plselstart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelLength(::core::mem::transmute_copy(&plsellength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelLength(plsellength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelText(::core::mem::transmute_copy(&pbstrseltext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelText(&*(&pbstrseltext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelRTF(::core::mem::transmute_copy(&pbstrselrtf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelRTF(&*(&pbstrselrtf as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
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
            ::windows::core::GetRuntimeClassName::<IInkEdit>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, OFFSET>,
            UseMouseForInput::<Impl, OFFSET>,
            SetUseMouseForInput::<Impl, OFFSET>,
            InkMode::<Impl, OFFSET>,
            SetInkMode::<Impl, OFFSET>,
            InkInsertMode::<Impl, OFFSET>,
            SetInkInsertMode::<Impl, OFFSET>,
            DrawingAttributes::<Impl, OFFSET>,
            putref_DrawingAttributes::<Impl, OFFSET>,
            RecognitionTimeout::<Impl, OFFSET>,
            SetRecognitionTimeout::<Impl, OFFSET>,
            Recognizer::<Impl, OFFSET>,
            putref_Recognizer::<Impl, OFFSET>,
            Factoid::<Impl, OFFSET>,
            SetFactoid::<Impl, OFFSET>,
            SelInks::<Impl, OFFSET>,
            SetSelInks::<Impl, OFFSET>,
            SelInksDisplayMode::<Impl, OFFSET>,
            SetSelInksDisplayMode::<Impl, OFFSET>,
            Recognize::<Impl, OFFSET>,
            GetGestureStatus::<Impl, OFFSET>,
            SetGestureStatus::<Impl, OFFSET>,
            SetBackColor::<Impl, OFFSET>,
            BackColor::<Impl, OFFSET>,
            Appearance::<Impl, OFFSET>,
            SetAppearance::<Impl, OFFSET>,
            BorderStyle::<Impl, OFFSET>,
            SetBorderStyle::<Impl, OFFSET>,
            Hwnd::<Impl, OFFSET>,
            Font::<Impl, OFFSET>,
            putref_Font::<Impl, OFFSET>,
            Text::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            MouseIcon::<Impl, OFFSET>,
            SetMouseIcon::<Impl, OFFSET>,
            putref_MouseIcon::<Impl, OFFSET>,
            MousePointer::<Impl, OFFSET>,
            SetMousePointer::<Impl, OFFSET>,
            Locked::<Impl, OFFSET>,
            SetLocked::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            MaxLength::<Impl, OFFSET>,
            SetMaxLength::<Impl, OFFSET>,
            MultiLine::<Impl, OFFSET>,
            SetMultiLine::<Impl, OFFSET>,
            ScrollBars::<Impl, OFFSET>,
            SetScrollBars::<Impl, OFFSET>,
            DisableNoScroll::<Impl, OFFSET>,
            SetDisableNoScroll::<Impl, OFFSET>,
            SelAlignment::<Impl, OFFSET>,
            SetSelAlignment::<Impl, OFFSET>,
            SelBold::<Impl, OFFSET>,
            SetSelBold::<Impl, OFFSET>,
            SelItalic::<Impl, OFFSET>,
            SetSelItalic::<Impl, OFFSET>,
            SelUnderline::<Impl, OFFSET>,
            SetSelUnderline::<Impl, OFFSET>,
            SelColor::<Impl, OFFSET>,
            SetSelColor::<Impl, OFFSET>,
            SelFontName::<Impl, OFFSET>,
            SetSelFontName::<Impl, OFFSET>,
            SelFontSize::<Impl, OFFSET>,
            SetSelFontSize::<Impl, OFFSET>,
            SelCharOffset::<Impl, OFFSET>,
            SetSelCharOffset::<Impl, OFFSET>,
            TextRTF::<Impl, OFFSET>,
            SetTextRTF::<Impl, OFFSET>,
            SelStart::<Impl, OFFSET>,
            SetSelStart::<Impl, OFFSET>,
            SelLength::<Impl, OFFSET>,
            SetSelLength::<Impl, OFFSET>,
            SelText::<Impl, OFFSET>,
            SetSelText::<Impl, OFFSET>,
            SelRTF::<Impl, OFFSET>,
            SetSelRTF::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedPropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
    fn DoesPropertyExist();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkExtendedProperties {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkExtendedProperties";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkExtendedPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>() -> IInkExtendedPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&identifier as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&guid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inkextendedproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&identifier as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesPropertyExist<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesPropertyExist(&*(&guid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&doespropertyexist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkExtendedProperties>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Clear::<Impl, OFFSET>, DoesPropertyExist::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedPropertyImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkExtendedProperty {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkExtendedProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkExtendedPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedPropertyImpl, const OFFSET: isize>() -> IInkExtendedPropertyVtbl {
        unsafe extern "system" fn Guid<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&data as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkExtendedProperty>, ::windows::core::GetTrustLevel, Guid::<Impl, OFFSET>, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkGestureImpl: Sized + IDispatchImpl {
    fn Confidence();
    fn Id();
    fn GetHotPoint();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkGesture {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkGesture";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkGestureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkGestureImpl, const OFFSET: isize>() -> IInkGestureVtbl {
        unsafe extern "system" fn Confidence<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence(::core::mem::transmute_copy(&confidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut InkApplicationGesture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotPoint<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHotPoint(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkGesture>, ::windows::core::GetTrustLevel, Confidence::<Impl, OFFSET>, Id::<Impl, OFFSET>, GetHotPoint::<Impl, OFFSET>)
    }
}
pub trait IInkLineInfoImpl: Sized {
    fn SetFormat();
    fn GetFormat();
    fn GetInkExtent();
    fn GetCandidate();
    fn SetCandidate();
    fn Recognize();
}
impl ::windows::core::RuntimeName for IInkLineInfo {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkLineInfo";
}
impl IInkLineInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfoImpl, const OFFSET: isize>() -> IInkLineInfoVtbl {
        unsafe extern "system" fn SetFormat<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&pim as *const <INKMETRIC as ::windows::core::Abi>::Abi as *const <INKMETRIC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(&*(&pim as *const <INKMETRIC as ::windows::core::Abi>::Abi as *const <INKMETRIC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInkExtent<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInkExtent(&*(&pim as *const <INKMETRIC as ::windows::core::Abi>::Abi as *const <INKMETRIC as ::windows::core::DefaultType>::DefaultType), pnwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, pwcrecogword: super::super::Foundation::PWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidate(ncandidatenum, &*(&pwcrecogword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcwcrecogword, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCandidate<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, strrecogword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCandidate(ncandidatenum, &*(&strrecogword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkLineInfo>, ::windows::core::GetTrustLevel, SetFormat::<Impl, OFFSET>, GetFormat::<Impl, OFFSET>, GetInkExtent::<Impl, OFFSET>, GetCandidate::<Impl, OFFSET>, SetCandidate::<Impl, OFFSET>, Recognize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkOverlayImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn Enabled();
    fn SetEnabled();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn EditingMode();
    fn SetEditingMode();
    fn Selection();
    fn SetSelection();
    fn EraserMode();
    fn SetEraserMode();
    fn EraserWidth();
    fn SetEraserWidth();
    fn AttachMode();
    fn SetAttachMode();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SupportHighContrastSelectionUI();
    fn SetSupportHighContrastSelectionUI();
    fn HitTestSelection();
    fn Draw();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkOverlay {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkOverlay";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkOverlayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlayImpl, const OFFSET: isize>() -> IInkOverlayVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hWnd(::core::mem::transmute_copy(&currentwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SethWnd(newwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(collecting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDrawingAttributes(::core::mem::transmute_copy(&currentattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DefaultDrawingAttributes(&*(&newattributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Renderer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Renderer(::core::mem::transmute_copy(&currentinkrenderer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Renderer(&*(&newinkrenderer as *const <IInkRenderer as ::windows::core::Abi>::Abi as *const <IInkRenderer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Ink(&*(&newink as *const <IInkDisp as ::windows::core::Abi>::Abi as *const <IInkDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRedraw(::core::mem::transmute_copy(&autoredraw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoRedraw(autoredraw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectingInk(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCollectionMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicRendering(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDynamicRendering(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPacketDescription(::core::mem::transmute_copy(&packetguids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredPacketDescription(&*(&packetguids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseIcon(::core::mem::transmute_copy(&mouseicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_MouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MousePointer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MousePointer(::core::mem::transmute_copy(&mousepointer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMousePointer(mousepointer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditingMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditingMode(::core::mem::transmute_copy(&editingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEditingMode(editingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection(::core::mem::transmute_copy(&selection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(&*(&selection as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraserMode(::core::mem::transmute_copy(&erasermode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEraserMode(erasermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserWidth<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraserWidth(::core::mem::transmute_copy(&eraserwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEraserWidth(neweraserwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachMode(::core::mem::transmute_copy(&attachmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachMode(attachmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cursors(::core::mem::transmute_copy(&cursors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginX(::core::mem::transmute_copy(&marginx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginX(marginx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginY<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginY(::core::mem::transmute_copy(&marginy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginY(marginy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tablet<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tablet(::core::mem::transmute_copy(&singletablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportHighContrastInk(::core::mem::transmute_copy(&support)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportHighContrastInk(support) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportHighContrastSelectionUI(support) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestSelection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestSelection(x, y, ::core::mem::transmute_copy(&selarea)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&rect as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGestureStatus(gesture, listen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGestureStatus(gesture, ::core::mem::transmute_copy(&listening)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllTabletsMode(usemouseforinput) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSingleTabletIntegratedMode(&*(&tablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventInterest(eventid, ::core::mem::transmute_copy(&listen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterest(eventid, listen) {
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
            ::windows::core::GetRuntimeClassName::<IInkOverlay>,
            ::windows::core::GetTrustLevel,
            hWnd::<Impl, OFFSET>,
            SethWnd::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            DefaultDrawingAttributes::<Impl, OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, OFFSET>,
            Renderer::<Impl, OFFSET>,
            putref_Renderer::<Impl, OFFSET>,
            Ink::<Impl, OFFSET>,
            putref_Ink::<Impl, OFFSET>,
            AutoRedraw::<Impl, OFFSET>,
            SetAutoRedraw::<Impl, OFFSET>,
            CollectingInk::<Impl, OFFSET>,
            CollectionMode::<Impl, OFFSET>,
            SetCollectionMode::<Impl, OFFSET>,
            DynamicRendering::<Impl, OFFSET>,
            SetDynamicRendering::<Impl, OFFSET>,
            DesiredPacketDescription::<Impl, OFFSET>,
            SetDesiredPacketDescription::<Impl, OFFSET>,
            MouseIcon::<Impl, OFFSET>,
            SetMouseIcon::<Impl, OFFSET>,
            putref_MouseIcon::<Impl, OFFSET>,
            MousePointer::<Impl, OFFSET>,
            SetMousePointer::<Impl, OFFSET>,
            EditingMode::<Impl, OFFSET>,
            SetEditingMode::<Impl, OFFSET>,
            Selection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            EraserMode::<Impl, OFFSET>,
            SetEraserMode::<Impl, OFFSET>,
            EraserWidth::<Impl, OFFSET>,
            SetEraserWidth::<Impl, OFFSET>,
            AttachMode::<Impl, OFFSET>,
            SetAttachMode::<Impl, OFFSET>,
            Cursors::<Impl, OFFSET>,
            MarginX::<Impl, OFFSET>,
            SetMarginX::<Impl, OFFSET>,
            MarginY::<Impl, OFFSET>,
            SetMarginY::<Impl, OFFSET>,
            Tablet::<Impl, OFFSET>,
            SupportHighContrastInk::<Impl, OFFSET>,
            SetSupportHighContrastInk::<Impl, OFFSET>,
            SupportHighContrastSelectionUI::<Impl, OFFSET>,
            SetSupportHighContrastSelectionUI::<Impl, OFFSET>,
            HitTestSelection::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            SetGestureStatus::<Impl, OFFSET>,
            GetGestureStatus::<Impl, OFFSET>,
            GetWindowInputRectangle::<Impl, OFFSET>,
            SetWindowInputRectangle::<Impl, OFFSET>,
            SetAllTabletsMode::<Impl, OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, OFFSET>,
            GetEventInterest::<Impl, OFFSET>,
            SetEventInterest::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkPictureImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn EditingMode();
    fn SetEditingMode();
    fn Selection();
    fn SetSelection();
    fn EraserMode();
    fn SetEraserMode();
    fn EraserWidth();
    fn SetEraserWidth();
    fn putref_Picture();
    fn SetPicture();
    fn Picture();
    fn SetSizeMode();
    fn SizeMode();
    fn SetBackColor();
    fn BackColor();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SupportHighContrastSelectionUI();
    fn SetSupportHighContrastSelectionUI();
    fn HitTestSelection();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
    fn InkEnabled();
    fn SetInkEnabled();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkPicture {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkPicture";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPictureImpl, const OFFSET: isize>() -> IInkPictureVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hWnd(::core::mem::transmute_copy(&currentwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDrawingAttributes(::core::mem::transmute_copy(&currentattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DefaultDrawingAttributes(&*(&newattributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Renderer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Renderer(::core::mem::transmute_copy(&currentinkrenderer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Renderer(&*(&newinkrenderer as *const <IInkRenderer as ::windows::core::Abi>::Abi as *const <IInkRenderer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Ink(&*(&newink as *const <IInkDisp as ::windows::core::Abi>::Abi as *const <IInkDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRedraw(::core::mem::transmute_copy(&autoredraw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoRedraw(autoredraw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectingInk(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCollectionMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicRendering(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDynamicRendering(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPacketDescription(::core::mem::transmute_copy(&packetguids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredPacketDescription(&*(&packetguids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseIcon(::core::mem::transmute_copy(&mouseicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_MouseIcon(&*(&mouseicon as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MousePointer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MousePointer(::core::mem::transmute_copy(&mousepointer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMousePointer(mousepointer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditingMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditingMode(::core::mem::transmute_copy(&editingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEditingMode(editingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection(::core::mem::transmute_copy(&selection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(&*(&selection as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraserMode(::core::mem::transmute_copy(&erasermode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEraserMode(erasermode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserWidth<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraserWidth(::core::mem::transmute_copy(&eraserwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEraserWidth(neweraserwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Picture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Picture(&*(&ppicture as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPicture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPicture(&*(&ppicture as *const <super::super::System::Ole::IPictureDisp as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IPictureDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Picture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppicture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Picture(::core::mem::transmute_copy(&pppicture)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSizeMode(smnewsizemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeMode(::core::mem::transmute_copy(&smsizemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackColor(newcolor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackColor<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColor(::core::mem::transmute_copy(&pcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cursors(::core::mem::transmute_copy(&cursors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginX(::core::mem::transmute_copy(&marginx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginX(marginx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginY<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginY(::core::mem::transmute_copy(&marginy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMarginY(marginy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tablet<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tablet(::core::mem::transmute_copy(&singletablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportHighContrastInk(::core::mem::transmute_copy(&support)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportHighContrastInk(support) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportHighContrastSelectionUI(support) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestSelection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestSelection(x, y, ::core::mem::transmute_copy(&selarea)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGestureStatus(gesture, listen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGestureStatus(gesture, ::core::mem::transmute_copy(&listening)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowInputRectangle(&*(&windowinputrectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllTabletsMode(usemouseforinput) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSingleTabletIntegratedMode(&*(&tablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventInterest(eventid, ::core::mem::transmute_copy(&listen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterest(eventid, listen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkEnabled(::core::mem::transmute_copy(&collecting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInkEnabled(collecting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pbool)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbool: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(vbool) {
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
            ::windows::core::GetRuntimeClassName::<IInkPicture>,
            ::windows::core::GetTrustLevel,
            hWnd::<Impl, OFFSET>,
            DefaultDrawingAttributes::<Impl, OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, OFFSET>,
            Renderer::<Impl, OFFSET>,
            putref_Renderer::<Impl, OFFSET>,
            Ink::<Impl, OFFSET>,
            putref_Ink::<Impl, OFFSET>,
            AutoRedraw::<Impl, OFFSET>,
            SetAutoRedraw::<Impl, OFFSET>,
            CollectingInk::<Impl, OFFSET>,
            CollectionMode::<Impl, OFFSET>,
            SetCollectionMode::<Impl, OFFSET>,
            DynamicRendering::<Impl, OFFSET>,
            SetDynamicRendering::<Impl, OFFSET>,
            DesiredPacketDescription::<Impl, OFFSET>,
            SetDesiredPacketDescription::<Impl, OFFSET>,
            MouseIcon::<Impl, OFFSET>,
            SetMouseIcon::<Impl, OFFSET>,
            putref_MouseIcon::<Impl, OFFSET>,
            MousePointer::<Impl, OFFSET>,
            SetMousePointer::<Impl, OFFSET>,
            EditingMode::<Impl, OFFSET>,
            SetEditingMode::<Impl, OFFSET>,
            Selection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            EraserMode::<Impl, OFFSET>,
            SetEraserMode::<Impl, OFFSET>,
            EraserWidth::<Impl, OFFSET>,
            SetEraserWidth::<Impl, OFFSET>,
            putref_Picture::<Impl, OFFSET>,
            SetPicture::<Impl, OFFSET>,
            Picture::<Impl, OFFSET>,
            SetSizeMode::<Impl, OFFSET>,
            SizeMode::<Impl, OFFSET>,
            SetBackColor::<Impl, OFFSET>,
            BackColor::<Impl, OFFSET>,
            Cursors::<Impl, OFFSET>,
            MarginX::<Impl, OFFSET>,
            SetMarginX::<Impl, OFFSET>,
            MarginY::<Impl, OFFSET>,
            SetMarginY::<Impl, OFFSET>,
            Tablet::<Impl, OFFSET>,
            SupportHighContrastInk::<Impl, OFFSET>,
            SetSupportHighContrastInk::<Impl, OFFSET>,
            SupportHighContrastSelectionUI::<Impl, OFFSET>,
            SetSupportHighContrastSelectionUI::<Impl, OFFSET>,
            HitTestSelection::<Impl, OFFSET>,
            SetGestureStatus::<Impl, OFFSET>,
            GetGestureStatus::<Impl, OFFSET>,
            GetWindowInputRectangle::<Impl, OFFSET>,
            SetWindowInputRectangle::<Impl, OFFSET>,
            SetAllTabletsMode::<Impl, OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, OFFSET>,
            GetEventInterest::<Impl, OFFSET>,
            SetEventInterest::<Impl, OFFSET>,
            InkEnabled::<Impl, OFFSET>,
            SetInkEnabled::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternateImpl: Sized + IDispatchImpl {
    fn String();
    fn Confidence();
    fn Baseline();
    fn Midline();
    fn Ascender();
    fn Descender();
    fn LineNumber();
    fn Strokes();
    fn LineAlternates();
    fn ConfidenceAlternates();
    fn GetStrokesFromStrokeRanges();
    fn GetStrokesFromTextRange();
    fn GetTextRangeFromStrokes();
    fn AlternatesWithConstantPropertyValues();
    fn GetPropertyValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognitionAlternate {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognitionAlternate";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionAlternateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>() -> IInkRecognitionAlternateVtbl {
        unsafe extern "system" fn String<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).String(::core::mem::transmute_copy(&recostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence(::core::mem::transmute_copy(&confidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Baseline(::core::mem::transmute_copy(&baseline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Midline<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Midline(::core::mem::transmute_copy(&midline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascender<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ascender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ascender(::core::mem::transmute_copy(&ascender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descender<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descender(::core::mem::transmute_copy(&descender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineNumber(::core::mem::transmute_copy(&linenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineAlternates<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineAlternates(::core::mem::transmute_copy(&linealternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfidenceAlternates<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidencealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfidenceAlternates(::core::mem::transmute_copy(&confidencealternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromStrokeRanges<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, getstrokesfromstrokeranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokesFromStrokeRanges(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&getstrokesfromstrokeranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromTextRange<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokesFromTextRange(selectionstart, selectionlength, ::core::mem::transmute_copy(&getstrokesfromtextrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextRangeFromStrokes<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextRangeFromStrokes(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), selectionstart, selectionlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternatesWithConstantPropertyValues<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternatesWithConstantPropertyValues(&*(&propertytype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&alternateswithconstantpropertyvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(&*(&propertytype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyvalue)) {
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
            ::windows::core::GetRuntimeClassName::<IInkRecognitionAlternate>,
            ::windows::core::GetTrustLevel,
            String::<Impl, OFFSET>,
            Confidence::<Impl, OFFSET>,
            Baseline::<Impl, OFFSET>,
            Midline::<Impl, OFFSET>,
            Ascender::<Impl, OFFSET>,
            Descender::<Impl, OFFSET>,
            LineNumber::<Impl, OFFSET>,
            Strokes::<Impl, OFFSET>,
            LineAlternates::<Impl, OFFSET>,
            ConfidenceAlternates::<Impl, OFFSET>,
            GetStrokesFromStrokeRanges::<Impl, OFFSET>,
            GetStrokesFromTextRange::<Impl, OFFSET>,
            GetTextRangeFromStrokes::<Impl, OFFSET>,
            AlternatesWithConstantPropertyValues::<Impl, OFFSET>,
            GetPropertyValue::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternatesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Strokes();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognitionAlternates {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognitionAlternates";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>() -> IInkRecognitionAlternatesVtbl {
        unsafe extern "system" fn Count<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecoalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&inkrecoalternate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkRecognitionAlternates>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Strokes::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionResultImpl: Sized + IDispatchImpl {
    fn TopString();
    fn TopAlternate();
    fn TopConfidence();
    fn Strokes();
    fn AlternatesFromSelection();
    fn ModifyTopAlternate();
    fn SetResultOnStrokes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognitionResult {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognitionResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResultImpl, const OFFSET: isize>() -> IInkRecognitionResultVtbl {
        unsafe extern "system" fn TopString<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopString(::core::mem::transmute_copy(&topstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopAlternate<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopAlternate(::core::mem::transmute_copy(&topalternate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopConfidence<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopConfidence(::core::mem::transmute_copy(&topconfidence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternatesFromSelection<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternatesFromSelection(selectionstart, selectionlength, maximumalternates, ::core::mem::transmute_copy(&alternatesfromselection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTopAlternate<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alternate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTopAlternate(&*(&alternate as *const <IInkRecognitionAlternate as ::windows::core::Abi>::Abi as *const <IInkRecognitionAlternate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResultOnStrokes<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResultOnStrokes() {
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
            ::windows::core::GetRuntimeClassName::<IInkRecognitionResult>,
            ::windows::core::GetTrustLevel,
            TopString::<Impl, OFFSET>,
            TopAlternate::<Impl, OFFSET>,
            TopConfidence::<Impl, OFFSET>,
            Strokes::<Impl, OFFSET>,
            AlternatesFromSelection::<Impl, OFFSET>,
            ModifyTopAlternate::<Impl, OFFSET>,
            SetResultOnStrokes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Capabilities();
    fn Languages();
    fn SupportedProperties();
    fn PreferredPacketDescription();
    fn CreateRecognizerContext();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizer {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizer";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerImpl, const OFFSET: isize>() -> IInkRecognizerVtbl {
        unsafe extern "system" fn Name<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vendor(::core::mem::transmute_copy(&vendor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities(::core::mem::transmute_copy(&capabilitiesflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages(::core::mem::transmute_copy(&languages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProperties<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedproperties: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedProperties(::core::mem::transmute_copy(&supportedproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPacketDescription<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredpacketdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredPacketDescription(::core::mem::transmute_copy(&preferredpacketdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecognizerContext<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRecognizerContext(::core::mem::transmute_copy(&context)) {
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
            ::windows::core::GetRuntimeClassName::<IInkRecognizer>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            Vendor::<Impl, OFFSET>,
            Capabilities::<Impl, OFFSET>,
            Languages::<Impl, OFFSET>,
            SupportedProperties::<Impl, OFFSET>,
            PreferredPacketDescription::<Impl, OFFSET>,
            CreateRecognizerContext::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizer2Impl: Sized + IDispatchImpl {
    fn Id();
    fn UnicodeRanges();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizer2 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizer2";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer2Impl, const OFFSET: isize>() -> IInkRecognizer2Vtbl {
        unsafe extern "system" fn Id<Impl: IInkRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeRanges<Impl: IInkRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeRanges(::core::mem::transmute_copy(&unicoderanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkRecognizer2>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, UnicodeRanges::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContextImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn putref_Strokes();
    fn CharacterAutoCompletionMode();
    fn SetCharacterAutoCompletionMode();
    fn Factoid();
    fn SetFactoid();
    fn Guide();
    fn putref_Guide();
    fn PrefixText();
    fn SetPrefixText();
    fn SuffixText();
    fn SetSuffixText();
    fn RecognitionFlags();
    fn SetRecognitionFlags();
    fn WordList();
    fn putref_WordList();
    fn Recognizer();
    fn Recognize();
    fn StopBackgroundRecognition();
    fn EndInkInput();
    fn BackgroundRecognize();
    fn BackgroundRecognizeWithAlternates();
    fn Clone();
    fn IsStringSupported();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizerContext {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizerContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContextImpl, const OFFSET: isize>() -> IInkRecognizerContextVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strokes(::core::mem::transmute_copy(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Strokes(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterAutoCompletionMode<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterAutoCompletionMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterAutoCompletionMode<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCharacterAutoCompletionMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Factoid(::core::mem::transmute_copy(&factoid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFactoid(&*(&factoid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guide<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guide(::core::mem::transmute_copy(&recognizerguide)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Guide<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Guide(&*(&recognizerguide as *const <IInkRecognizerGuide as ::windows::core::Abi>::Abi as *const <IInkRecognizerGuide as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefixText(::core::mem::transmute_copy(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrefixText(&*(&prefix as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuffixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuffixText(::core::mem::transmute_copy(&suffix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuffixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSuffixText(&*(&suffix as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionFlags<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: *mut InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognitionFlags(::core::mem::transmute_copy(&modes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionFlags<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecognitionFlags(modes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WordList<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WordList(::core::mem::transmute_copy(&wordlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_WordList<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_WordList(&*(&wordlist as *const <IInkWordList as ::windows::core::Abi>::Abi as *const <IInkWordList as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognizer<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer(::core::mem::transmute_copy(&recognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognize(recognitionstatus, ::core::mem::transmute_copy(&recognitionresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBackgroundRecognition<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopBackgroundRecognition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInkInput<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndInkInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundRecognize<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundRecognize(&*(&customdata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundRecognizeWithAlternates<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundRecognizeWithAlternates(&*(&customdata as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&recocontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStringSupported<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStringSupported(&*(&string as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
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
            ::windows::core::GetRuntimeClassName::<IInkRecognizerContext>,
            ::windows::core::GetTrustLevel,
            Strokes::<Impl, OFFSET>,
            putref_Strokes::<Impl, OFFSET>,
            CharacterAutoCompletionMode::<Impl, OFFSET>,
            SetCharacterAutoCompletionMode::<Impl, OFFSET>,
            Factoid::<Impl, OFFSET>,
            SetFactoid::<Impl, OFFSET>,
            Guide::<Impl, OFFSET>,
            putref_Guide::<Impl, OFFSET>,
            PrefixText::<Impl, OFFSET>,
            SetPrefixText::<Impl, OFFSET>,
            SuffixText::<Impl, OFFSET>,
            SetSuffixText::<Impl, OFFSET>,
            RecognitionFlags::<Impl, OFFSET>,
            SetRecognitionFlags::<Impl, OFFSET>,
            WordList::<Impl, OFFSET>,
            putref_WordList::<Impl, OFFSET>,
            Recognizer::<Impl, OFFSET>,
            Recognize::<Impl, OFFSET>,
            StopBackgroundRecognition::<Impl, OFFSET>,
            EndInkInput::<Impl, OFFSET>,
            BackgroundRecognize::<Impl, OFFSET>,
            BackgroundRecognizeWithAlternates::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            IsStringSupported::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContext2Impl: Sized + IDispatchImpl {
    fn EnabledUnicodeRanges();
    fn SetEnabledUnicodeRanges();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizerContext2 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizerContext2";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext2Impl, const OFFSET: isize>() -> IInkRecognizerContext2Vtbl {
        unsafe extern "system" fn EnabledUnicodeRanges<Impl: IInkRecognizerContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnabledUnicodeRanges(::core::mem::transmute_copy(&unicoderanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledUnicodeRanges<Impl: IInkRecognizerContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabledUnicodeRanges(&*(&unicoderanges as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkRecognizerContext2>, ::windows::core::GetTrustLevel, EnabledUnicodeRanges::<Impl, OFFSET>, SetEnabledUnicodeRanges::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerGuideImpl: Sized + IDispatchImpl {
    fn WritingBox();
    fn SetWritingBox();
    fn DrawnBox();
    fn SetDrawnBox();
    fn Rows();
    fn SetRows();
    fn Columns();
    fn SetColumns();
    fn Midline();
    fn SetMidline();
    fn GuideData();
    fn SetGuideData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizerGuide {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizerGuide";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerGuideVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuideImpl, const OFFSET: isize>() -> IInkRecognizerGuideVtbl {
        unsafe extern "system" fn WritingBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WritingBox(::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWritingBox(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawnBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawnBox(::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawnBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDrawnBox(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rows<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rows(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRows<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRows(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Columns<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Columns(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumns<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumns(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Midline<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Midline(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMidline<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMidline(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GuideData<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoguide: *mut InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GuideData(::core::mem::transmute_copy(&precoguide)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuideData<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoguide: InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGuideData(&*(&recoguide as *const <InkRecoGuide as ::windows::core::Abi>::Abi as *const <InkRecoGuide as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IInkRecognizerGuide>,
            ::windows::core::GetTrustLevel,
            WritingBox::<Impl, OFFSET>,
            SetWritingBox::<Impl, OFFSET>,
            DrawnBox::<Impl, OFFSET>,
            SetDrawnBox::<Impl, OFFSET>,
            Rows::<Impl, OFFSET>,
            SetRows::<Impl, OFFSET>,
            Columns::<Impl, OFFSET>,
            SetColumns::<Impl, OFFSET>,
            Midline::<Impl, OFFSET>,
            SetMidline::<Impl, OFFSET>,
            GuideData::<Impl, OFFSET>,
            SetGuideData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn GetDefaultRecognizer();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRecognizers {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRecognizers";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizersImpl, const OFFSET: isize>() -> IInkRecognizersVtbl {
        unsafe extern "system" fn Count<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultRecognizer<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: i32, defaultrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultRecognizer(lcid, ::core::mem::transmute_copy(&defaultrecognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&inkrecognizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkRecognizers>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, GetDefaultRecognizer::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRectangleImpl: Sized + IDispatchImpl {
    fn Top();
    fn SetTop();
    fn Left();
    fn SetLeft();
    fn Bottom();
    fn SetBottom();
    fn Right();
    fn SetRight();
    fn Data();
    fn SetData();
    fn GetRectangle();
    fn SetRectangle();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRectangle {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRectangle";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRectangleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangleImpl, const OFFSET: isize>() -> IInkRectangleVtbl {
        unsafe extern "system" fn Top<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Top(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTop(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Left(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLeft(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bottom<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bottom(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBottom(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Right<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Right(::core::mem::transmute_copy(&units)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRight(units) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&rect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&rect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectangle<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRectangle(::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&right)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectangle<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRectangle(top, left, bottom, right) {
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
            ::windows::core::GetRuntimeClassName::<IInkRectangle>,
            ::windows::core::GetTrustLevel,
            Top::<Impl, OFFSET>,
            SetTop::<Impl, OFFSET>,
            Left::<Impl, OFFSET>,
            SetLeft::<Impl, OFFSET>,
            Bottom::<Impl, OFFSET>,
            SetBottom::<Impl, OFFSET>,
            Right::<Impl, OFFSET>,
            SetRight::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            GetRectangle::<Impl, OFFSET>,
            SetRectangle::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRendererImpl: Sized + IDispatchImpl {
    fn GetViewTransform();
    fn SetViewTransform();
    fn GetObjectTransform();
    fn SetObjectTransform();
    fn Draw();
    fn DrawStroke();
    fn PixelToInkSpace();
    fn InkSpaceToPixel();
    fn PixelToInkSpaceFromPoints();
    fn InkSpaceToPixelFromPoints();
    fn Measure();
    fn MeasureStroke();
    fn Move();
    fn Rotate();
    fn ScaleTransform();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkRenderer {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkRenderer";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRendererImpl, const OFFSET: isize>() -> IInkRendererVtbl {
        unsafe extern "system" fn GetViewTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewTransform(&*(&viewtransform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetViewTransform(&*(&viewtransform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectTransform(&*(&objecttransform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectTransform(&*(&objecttransform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(hdc, &*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawStroke<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawStroke(hdc, &*(&stroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType), &*(&drawingattributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelToInkSpace<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelToInkSpace(hdc, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkSpaceToPixel<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkSpaceToPixel(hdcdisplay, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelToInkSpaceFromPoints<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelToInkSpaceFromPoints(hdc, &*(&points as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkSpaceToPixelFromPoints<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkSpaceToPixelFromPoints(hdc, &*(&points as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Measure<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Measure(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureStroke<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasureStroke(&*(&stroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType), &*(&drawingattributes as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(horizontalcomponent, verticalcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleTransform(horizontalmultiplier, verticalmultiplier, applyonpenwidth) {
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
            ::windows::core::GetRuntimeClassName::<IInkRenderer>,
            ::windows::core::GetTrustLevel,
            GetViewTransform::<Impl, OFFSET>,
            SetViewTransform::<Impl, OFFSET>,
            GetObjectTransform::<Impl, OFFSET>,
            SetObjectTransform::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            DrawStroke::<Impl, OFFSET>,
            PixelToInkSpace::<Impl, OFFSET>,
            InkSpaceToPixel::<Impl, OFFSET>,
            PixelToInkSpaceFromPoints::<Impl, OFFSET>,
            InkSpaceToPixelFromPoints::<Impl, OFFSET>,
            Measure::<Impl, OFFSET>,
            MeasureStroke::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Rotate::<Impl, OFFSET>,
            ScaleTransform::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokeDispImpl: Sized + IDispatchImpl {
    fn ID();
    fn BezierPoints();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn Ink();
    fn ExtendedProperties();
    fn PolylineCusps();
    fn BezierCusps();
    fn SelfIntersections();
    fn PacketCount();
    fn PacketSize();
    fn PacketDescription();
    fn Deleted();
    fn GetBoundingBox();
    fn FindIntersections();
    fn GetRectangleIntersections();
    fn Clip();
    fn HitTestCircle();
    fn NearestPoint();
    fn Split();
    fn GetPacketDescriptionPropertyMetrics();
    fn GetPoints();
    fn SetPoints();
    fn GetPacketData();
    fn GetPacketValuesByProperty();
    fn SetPacketValuesByProperty();
    fn GetFlattenedBezierPoints();
    fn Transform();
    fn ScaleToRectangle();
    fn Move();
    fn Rotate();
    fn Shear();
    fn ScaleTransform();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkStrokeDisp {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkStrokeDisp";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkStrokeDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDispImpl, const OFFSET: isize>() -> IInkStrokeDispVtbl {
        unsafe extern "system" fn ID<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BezierPoints(::core::mem::transmute_copy(&points)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawingAttributes(::core::mem::transmute_copy(&drawattrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_DrawingAttributes(&*(&drawattrs as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties(::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolylineCusps<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolylineCusps(::core::mem::transmute_copy(&cusps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierCusps<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BezierCusps(::core::mem::transmute_copy(&cusps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelfIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelfIntersections(::core::mem::transmute_copy(&intersections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketCount<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PacketCount(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketSize<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PacketSize(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketDescription<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PacketDescription(::core::mem::transmute_copy(&packetdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deleted<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deleted(::core::mem::transmute_copy(&deleted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingBox(boundingboxmode, ::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindIntersections(&*(&strokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&intersections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectangleIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRectangleIntersections(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&intersections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestCircle<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTestCircle(x, y, radius, ::core::mem::transmute_copy(&intersects)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearestPoint<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearestPoint(x, y, distance, ::core::mem::transmute_copy(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, splitat: f32, newstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Split(splitat, ::core::mem::transmute_copy(&newstroke)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketDescriptionPropertyMetrics<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPacketDescriptionPropertyMetrics(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPoints(index, count, ::core::mem::transmute_copy(&points)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPoints(&*(&points as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), index, count, ::core::mem::transmute_copy(&numberofpointsset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketData<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, packetdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPacketData(index, count, ::core::mem::transmute_copy(&packetdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketValuesByProperty<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPacketValuesByProperty(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), index, count, ::core::mem::transmute_copy(&packetvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPacketValuesByProperty<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPacketValuesByProperty(&*(&bstrpropertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&packetvalues as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), index, count, ::core::mem::transmute_copy(&numberofpacketsset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlattenedBezierPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlattenedBezierPoints(fittingerror, ::core::mem::transmute_copy(&flattenedbezierpoints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform(&*(&transform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType), applyonpenwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToRectangle<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleToRectangle(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(horizontalcomponent, verticalcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shear<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shear(horizontalmultiplier, verticalmultiplier) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleTransform(horizontalmultiplier, verticalmultiplier) {
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
            ::windows::core::GetRuntimeClassName::<IInkStrokeDisp>,
            ::windows::core::GetTrustLevel,
            ID::<Impl, OFFSET>,
            BezierPoints::<Impl, OFFSET>,
            DrawingAttributes::<Impl, OFFSET>,
            putref_DrawingAttributes::<Impl, OFFSET>,
            Ink::<Impl, OFFSET>,
            ExtendedProperties::<Impl, OFFSET>,
            PolylineCusps::<Impl, OFFSET>,
            BezierCusps::<Impl, OFFSET>,
            SelfIntersections::<Impl, OFFSET>,
            PacketCount::<Impl, OFFSET>,
            PacketSize::<Impl, OFFSET>,
            PacketDescription::<Impl, OFFSET>,
            Deleted::<Impl, OFFSET>,
            GetBoundingBox::<Impl, OFFSET>,
            FindIntersections::<Impl, OFFSET>,
            GetRectangleIntersections::<Impl, OFFSET>,
            Clip::<Impl, OFFSET>,
            HitTestCircle::<Impl, OFFSET>,
            NearestPoint::<Impl, OFFSET>,
            Split::<Impl, OFFSET>,
            GetPacketDescriptionPropertyMetrics::<Impl, OFFSET>,
            GetPoints::<Impl, OFFSET>,
            SetPoints::<Impl, OFFSET>,
            GetPacketData::<Impl, OFFSET>,
            GetPacketValuesByProperty::<Impl, OFFSET>,
            SetPacketValuesByProperty::<Impl, OFFSET>,
            GetFlattenedBezierPoints::<Impl, OFFSET>,
            Transform::<Impl, OFFSET>,
            ScaleToRectangle::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Rotate::<Impl, OFFSET>,
            Shear::<Impl, OFFSET>,
            ScaleTransform::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Ink();
    fn RecognitionResult();
    fn ToString();
    fn Item();
    fn Add();
    fn AddStrokes();
    fn Remove();
    fn RemoveStrokes();
    fn ModifyDrawingAttributes();
    fn GetBoundingBox();
    fn Transform();
    fn ScaleToRectangle();
    fn Move();
    fn Rotate();
    fn Shear();
    fn ScaleTransform();
    fn Clip();
    fn RemoveRecognitionResult();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkStrokes {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkStrokes";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkStrokesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesImpl, const OFFSET: isize>() -> IInkStrokesVtbl {
        unsafe extern "system" fn Count<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionResult<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognitionResult(::core::mem::transmute_copy(&recognitionresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToString<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToString(::core::mem::transmute_copy(&tostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&stroke)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&inkstroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStrokes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStrokes(&*(&inkstrokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&inkstroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStrokes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStrokes(&*(&inkstrokes as *const <IInkStrokes as ::windows::core::Abi>::Abi as *const <IInkStrokes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyDrawingAttributes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyDrawingAttributes(&*(&drawattrs as *const <IInkDrawingAttributes as ::windows::core::Abi>::Abi as *const <IInkDrawingAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingBox(boundingboxmode, ::core::mem::transmute_copy(&boundingbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform(&*(&transform as *const <IInkTransform as ::windows::core::Abi>::Abi as *const <IInkTransform as ::windows::core::DefaultType>::DefaultType), applyonpenwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToRectangle<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleToRectangle(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(horizontalcomponent, verticalcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shear<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shear(horizontalmultiplier, verticalmultiplier) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleTransform(horizontalmultiplier, verticalmultiplier) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip(&*(&rectangle as *const <IInkRectangle as ::windows::core::Abi>::Abi as *const <IInkRectangle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionResult<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveRecognitionResult() {
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
            ::windows::core::GetRuntimeClassName::<IInkStrokes>,
            ::windows::core::GetTrustLevel,
            Count::<Impl, OFFSET>,
            _NewEnum::<Impl, OFFSET>,
            Ink::<Impl, OFFSET>,
            RecognitionResult::<Impl, OFFSET>,
            ToString::<Impl, OFFSET>,
            Item::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            AddStrokes::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            RemoveStrokes::<Impl, OFFSET>,
            ModifyDrawingAttributes::<Impl, OFFSET>,
            GetBoundingBox::<Impl, OFFSET>,
            Transform::<Impl, OFFSET>,
            ScaleToRectangle::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Rotate::<Impl, OFFSET>,
            Shear::<Impl, OFFSET>,
            ScaleTransform::<Impl, OFFSET>,
            Clip::<Impl, OFFSET>,
            RemoveRecognitionResult::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTabletImpl: Sized + IDispatchImpl {
    fn Name();
    fn PlugAndPlayId();
    fn MaximumInputRectangle();
    fn HardwareCapabilities();
    fn IsPacketPropertySupported();
    fn GetPropertyMetrics();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkTablet {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkTablet";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkTabletVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTabletImpl, const OFFSET: isize>() -> IInkTabletVtbl {
        unsafe extern "system" fn Name<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugAndPlayId<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugAndPlayId(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumInputRectangle<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumInputRectangle(::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareCapabilities<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareCapabilities(::core::mem::transmute_copy(&capabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPacketPropertySupported(&*(&packetpropertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyMetrics<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyMetrics(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkTablet>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, PlugAndPlayId::<Impl, OFFSET>, MaximumInputRectangle::<Impl, OFFSET>, HardwareCapabilities::<Impl, OFFSET>, IsPacketPropertySupported::<Impl, OFFSET>, GetPropertyMetrics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet2Impl: Sized + IDispatchImpl {
    fn DeviceKind();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkTablet2 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkTablet2";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet2Impl, const OFFSET: isize>() -> IInkTablet2Vtbl {
        unsafe extern "system" fn DeviceKind<Impl: IInkTablet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: *mut TabletDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceKind(::core::mem::transmute_copy(&kind)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkTablet2>, ::windows::core::GetTrustLevel, DeviceKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet3Impl: Sized + IDispatchImpl {
    fn IsMultiTouch();
    fn MaximumCursors();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkTablet3 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkTablet3";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet3Impl, const OFFSET: isize>() -> IInkTablet3Vtbl {
        unsafe extern "system" fn IsMultiTouch<Impl: IInkTablet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismultitouch: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMultiTouch(::core::mem::transmute_copy(&pismultitouch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumCursors<Impl: IInkTablet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaximumcursors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumCursors(::core::mem::transmute_copy(&pmaximumcursors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkTablet3>, ::windows::core::GetTrustLevel, IsMultiTouch::<Impl, OFFSET>, MaximumCursors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTabletsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn DefaultTablet();
    fn Item();
    fn IsPacketPropertySupported();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkTablets {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkTablets";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkTabletsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTabletsImpl, const OFFSET: isize>() -> IInkTabletsVtbl {
        unsafe extern "system" fn Count<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&_newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTablet<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultTablet(::core::mem::transmute_copy(&defaulttablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&tablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPacketPropertySupported(&*(&packetpropertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&supported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkTablets>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, DefaultTablet::<Impl, OFFSET>, Item::<Impl, OFFSET>, IsPacketPropertySupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTransformImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Translate();
    fn Rotate();
    fn Reflect();
    fn Shear();
    fn ScaleTransform();
    fn GetTransform();
    fn SetTransform();
    fn eM11();
    fn SeteM11();
    fn eM12();
    fn SeteM12();
    fn eM21();
    fn SeteM21();
    fn eM22();
    fn SeteM22();
    fn eDx();
    fn SeteDx();
    fn eDy();
    fn SeteDy();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkTransform {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkTransform";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransformImpl, const OFFSET: isize>() -> IInkTransformVtbl {
        unsafe extern "system" fn Reset<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Translate<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Translate(horizontalcomponent, verticalcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reflect<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontally: i16, vertically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reflect(horizontally, vertically) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shear<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shear(horizontalcomponent, verticalcomponent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleTransform(horizontalmultiplier, verticalmultiplier) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform(::core::mem::transmute_copy(&em11), ::core::mem::transmute_copy(&em12), ::core::mem::transmute_copy(&em21), ::core::mem::transmute_copy(&em22), ::core::mem::transmute_copy(&edx), ::core::mem::transmute_copy(&edy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransform(em11, em12, em21, em22, edx, edy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eM11<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eM11(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM11<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteM11(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eM12<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eM12(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM12<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteM12(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eM21<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eM21(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM21<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteM21(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eM22<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eM22(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM22<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteM22(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eDx<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eDx(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDx<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteDx(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eDy<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eDy(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDy<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeteDy(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&xform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&xform as *const <super::super::Graphics::Gdi::XFORM as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::XFORM as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IInkTransform>,
            ::windows::core::GetTrustLevel,
            Reset::<Impl, OFFSET>,
            Translate::<Impl, OFFSET>,
            Rotate::<Impl, OFFSET>,
            Reflect::<Impl, OFFSET>,
            Shear::<Impl, OFFSET>,
            ScaleTransform::<Impl, OFFSET>,
            GetTransform::<Impl, OFFSET>,
            SetTransform::<Impl, OFFSET>,
            eM11::<Impl, OFFSET>,
            SeteM11::<Impl, OFFSET>,
            eM12::<Impl, OFFSET>,
            SeteM12::<Impl, OFFSET>,
            eM21::<Impl, OFFSET>,
            SeteM21::<Impl, OFFSET>,
            eM22::<Impl, OFFSET>,
            SeteM22::<Impl, OFFSET>,
            eDx::<Impl, OFFSET>,
            SeteDx::<Impl, OFFSET>,
            eDy::<Impl, OFFSET>,
            SeteDy::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordListImpl: Sized + IDispatchImpl {
    fn AddWord();
    fn RemoveWord();
    fn Merge();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkWordList {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkWordList";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkWordListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordListImpl, const OFFSET: isize>() -> IInkWordListVtbl {
        unsafe extern "system" fn AddWord<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWord(&*(&newword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWord<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, removeword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveWord(&*(&removeword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergewordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Merge(&*(&mergewordlist as *const <IInkWordList as ::windows::core::Abi>::Abi as *const <IInkWordList as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkWordList>, ::windows::core::GetTrustLevel, AddWord::<Impl, OFFSET>, RemoveWord::<Impl, OFFSET>, Merge::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordList2Impl: Sized + IDispatchImpl {
    fn AddWords();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInkWordList2 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInkWordList2";
}
#[cfg(feature = "Win32_System_Com")]
impl IInkWordList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList2Impl, const OFFSET: isize>() -> IInkWordList2Vtbl {
        unsafe extern "system" fn AddWords<Impl: IInkWordList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwords: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWords(&*(&newwords as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkWordList2>, ::windows::core::GetTrustLevel, AddWords::<Impl, OFFSET>)
    }
}
pub trait IInputPanelWindowHandleImpl: Sized {
    fn AttachedEditWindow32();
    fn SetAttachedEditWindow32();
    fn AttachedEditWindow64();
    fn SetAttachedEditWindow64();
}
impl ::windows::core::RuntimeName for IInputPanelWindowHandle {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IInputPanelWindowHandle";
}
impl IInputPanelWindowHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>() -> IInputPanelWindowHandleVtbl {
        unsafe extern "system" fn AttachedEditWindow32<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachedEditWindow32(::core::mem::transmute_copy(&attachededitwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachedEditWindow32(attachededitwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachedEditWindow64<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachedEditWindow64(::core::mem::transmute_copy(&attachededitwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachedEditWindow64(attachededitwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInputPanelWindowHandle>, ::windows::core::GetTrustLevel, AttachedEditWindow32::<Impl, OFFSET>, SetAttachedEditWindow32::<Impl, OFFSET>, AttachedEditWindow64::<Impl, OFFSET>, SetAttachedEditWindow64::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMathInputControlImpl: Sized + IDispatchImpl {
    fn Show();
    fn Hide();
    fn IsVisible();
    fn GetPosition();
    fn SetPosition();
    fn Clear();
    fn SetCustomPaint();
    fn SetCaptionText();
    fn LoadInk();
    fn SetOwnerWindow();
    fn EnableExtendedButtons();
    fn GetPreviewHeight();
    fn SetPreviewHeight();
    fn EnableAutoGrow();
    fn AddFunctionName();
    fn RemoveFunctionName();
    fn GetHoverIcon();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMathInputControl {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IMathInputControl";
}
#[cfg(feature = "Win32_System_Com")]
impl IMathInputControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControlImpl, const OFFSET: isize>() -> IMathInputControlVtbl {
        unsafe extern "system" fn Show<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hide<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshown: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible(::core::mem::transmute_copy(&pvbshown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPosition(left, top, right, bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomPaint<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: i32, paint: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCustomPaint(element, paint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaptionText<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captiontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCaptionText(&*(&captiontext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadInk<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadInk(&*(&ink as *const <IInkDisp as ::windows::core::Abi>::Abi as *const <IInkDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerWindow<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownerwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOwnerWindow(ownerwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableExtendedButtons<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableExtendedButtons(extended) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewHeight<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewHeight(::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewHeight<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreviewHeight(height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAutoGrow<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autogrow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAutoGrow(autogrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFunctionName<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFunctionName(&*(&functionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFunctionName<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFunctionName(&*(&functionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHoverIcon<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hoverimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHoverIcon(::core::mem::transmute_copy(&hoverimage)) {
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
            ::windows::core::GetRuntimeClassName::<IMathInputControl>,
            ::windows::core::GetTrustLevel,
            Show::<Impl, OFFSET>,
            Hide::<Impl, OFFSET>,
            IsVisible::<Impl, OFFSET>,
            GetPosition::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            SetCustomPaint::<Impl, OFFSET>,
            SetCaptionText::<Impl, OFFSET>,
            LoadInk::<Impl, OFFSET>,
            SetOwnerWindow::<Impl, OFFSET>,
            EnableExtendedButtons::<Impl, OFFSET>,
            GetPreviewHeight::<Impl, OFFSET>,
            SetPreviewHeight::<Impl, OFFSET>,
            EnableAutoGrow::<Impl, OFFSET>,
            AddFunctionName::<Impl, OFFSET>,
            RemoveFunctionName::<Impl, OFFSET>,
            GetHoverIcon::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPenInputPanelImpl: Sized + IDispatchImpl {
    fn Busy();
    fn Factoid();
    fn SetFactoid();
    fn AttachedEditWindow();
    fn SetAttachedEditWindow();
    fn CurrentPanel();
    fn SetCurrentPanel();
    fn DefaultPanel();
    fn SetDefaultPanel();
    fn Visible();
    fn SetVisible();
    fn Top();
    fn Left();
    fn Width();
    fn Height();
    fn VerticalOffset();
    fn SetVerticalOffset();
    fn HorizontalOffset();
    fn SetHorizontalOffset();
    fn AutoShow();
    fn SetAutoShow();
    fn MoveTo();
    fn CommitPendingInput();
    fn Refresh();
    fn EnableTsf();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPenInputPanel {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IPenInputPanel";
}
#[cfg(feature = "Win32_System_Com")]
impl IPenInputPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanelImpl, const OFFSET: isize>() -> IPenInputPanelVtbl {
        unsafe extern "system" fn Busy<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busy: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Busy(::core::mem::transmute_copy(&busy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Factoid(::core::mem::transmute_copy(&factoid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFactoid(&*(&factoid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachedEditWindow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachedEditWindow(attachededitwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPanel(::core::mem::transmute_copy(&currentpanel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentPanel(currentpanel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultPanel(::core::mem::transmute_copy(&pdefaultpanel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultPanel(defaultpanel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible(::core::mem::transmute_copy(&visible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVisible(visible) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Top<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Top(::core::mem::transmute_copy(&top)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Left(::core::mem::transmute_copy(&left)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&width)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset(::core::mem::transmute_copy(&verticaloffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVerticalOffset(verticaloffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset(::core::mem::transmute_copy(&horizontaloffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHorizontalOffset(horizontaloffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoShow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pautoshow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoShow(::core::mem::transmute_copy(&pautoshow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoShow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoshow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoShow(autoshow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveTo<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveTo(left, top) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitPendingInput<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitPendingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTsf<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableTsf(enable) {
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
            ::windows::core::GetRuntimeClassName::<IPenInputPanel>,
            ::windows::core::GetTrustLevel,
            Busy::<Impl, OFFSET>,
            Factoid::<Impl, OFFSET>,
            SetFactoid::<Impl, OFFSET>,
            AttachedEditWindow::<Impl, OFFSET>,
            SetAttachedEditWindow::<Impl, OFFSET>,
            CurrentPanel::<Impl, OFFSET>,
            SetCurrentPanel::<Impl, OFFSET>,
            DefaultPanel::<Impl, OFFSET>,
            SetDefaultPanel::<Impl, OFFSET>,
            Visible::<Impl, OFFSET>,
            SetVisible::<Impl, OFFSET>,
            Top::<Impl, OFFSET>,
            Left::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            VerticalOffset::<Impl, OFFSET>,
            SetVerticalOffset::<Impl, OFFSET>,
            HorizontalOffset::<Impl, OFFSET>,
            SetHorizontalOffset::<Impl, OFFSET>,
            AutoShow::<Impl, OFFSET>,
            SetAutoShow::<Impl, OFFSET>,
            MoveTo::<Impl, OFFSET>,
            CommitPendingInput::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            EnableTsf::<Impl, OFFSET>,
        )
    }
}
pub trait IRealTimeStylusImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn HWND();
    fn SetHWND();
    fn WindowInputRectangle();
    fn SetWindowInputRectangle();
    fn AddStylusSyncPlugin();
    fn RemoveStylusSyncPlugin();
    fn RemoveAllStylusSyncPlugins();
    fn GetStylusSyncPlugin();
    fn GetStylusSyncPluginCount();
    fn AddStylusAsyncPlugin();
    fn RemoveStylusAsyncPlugin();
    fn RemoveAllStylusAsyncPlugins();
    fn GetStylusAsyncPlugin();
    fn GetStylusAsyncPluginCount();
    fn ChildRealTimeStylusPlugin();
    fn putref_ChildRealTimeStylusPlugin();
    fn AddCustomStylusDataToQueue();
    fn ClearStylusQueues();
    fn SetAllTabletsMode();
    fn SetSingleTabletMode();
    fn GetTablet();
    fn GetTabletContextIdFromTablet();
    fn GetTabletFromTabletContextId();
    fn GetAllTabletContextIds();
    fn GetStyluses();
    fn GetStylusForId();
    fn SetDesiredPacketDescription();
    fn GetDesiredPacketDescription();
    fn GetPacketDescriptionData();
}
impl ::windows::core::RuntimeName for IRealTimeStylus {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IRealTimeStylus";
}
impl IRealTimeStylusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusImpl, const OFFSET: isize>() -> IRealTimeStylusVtbl {
        unsafe extern "system" fn Enabled<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pfenable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HWND<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HWND(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHWND(&*(&hwnd as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInputRectangle<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowInputRectangle(::core::mem::transmute_copy(&prcwndinputrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowInputRectangle(&*(&prcwndinputrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStylusSyncPlugin(iindex, &*(&piplugin as *const <IStylusSyncPlugin as ::windows::core::Abi>::Abi as *const <IStylusSyncPlugin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStylusSyncPlugin(iindex, &*(&ppiplugin as *const <IStylusSyncPlugin as ::windows::core::Abi>::Abi as *const <IStylusSyncPlugin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllStylusSyncPlugins<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAllStylusSyncPlugins() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylusSyncPlugin(iindex, ::core::mem::transmute_copy(&ppiplugin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusSyncPluginCount<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylusSyncPluginCount(::core::mem::transmute_copy(&pcplugins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStylusAsyncPlugin(iindex, &*(&piplugin as *const <IStylusAsyncPlugin as ::windows::core::Abi>::Abi as *const <IStylusAsyncPlugin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStylusAsyncPlugin(iindex, &*(&ppiplugin as *const <IStylusAsyncPlugin as ::windows::core::Abi>::Abi as *const <IStylusAsyncPlugin as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllStylusAsyncPlugins<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAllStylusAsyncPlugins() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylusAsyncPlugin(iindex, ::core::mem::transmute_copy(&ppiplugin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusAsyncPluginCount<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylusAsyncPluginCount(::core::mem::transmute_copy(&pcplugins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildRealTimeStylusPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppirts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildRealTimeStylusPlugin(::core::mem::transmute_copy(&ppirts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ChildRealTimeStylusPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ChildRealTimeStylusPlugin(&*(&pirts as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCustomStylusDataToQueue<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sq: StylusQueue, pguidid: &::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddCustomStylusDataToQueue(sq, &*(&pguidid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbdata, pbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStylusQueues<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearStylusQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllTabletsMode(&*(&fusemouseforinput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSingleTabletMode<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSingleTabletMode(&*(&pitablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTablet<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisingletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTablet(::core::mem::transmute_copy(&ppisingletablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletContextIdFromTablet<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr, ptcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTabletContextIdFromTablet(&*(&pitablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletFromTabletContextId<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, ppitablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTabletFromTabletContextId(tcid, ::core::mem::transmute_copy(&ppitablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllTabletContextIds<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllTabletContextIds(pctcidcount, ::core::mem::transmute_copy(&pptcids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyluses<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkcursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyluses(::core::mem::transmute_copy(&ppiinkcursors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusForId<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u32, ppiinkcursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStylusForId(sid, ::core::mem::transmute_copy(&ppiinkcursor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropertyguids: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredPacketDescription(cproperties, &*(&ppropertyguids as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesiredPacketDescription<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesiredPacketDescription(pcproperties, ::core::mem::transmute_copy(&pppropertyguids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketDescriptionData<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPacketDescriptionData(tcid, pfinktodevicescalex, pfinktodevicescaley, pcpacketproperties, ::core::mem::transmute_copy(&pppacketproperties)) {
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
            ::windows::core::GetRuntimeClassName::<IRealTimeStylus>,
            ::windows::core::GetTrustLevel,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            HWND::<Impl, OFFSET>,
            SetHWND::<Impl, OFFSET>,
            WindowInputRectangle::<Impl, OFFSET>,
            SetWindowInputRectangle::<Impl, OFFSET>,
            AddStylusSyncPlugin::<Impl, OFFSET>,
            RemoveStylusSyncPlugin::<Impl, OFFSET>,
            RemoveAllStylusSyncPlugins::<Impl, OFFSET>,
            GetStylusSyncPlugin::<Impl, OFFSET>,
            GetStylusSyncPluginCount::<Impl, OFFSET>,
            AddStylusAsyncPlugin::<Impl, OFFSET>,
            RemoveStylusAsyncPlugin::<Impl, OFFSET>,
            RemoveAllStylusAsyncPlugins::<Impl, OFFSET>,
            GetStylusAsyncPlugin::<Impl, OFFSET>,
            GetStylusAsyncPluginCount::<Impl, OFFSET>,
            ChildRealTimeStylusPlugin::<Impl, OFFSET>,
            putref_ChildRealTimeStylusPlugin::<Impl, OFFSET>,
            AddCustomStylusDataToQueue::<Impl, OFFSET>,
            ClearStylusQueues::<Impl, OFFSET>,
            SetAllTabletsMode::<Impl, OFFSET>,
            SetSingleTabletMode::<Impl, OFFSET>,
            GetTablet::<Impl, OFFSET>,
            GetTabletContextIdFromTablet::<Impl, OFFSET>,
            GetTabletFromTabletContextId::<Impl, OFFSET>,
            GetAllTabletContextIds::<Impl, OFFSET>,
            GetStyluses::<Impl, OFFSET>,
            GetStylusForId::<Impl, OFFSET>,
            SetDesiredPacketDescription::<Impl, OFFSET>,
            GetDesiredPacketDescription::<Impl, OFFSET>,
            GetPacketDescriptionData::<Impl, OFFSET>,
        )
    }
}
pub trait IRealTimeStylus2Impl: Sized {
    fn FlicksEnabled();
    fn SetFlicksEnabled();
}
impl ::windows::core::RuntimeName for IRealTimeStylus2 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IRealTimeStylus2";
}
impl IRealTimeStylus2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus2Impl, const OFFSET: isize>() -> IRealTimeStylus2Vtbl {
        unsafe extern "system" fn FlicksEnabled<Impl: IRealTimeStylus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlicksEnabled(::core::mem::transmute_copy(&pfenable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlicksEnabled<Impl: IRealTimeStylus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlicksEnabled(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRealTimeStylus2>, ::windows::core::GetTrustLevel, FlicksEnabled::<Impl, OFFSET>, SetFlicksEnabled::<Impl, OFFSET>)
    }
}
pub trait IRealTimeStylus3Impl: Sized {
    fn MultiTouchEnabled();
    fn SetMultiTouchEnabled();
}
impl ::windows::core::RuntimeName for IRealTimeStylus3 {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IRealTimeStylus3";
}
impl IRealTimeStylus3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus3Impl, const OFFSET: isize>() -> IRealTimeStylus3Vtbl {
        unsafe extern "system" fn MultiTouchEnabled<Impl: IRealTimeStylus3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiTouchEnabled(::core::mem::transmute_copy(&pfenable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiTouchEnabled<Impl: IRealTimeStylus3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultiTouchEnabled(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRealTimeStylus3>, ::windows::core::GetTrustLevel, MultiTouchEnabled::<Impl, OFFSET>, SetMultiTouchEnabled::<Impl, OFFSET>)
    }
}
pub trait IRealTimeStylusSynchronizationImpl: Sized {
    fn AcquireLock();
    fn ReleaseLock();
}
impl ::windows::core::RuntimeName for IRealTimeStylusSynchronization {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IRealTimeStylusSynchronization";
}
impl IRealTimeStylusSynchronizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusSynchronizationImpl, const OFFSET: isize>() -> IRealTimeStylusSynchronizationVtbl {
        unsafe extern "system" fn AcquireLock<Impl: IRealTimeStylusSynchronizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireLock(lock) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseLock<Impl: IRealTimeStylusSynchronizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseLock(lock) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRealTimeStylusSynchronization>, ::windows::core::GetTrustLevel, AcquireLock::<Impl, OFFSET>, ReleaseLock::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISketchInkImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISketchInk {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ISketchInk";
}
#[cfg(feature = "Win32_System_Com")]
impl ISketchInkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISketchInkImpl, const OFFSET: isize>() -> ISketchInkVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISketchInk>, ::windows::core::GetTrustLevel)
    }
}
pub trait IStrokeBuilderImpl: Sized {
    fn CreateStroke();
    fn BeginStroke();
    fn AppendPackets();
    fn EndStroke();
    fn Ink();
    fn putref_Ink();
}
impl ::windows::core::RuntimeName for IStrokeBuilder {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IStrokeBuilder";
}
impl IStrokeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilderImpl, const OFFSET: isize>() -> IStrokeBuilderVtbl {
        unsafe extern "system" fn CreateStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStroke(cpktbufflength, ppackets, cpacketproperties, &*(&ppacketproperties as *const <PACKET_PROPERTY as ::windows::core::Abi>::Abi as *const <PACKET_PROPERTY as ::windows::core::DefaultType>::DefaultType), finktodevicescalex, finktodevicescaley, &*(&ppiinkstroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginStroke(tcid, sid, ppacket, cpacketproperties, &*(&ppacketproperties as *const <PACKET_PROPERTY as ::windows::core::Abi>::Abi as *const <PACKET_PROPERTY as ::windows::core::DefaultType>::DefaultType), finktodevicescalex, finktodevicescaley, &*(&ppiinkstroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendPackets<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendPackets(tcid, sid, cpktbufflength, ppackets) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut ::windows::core::RawPtr, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndStroke(tcid, sid, &*(&ppiinkstroke as *const <IInkStrokeDisp as ::windows::core::Abi>::Abi as *const <IInkStrokeDisp as ::windows::core::DefaultType>::DefaultType), &*(&pdirtyrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ink(::core::mem::transmute_copy(&ppiinkobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Ink(&*(&piinkobj as *const <IInkDisp as ::windows::core::Abi>::Abi as *const <IInkDisp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStrokeBuilder>, ::windows::core::GetTrustLevel, CreateStroke::<Impl, OFFSET>, BeginStroke::<Impl, OFFSET>, AppendPackets::<Impl, OFFSET>, EndStroke::<Impl, OFFSET>, Ink::<Impl, OFFSET>, putref_Ink::<Impl, OFFSET>)
    }
}
pub trait IStylusAsyncPluginImpl: Sized + IStylusPluginImpl {}
impl ::windows::core::RuntimeName for IStylusAsyncPlugin {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IStylusAsyncPlugin";
}
impl IStylusAsyncPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusAsyncPluginImpl, const OFFSET: isize>() -> IStylusAsyncPluginVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStylusAsyncPlugin>, ::windows::core::GetTrustLevel)
    }
}
pub trait IStylusPluginImpl: Sized {
    fn RealTimeStylusEnabled();
    fn RealTimeStylusDisabled();
    fn StylusInRange();
    fn StylusOutOfRange();
    fn StylusDown();
    fn StylusUp();
    fn StylusButtonDown();
    fn StylusButtonUp();
    fn InAirPackets();
    fn Packets();
    fn CustomStylusDataAdded();
    fn SystemEvent();
    fn TabletAdded();
    fn TabletRemoved();
    fn Error();
    fn UpdateMapping();
    fn DataInterest();
}
impl ::windows::core::RuntimeName for IStylusPlugin {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IStylusPlugin";
}
impl IStylusPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPluginImpl, const OFFSET: isize>() -> IStylusPluginVtbl {
        unsafe extern "system" fn RealTimeStylusEnabled<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RealTimeStylusEnabled(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), ctcidcount, ptcids) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RealTimeStylusDisabled<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RealTimeStylusDisabled(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), ctcidcount, ptcids) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusInRange<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusInRange(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), tcid, sid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusOutOfRange<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusOutOfRange(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), tcid, sid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusDown<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusDown(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pstylusinfo as *const <StylusInfo as ::windows::core::Abi>::Abi as *const <StylusInfo as ::windows::core::DefaultType>::DefaultType), cpropcountperpkt, ppacket, ppinoutpkt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusUp<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusUp(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pstylusinfo as *const <StylusInfo as ::windows::core::Abi>::Abi as *const <StylusInfo as ::windows::core::DefaultType>::DefaultType), cpropcountperpkt, ppacket, ppinoutpkt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusButtonDown<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: &::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusButtonDown(
                &*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType),
                sid,
                &*(&pguidstylusbutton as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pstyluspos as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StylusButtonUp<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: &::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StylusButtonUp(
                &*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType),
                sid,
                &*(&pguidstylusbutton as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pstyluspos as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InAirPackets<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InAirPackets(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pstylusinfo as *const <StylusInfo as ::windows::core::Abi>::Abi as *const <StylusInfo as ::windows::core::DefaultType>::DefaultType), cpktcount, cpktbufflength, ppackets, pcinoutpkts, ppinoutpkts) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Packets<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Packets(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pstylusinfo as *const <StylusInfo as ::windows::core::Abi>::Abi as *const <StylusInfo as ::windows::core::DefaultType>::DefaultType), cpktcount, cpktbufflength, ppackets, pcinoutpkts, ppinoutpkts) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomStylusDataAdded<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pguidid: &::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomStylusDataAdded(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pguidid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbdata, pbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemEvent<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemEvent(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), tcid, sid, event, &*(&eventdata as *const <SYSTEM_EVENT_DATA as ::windows::core::Abi>::Abi as *const <SYSTEM_EVENT_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabletAdded<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabletAdded(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&pitablet as *const <IInkTablet as ::windows::core::Abi>::Abi as *const <IInkTablet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabletRemoved<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, itabletindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabletRemoved(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), itabletindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, piplugin: ::windows::core::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType), &*(&piplugin as *const <IStylusPlugin as ::windows::core::Abi>::Abi as *const <IStylusPlugin as ::windows::core::DefaultType>::DefaultType), datainterest, hrerrorcode, lptrkey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateMapping<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateMapping(&*(&pirtssrc as *const <IRealTimeStylus as ::windows::core::Abi>::Abi as *const <IRealTimeStylus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataInterest<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataInterest(::core::mem::transmute_copy(&pdatainterest)) {
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
            ::windows::core::GetRuntimeClassName::<IStylusPlugin>,
            ::windows::core::GetTrustLevel,
            RealTimeStylusEnabled::<Impl, OFFSET>,
            RealTimeStylusDisabled::<Impl, OFFSET>,
            StylusInRange::<Impl, OFFSET>,
            StylusOutOfRange::<Impl, OFFSET>,
            StylusDown::<Impl, OFFSET>,
            StylusUp::<Impl, OFFSET>,
            StylusButtonDown::<Impl, OFFSET>,
            StylusButtonUp::<Impl, OFFSET>,
            InAirPackets::<Impl, OFFSET>,
            Packets::<Impl, OFFSET>,
            CustomStylusDataAdded::<Impl, OFFSET>,
            SystemEvent::<Impl, OFFSET>,
            TabletAdded::<Impl, OFFSET>,
            TabletRemoved::<Impl, OFFSET>,
            Error::<Impl, OFFSET>,
            UpdateMapping::<Impl, OFFSET>,
            DataInterest::<Impl, OFFSET>,
        )
    }
}
pub trait IStylusSyncPluginImpl: Sized + IStylusPluginImpl {}
impl ::windows::core::RuntimeName for IStylusSyncPlugin {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.IStylusSyncPlugin";
}
impl IStylusSyncPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusSyncPluginImpl, const OFFSET: isize>() -> IStylusSyncPluginVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStylusSyncPlugin>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITextInputPanelImpl: Sized {
    fn AttachedEditWindow();
    fn SetAttachedEditWindow();
    fn CurrentInteractionMode();
    fn DefaultInPlaceState();
    fn SetDefaultInPlaceState();
    fn CurrentInPlaceState();
    fn DefaultInputArea();
    fn SetDefaultInputArea();
    fn CurrentInputArea();
    fn CurrentCorrectionMode();
    fn PreferredInPlaceDirection();
    fn SetPreferredInPlaceDirection();
    fn ExpandPostInsertionCorrection();
    fn SetExpandPostInsertionCorrection();
    fn InPlaceVisibleOnFocus();
    fn SetInPlaceVisibleOnFocus();
    fn InPlaceBoundingRectangle();
    fn PopUpCorrectionHeight();
    fn PopDownCorrectionHeight();
    fn CommitPendingInput();
    fn SetInPlaceVisibility();
    fn SetInPlacePosition();
    fn SetInPlaceHoverTargetPosition();
    fn Advise();
    fn Unadvise();
}
impl ::windows::core::RuntimeName for ITextInputPanel {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ITextInputPanel";
}
impl ITextInputPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelImpl, const OFFSET: isize>() -> ITextInputPanelVtbl {
        unsafe extern "system" fn AttachedEditWindow<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachedEditWindow(&*(&attachededitwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentInteractionMode<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentInteractionMode(::core::mem::transmute_copy(&currentinteractionmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInPlaceState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultInPlaceState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentInPlaceState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInputArea(::core::mem::transmute_copy(&area)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultInputArea(area) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentInputArea(::core::mem::transmute_copy(&area)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCorrectionMode<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCorrectionMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: *mut InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredInPlaceDirection(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredInPlaceDirection(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandPostInsertionCorrection(::core::mem::transmute_copy(&expand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExpandPostInsertionCorrection(&*(&expand as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceVisibleOnFocus(::core::mem::transmute_copy(&visible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPlaceVisibleOnFocus(&*(&visible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceBoundingRectangle(::core::mem::transmute_copy(&boundingrectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopUpCorrectionHeight(::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopDownCorrectionHeight(::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitPendingInput<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitPendingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceVisibility<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPlaceVisibility(&*(&visible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlacePosition<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPlacePosition(xposition, yposition, position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPlaceHoverTargetPosition(xposition, yposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr, eventmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&eventsink as *const <ITextInputPanelEventSink as ::windows::core::Abi>::Abi as *const <ITextInputPanelEventSink as ::windows::core::DefaultType>::DefaultType), eventmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&eventsink as *const <ITextInputPanelEventSink as ::windows::core::Abi>::Abi as *const <ITextInputPanelEventSink as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITextInputPanel>,
            ::windows::core::GetTrustLevel,
            AttachedEditWindow::<Impl, OFFSET>,
            SetAttachedEditWindow::<Impl, OFFSET>,
            CurrentInteractionMode::<Impl, OFFSET>,
            DefaultInPlaceState::<Impl, OFFSET>,
            SetDefaultInPlaceState::<Impl, OFFSET>,
            CurrentInPlaceState::<Impl, OFFSET>,
            DefaultInputArea::<Impl, OFFSET>,
            SetDefaultInputArea::<Impl, OFFSET>,
            CurrentInputArea::<Impl, OFFSET>,
            CurrentCorrectionMode::<Impl, OFFSET>,
            PreferredInPlaceDirection::<Impl, OFFSET>,
            SetPreferredInPlaceDirection::<Impl, OFFSET>,
            ExpandPostInsertionCorrection::<Impl, OFFSET>,
            SetExpandPostInsertionCorrection::<Impl, OFFSET>,
            InPlaceVisibleOnFocus::<Impl, OFFSET>,
            SetInPlaceVisibleOnFocus::<Impl, OFFSET>,
            InPlaceBoundingRectangle::<Impl, OFFSET>,
            PopUpCorrectionHeight::<Impl, OFFSET>,
            PopDownCorrectionHeight::<Impl, OFFSET>,
            CommitPendingInput::<Impl, OFFSET>,
            SetInPlaceVisibility::<Impl, OFFSET>,
            SetInPlacePosition::<Impl, OFFSET>,
            SetInPlaceHoverTargetPosition::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
        )
    }
}
pub trait ITextInputPanelEventSinkImpl: Sized {
    fn InPlaceStateChanging();
    fn InPlaceStateChanged();
    fn InPlaceSizeChanging();
    fn InPlaceSizeChanged();
    fn InputAreaChanging();
    fn InputAreaChanged();
    fn CorrectionModeChanging();
    fn CorrectionModeChanged();
    fn InPlaceVisibilityChanging();
    fn InPlaceVisibilityChanged();
    fn TextInserting();
    fn TextInserted();
}
impl ::windows::core::RuntimeName for ITextInputPanelEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ITextInputPanelEventSink";
}
impl ITextInputPanelEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>() -> ITextInputPanelEventSinkVtbl {
        unsafe extern "system" fn InPlaceStateChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceStateChanging(oldinplacestate, newinplacestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceStateChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceStateChanged(oldinplacestate, newinplacestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceSizeChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceSizeChanging(&*(&oldboundingrectangle as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&newboundingrectangle as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceSizeChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceSizeChanged(&*(&oldboundingrectangle as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&newboundingrectangle as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputAreaChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputAreaChanging(oldinputarea, newinputarea) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputAreaChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputAreaChanged(oldinputarea, newinputarea) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrectionModeChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrectionModeChanging(oldcorrectionmode, newcorrectionmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrectionModeChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrectionModeChanged(oldcorrectionmode, newcorrectionmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceVisibilityChanging(&*(&oldvisible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&newvisible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceVisibilityChanged(&*(&oldvisible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&newvisible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextInserting<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextInserting(&*(&ink as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextInserted<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextInserted(&*(&ink as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITextInputPanelEventSink>,
            ::windows::core::GetTrustLevel,
            InPlaceStateChanging::<Impl, OFFSET>,
            InPlaceStateChanged::<Impl, OFFSET>,
            InPlaceSizeChanging::<Impl, OFFSET>,
            InPlaceSizeChanged::<Impl, OFFSET>,
            InputAreaChanging::<Impl, OFFSET>,
            InputAreaChanged::<Impl, OFFSET>,
            CorrectionModeChanging::<Impl, OFFSET>,
            CorrectionModeChanged::<Impl, OFFSET>,
            InPlaceVisibilityChanging::<Impl, OFFSET>,
            InPlaceVisibilityChanged::<Impl, OFFSET>,
            TextInserting::<Impl, OFFSET>,
            TextInserted::<Impl, OFFSET>,
        )
    }
}
pub trait ITextInputPanelRunInfoImpl: Sized {
    fn IsTipRunning();
}
impl ::windows::core::RuntimeName for ITextInputPanelRunInfo {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ITextInputPanelRunInfo";
}
impl ITextInputPanelRunInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelRunInfoImpl, const OFFSET: isize>() -> ITextInputPanelRunInfoVtbl {
        unsafe extern "system" fn IsTipRunning<Impl: ITextInputPanelRunInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTipRunning(::core::mem::transmute_copy(&pfrunning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextInputPanelRunInfo>, ::windows::core::GetTrustLevel, IsTipRunning::<Impl, OFFSET>)
    }
}
pub trait ITipAutoCompleteClientImpl: Sized {
    fn AdviseProvider();
    fn UnadviseProvider();
    fn UserSelection();
    fn PreferredRects();
    fn RequestShowUI();
}
impl ::windows::core::RuntimeName for ITipAutoCompleteClient {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ITipAutoCompleteClient";
}
impl ITipAutoCompleteClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>() -> ITipAutoCompleteClientVtbl {
        unsafe extern "system" fn AdviseProvider<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseProvider(&*(&hwndfield as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&piprovider as *const <ITipAutoCompleteProvider as ::windows::core::Abi>::Abi as *const <ITipAutoCompleteProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseProvider<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseProvider(&*(&hwndfield as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&piprovider as *const <ITipAutoCompleteProvider as ::windows::core::Abi>::Abi as *const <ITipAutoCompleteProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSelection<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredRects<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredRects(&*(&prcaclist as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&prcfield as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prcmodifiedaclist), ::core::mem::transmute_copy(&pfshownabovetip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestShowUI<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestShowUI(&*(&hwndlist as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfallowshowing)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITipAutoCompleteClient>, ::windows::core::GetTrustLevel, AdviseProvider::<Impl, OFFSET>, UnadviseProvider::<Impl, OFFSET>, UserSelection::<Impl, OFFSET>, PreferredRects::<Impl, OFFSET>, RequestShowUI::<Impl, OFFSET>)
    }
}
pub trait ITipAutoCompleteProviderImpl: Sized {
    fn UpdatePendingText();
    fn Show();
}
impl ::windows::core::RuntimeName for ITipAutoCompleteProvider {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC.ITipAutoCompleteProvider";
}
impl ITipAutoCompleteProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteProviderImpl, const OFFSET: isize>() -> ITipAutoCompleteProviderVtbl {
        unsafe extern "system" fn UpdatePendingText<Impl: ITipAutoCompleteProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpendingtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatePendingText(&*(&bstrpendingtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITipAutoCompleteProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITipAutoCompleteProvider>, ::windows::core::GetTrustLevel, UpdatePendingText::<Impl, OFFSET>, Show::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkCollectorEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkCollectorEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkCollectorEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkCollectorEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkCollectorEventsImpl, const OFFSET: isize>() -> _IInkCollectorEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkCollectorEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEditEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkEditEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkEditEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkEditEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEditEventsImpl, const OFFSET: isize>() -> _IInkEditEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkEditEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEventsImpl, const OFFSET: isize>() -> _IInkEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkOverlayEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkOverlayEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkOverlayEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkOverlayEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkOverlayEventsImpl, const OFFSET: isize>() -> _IInkOverlayEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkOverlayEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkPictureEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkPictureEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkPictureEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkPictureEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkPictureEventsImpl, const OFFSET: isize>() -> _IInkPictureEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkPictureEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkRecognitionEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkRecognitionEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkRecognitionEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkRecognitionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkRecognitionEventsImpl, const OFFSET: isize>() -> _IInkRecognitionEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkRecognitionEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkStrokesEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IInkStrokesEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IInkStrokesEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IInkStrokesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkStrokesEventsImpl, const OFFSET: isize>() -> _IInkStrokesEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IInkStrokesEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IMathInputControlEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IMathInputControlEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IMathInputControlEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IMathInputControlEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IMathInputControlEventsImpl, const OFFSET: isize>() -> _IMathInputControlEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IMathInputControlEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IPenInputPanelEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IPenInputPanelEvents {
    const NAME: &'static str = "Windows.Win32.UI.TabletPC._IPenInputPanelEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IPenInputPanelEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IPenInputPanelEventsImpl, const OFFSET: isize>() -> _IPenInputPanelEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_IPenInputPanelEvents>, ::windows::core::GetTrustLevel)
    }
}
