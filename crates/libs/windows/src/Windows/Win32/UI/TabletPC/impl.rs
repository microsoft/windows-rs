#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IDynamicRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDynamicRendererVtbl {
        unsafe extern "system" fn Enabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HWND<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHWND<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClipRectangle<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipRectangle<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClipRegion<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipRegion<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppida: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pida: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataCacheEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDataCacheEnabled<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseCachedData<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: IDynamicRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            HWND::<Impl, IMPL_OFFSET>,
            SetHWND::<Impl, IMPL_OFFSET>,
            ClipRectangle::<Impl, IMPL_OFFSET>,
            SetClipRectangle::<Impl, IMPL_OFFSET>,
            ClipRegion::<Impl, IMPL_OFFSET>,
            SetClipRegion::<Impl, IMPL_OFFSET>,
            DrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DrawingAttributes::<Impl, IMPL_OFFSET>,
            DataCacheEnabled::<Impl, IMPL_OFFSET>,
            SetDataCacheEnabled::<Impl, IMPL_OFFSET>,
            ReleaseCachedData::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGestureRecognizerImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn MaxStrokeCount();
    fn SetMaxStrokeCount();
    fn EnableGestures();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IGestureRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGestureRecognizerVtbl {
        unsafe extern "system" fn Enabled<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxStrokeCount<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstrokes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxStrokeCount<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstrokes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableGestures<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IGestureRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Enabled::<Impl, IMPL_OFFSET>, SetEnabled::<Impl, IMPL_OFFSET>, MaxStrokeCount::<Impl, IMPL_OFFSET>, SetMaxStrokeCount::<Impl, IMPL_OFFSET>, EnableGestures::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGestureRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IHandwrittenTextInsertionImpl: Sized {
    fn InsertRecognitionResultsArray();
    fn InsertInkRecognitionResult();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IHandwrittenTextInsertionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandwrittenTextInsertionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHandwrittenTextInsertionVtbl {
        unsafe extern "system" fn InsertRecognitionResultsArray<Impl: IHandwrittenTextInsertionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Impl: IHandwrittenTextInsertionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkrecoresult: ::windows::core::RawPtr, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InsertRecognitionResultsArray::<Impl, IMPL_OFFSET>, InsertInkRecognitionResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHandwrittenTextInsertion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInk as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCollectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCollectorVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SethWnd<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Renderer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MousePointer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cursors<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginX<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginY<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tablet<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            hWnd::<Impl, IMPL_OFFSET>,
            SethWnd::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            Renderer::<Impl, IMPL_OFFSET>,
            putref_Renderer::<Impl, IMPL_OFFSET>,
            Ink::<Impl, IMPL_OFFSET>,
            putref_Ink::<Impl, IMPL_OFFSET>,
            AutoRedraw::<Impl, IMPL_OFFSET>,
            SetAutoRedraw::<Impl, IMPL_OFFSET>,
            CollectingInk::<Impl, IMPL_OFFSET>,
            CollectionMode::<Impl, IMPL_OFFSET>,
            SetCollectionMode::<Impl, IMPL_OFFSET>,
            DynamicRendering::<Impl, IMPL_OFFSET>,
            SetDynamicRendering::<Impl, IMPL_OFFSET>,
            DesiredPacketDescription::<Impl, IMPL_OFFSET>,
            SetDesiredPacketDescription::<Impl, IMPL_OFFSET>,
            MouseIcon::<Impl, IMPL_OFFSET>,
            SetMouseIcon::<Impl, IMPL_OFFSET>,
            putref_MouseIcon::<Impl, IMPL_OFFSET>,
            MousePointer::<Impl, IMPL_OFFSET>,
            SetMousePointer::<Impl, IMPL_OFFSET>,
            Cursors::<Impl, IMPL_OFFSET>,
            MarginX::<Impl, IMPL_OFFSET>,
            SetMarginX::<Impl, IMPL_OFFSET>,
            MarginY::<Impl, IMPL_OFFSET>,
            SetMarginY::<Impl, IMPL_OFFSET>,
            Tablet::<Impl, IMPL_OFFSET>,
            SupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SetSupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SetGestureStatus::<Impl, IMPL_OFFSET>,
            GetGestureStatus::<Impl, IMPL_OFFSET>,
            GetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetAllTabletsMode::<Impl, IMPL_OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, IMPL_OFFSET>,
            GetEventInterest::<Impl, IMPL_OFFSET>,
            SetEventInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn Inverted();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn Tablet();
    fn Buttons();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCursorVtbl {
        unsafe extern "system" fn Name<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Inverted<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tablet<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buttons<Impl: IInkCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttons: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            Inverted::<Impl, IMPL_OFFSET>,
            DrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DrawingAttributes::<Impl, IMPL_OFFSET>,
            Tablet::<Impl, IMPL_OFFSET>,
            Buttons::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorButtonImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn State();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCursorButtonVtbl {
        unsafe extern "system" fn Name<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IInkCursorButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursorButton as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorButtonsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButtonsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtonsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCursorButtonsVtbl {
        unsafe extern "system" fn Count<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkCursorButtonsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursorButtons as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCursorsVtbl {
        unsafe extern "system" fn Count<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkCursorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursors as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCustomStrokesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCustomStrokesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCustomStrokesVtbl {
        unsafe extern "system" fn Count<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IInkCustomStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCustomStrokes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDispImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDispVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dirty<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDirty<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CustomStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkinkcustomstrokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteStroke<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtractStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtractWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clip<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestCircle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionrectangle: ::windows::core::RawPtr, intersectpercent: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestWithLasso<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NearestPoint<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStrokes<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStrokesAtRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestrokes: ::windows::core::RawPtr, targetrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStroke<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClipboardCopyWithRectangle<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClipboardCopy<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanPaste<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataobject: ::windows::core::RawPtr, canpaste: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClipboardPaste<Impl: IInkDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, dataobject: ::windows::core::RawPtr, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Strokes::<Impl, IMPL_OFFSET>,
            ExtendedProperties::<Impl, IMPL_OFFSET>,
            Dirty::<Impl, IMPL_OFFSET>,
            SetDirty::<Impl, IMPL_OFFSET>,
            CustomStrokes::<Impl, IMPL_OFFSET>,
            GetBoundingBox::<Impl, IMPL_OFFSET>,
            DeleteStrokes::<Impl, IMPL_OFFSET>,
            DeleteStroke::<Impl, IMPL_OFFSET>,
            ExtractStrokes::<Impl, IMPL_OFFSET>,
            ExtractWithRectangle::<Impl, IMPL_OFFSET>,
            Clip::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            HitTestCircle::<Impl, IMPL_OFFSET>,
            HitTestWithRectangle::<Impl, IMPL_OFFSET>,
            HitTestWithLasso::<Impl, IMPL_OFFSET>,
            NearestPoint::<Impl, IMPL_OFFSET>,
            CreateStrokes::<Impl, IMPL_OFFSET>,
            AddStrokesAtRectangle::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            Load::<Impl, IMPL_OFFSET>,
            CreateStroke::<Impl, IMPL_OFFSET>,
            ClipboardCopyWithRectangle::<Impl, IMPL_OFFSET>,
            ClipboardCopy::<Impl, IMPL_OFFSET>,
            CanPaste::<Impl, IMPL_OFFSET>,
            ClipboardPaste::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDisp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDividerImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn putref_Strokes();
    fn RecognizerContext();
    fn putref_RecognizerContext();
    fn LineHeight();
    fn SetLineHeight();
    fn Divide();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDividerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDividerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDividerVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Strokes<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecognizerContext<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_RecognizerContext<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LineHeight<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLineHeight<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Divide<Impl: IInkDividerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdivisionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Strokes::<Impl, IMPL_OFFSET>,
            putref_Strokes::<Impl, IMPL_OFFSET>,
            RecognizerContext::<Impl, IMPL_OFFSET>,
            putref_RecognizerContext::<Impl, IMPL_OFFSET>,
            LineHeight::<Impl, IMPL_OFFSET>,
            SetLineHeight::<Impl, IMPL_OFFSET>,
            Divide::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionResultImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn ResultByType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDivisionResultVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDivisionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResultByType<Impl: IInkDivisionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Strokes::<Impl, IMPL_OFFSET>, ResultByType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionUnitImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn DivisionType();
    fn RecognizedString();
    fn RotationTransform();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDivisionUnitVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DivisionType<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: *mut InkDivisionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecognizedString<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RotationTransform<Impl: IInkDivisionUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotationtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Strokes::<Impl, IMPL_OFFSET>, DivisionType::<Impl, IMPL_OFFSET>, RecognizedString::<Impl, IMPL_OFFSET>, RotationTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionUnitsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnitsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDivisionUnitsVtbl {
        unsafe extern "system" fn Count<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkDivisionUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkdivisionunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionUnits as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDrawingAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDrawingAttributesVtbl {
        unsafe extern "system" fn Color<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcolor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColor<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Width<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWidth<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Height<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentheight: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHeight<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFitToCurve<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIgnorePressure<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntiAliased<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAntiAliased<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Transparency<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currenttransparency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransparency<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newtransparency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RasterOperation<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRasterOperation<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newrasteroperation: InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpentip: *mut InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPenTip<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpentip: InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IInkDrawingAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Color::<Impl, IMPL_OFFSET>,
            SetColor::<Impl, IMPL_OFFSET>,
            Width::<Impl, IMPL_OFFSET>,
            SetWidth::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            SetHeight::<Impl, IMPL_OFFSET>,
            FitToCurve::<Impl, IMPL_OFFSET>,
            SetFitToCurve::<Impl, IMPL_OFFSET>,
            IgnorePressure::<Impl, IMPL_OFFSET>,
            SetIgnorePressure::<Impl, IMPL_OFFSET>,
            AntiAliased::<Impl, IMPL_OFFSET>,
            SetAntiAliased::<Impl, IMPL_OFFSET>,
            Transparency::<Impl, IMPL_OFFSET>,
            SetTransparency::<Impl, IMPL_OFFSET>,
            RasterOperation::<Impl, IMPL_OFFSET>,
            SetRasterOperation::<Impl, IMPL_OFFSET>,
            PenTip::<Impl, IMPL_OFFSET>,
            SetPenTip::<Impl, IMPL_OFFSET>,
            ExtendedProperties::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkEditVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkEditImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkEditVtbl {
        unsafe extern "system" fn Status<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut InkEditStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseMouseForInput<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseMouseForInput<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InkMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInkMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InkInsertMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInkInsertMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecognitionTimeout<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecognitionTimeout<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recognizer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Recognizer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Factoid<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFactoid<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelInks<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselink: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelInks<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selink: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelInksDisplayMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelInksDisplayMode<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recognize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Appearance<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: *mut AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppearance<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BorderStyle<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBorderStyle<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hwnd<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pohhwnd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Font<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Font<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Text<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MousePointer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Locked<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocked<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultiLine<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultiLine<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScrollBars<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScrollBars<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableNoScroll<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisableNoScroll<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelAlignment<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelAlignment<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelBold<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelBold<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelItalic<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelItalic<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelUnderline<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelUnderline<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelColor<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelFontName<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelFontName<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelFontSize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelFontSize<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelCharOffset<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelCharOffset<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TextRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTextRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelStart<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelStart<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelLength<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelText<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelRTF<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IInkEditImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            UseMouseForInput::<Impl, IMPL_OFFSET>,
            SetUseMouseForInput::<Impl, IMPL_OFFSET>,
            InkMode::<Impl, IMPL_OFFSET>,
            SetInkMode::<Impl, IMPL_OFFSET>,
            InkInsertMode::<Impl, IMPL_OFFSET>,
            SetInkInsertMode::<Impl, IMPL_OFFSET>,
            DrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DrawingAttributes::<Impl, IMPL_OFFSET>,
            RecognitionTimeout::<Impl, IMPL_OFFSET>,
            SetRecognitionTimeout::<Impl, IMPL_OFFSET>,
            Recognizer::<Impl, IMPL_OFFSET>,
            putref_Recognizer::<Impl, IMPL_OFFSET>,
            Factoid::<Impl, IMPL_OFFSET>,
            SetFactoid::<Impl, IMPL_OFFSET>,
            SelInks::<Impl, IMPL_OFFSET>,
            SetSelInks::<Impl, IMPL_OFFSET>,
            SelInksDisplayMode::<Impl, IMPL_OFFSET>,
            SetSelInksDisplayMode::<Impl, IMPL_OFFSET>,
            Recognize::<Impl, IMPL_OFFSET>,
            GetGestureStatus::<Impl, IMPL_OFFSET>,
            SetGestureStatus::<Impl, IMPL_OFFSET>,
            SetBackColor::<Impl, IMPL_OFFSET>,
            BackColor::<Impl, IMPL_OFFSET>,
            Appearance::<Impl, IMPL_OFFSET>,
            SetAppearance::<Impl, IMPL_OFFSET>,
            BorderStyle::<Impl, IMPL_OFFSET>,
            SetBorderStyle::<Impl, IMPL_OFFSET>,
            Hwnd::<Impl, IMPL_OFFSET>,
            Font::<Impl, IMPL_OFFSET>,
            putref_Font::<Impl, IMPL_OFFSET>,
            Text::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            MouseIcon::<Impl, IMPL_OFFSET>,
            SetMouseIcon::<Impl, IMPL_OFFSET>,
            putref_MouseIcon::<Impl, IMPL_OFFSET>,
            MousePointer::<Impl, IMPL_OFFSET>,
            SetMousePointer::<Impl, IMPL_OFFSET>,
            Locked::<Impl, IMPL_OFFSET>,
            SetLocked::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            MaxLength::<Impl, IMPL_OFFSET>,
            SetMaxLength::<Impl, IMPL_OFFSET>,
            MultiLine::<Impl, IMPL_OFFSET>,
            SetMultiLine::<Impl, IMPL_OFFSET>,
            ScrollBars::<Impl, IMPL_OFFSET>,
            SetScrollBars::<Impl, IMPL_OFFSET>,
            DisableNoScroll::<Impl, IMPL_OFFSET>,
            SetDisableNoScroll::<Impl, IMPL_OFFSET>,
            SelAlignment::<Impl, IMPL_OFFSET>,
            SetSelAlignment::<Impl, IMPL_OFFSET>,
            SelBold::<Impl, IMPL_OFFSET>,
            SetSelBold::<Impl, IMPL_OFFSET>,
            SelItalic::<Impl, IMPL_OFFSET>,
            SetSelItalic::<Impl, IMPL_OFFSET>,
            SelUnderline::<Impl, IMPL_OFFSET>,
            SetSelUnderline::<Impl, IMPL_OFFSET>,
            SelColor::<Impl, IMPL_OFFSET>,
            SetSelColor::<Impl, IMPL_OFFSET>,
            SelFontName::<Impl, IMPL_OFFSET>,
            SetSelFontName::<Impl, IMPL_OFFSET>,
            SelFontSize::<Impl, IMPL_OFFSET>,
            SetSelFontSize::<Impl, IMPL_OFFSET>,
            SelCharOffset::<Impl, IMPL_OFFSET>,
            SetSelCharOffset::<Impl, IMPL_OFFSET>,
            TextRTF::<Impl, IMPL_OFFSET>,
            SetTextRTF::<Impl, IMPL_OFFSET>,
            SelStart::<Impl, IMPL_OFFSET>,
            SetSelStart::<Impl, IMPL_OFFSET>,
            SelLength::<Impl, IMPL_OFFSET>,
            SetSelLength::<Impl, IMPL_OFFSET>,
            SelText::<Impl, IMPL_OFFSET>,
            SetSelText::<Impl, IMPL_OFFSET>,
            SelRTF::<Impl, IMPL_OFFSET>,
            SetSelRTF::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkEdit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkExtendedPropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
    fn DoesPropertyExist();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkExtendedPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesPropertyExist<Impl: IInkExtendedPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            DoesPropertyExist::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkExtendedProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkExtendedPropertyImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Data();
    fn SetData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkExtendedPropertyVtbl {
        unsafe extern "system" fn Guid<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IInkExtendedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Guid::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkExtendedProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkGestureImpl: Sized + IDispatchImpl {
    fn Confidence();
    fn Id();
    fn GetHotPoint();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkGestureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkGestureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkGestureVtbl {
        unsafe extern "system" fn Confidence<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut InkApplicationGesture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHotPoint<Impl: IInkGestureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Confidence::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, GetHotPoint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkGesture as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInkLineInfoImpl: Sized {
    fn SetFormat();
    fn GetFormat();
    fn GetInkExtent();
    fn GetCandidate();
    fn SetCandidate();
    fn Recognize();
}
#[cfg(feature = "Win32_Foundation")]
impl IInkLineInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkLineInfoVtbl {
        unsafe extern "system" fn SetFormat<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormat<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInkExtent<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidate<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, pwcrecogword: super::super::Foundation::PWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCandidate<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, strrecogword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recognize<Impl: IInkLineInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFormat::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, GetInkExtent::<Impl, IMPL_OFFSET>, GetCandidate::<Impl, IMPL_OFFSET>, SetCandidate::<Impl, IMPL_OFFSET>, Recognize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkLineInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkOverlayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkOverlayVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SethWnd<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Renderer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MousePointer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditingMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEditingMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Selection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EraserMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEraserMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EraserWidth<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEraserWidth<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cursors<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginX<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginY<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tablet<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestSelection<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            hWnd::<Impl, IMPL_OFFSET>,
            SethWnd::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            Renderer::<Impl, IMPL_OFFSET>,
            putref_Renderer::<Impl, IMPL_OFFSET>,
            Ink::<Impl, IMPL_OFFSET>,
            putref_Ink::<Impl, IMPL_OFFSET>,
            AutoRedraw::<Impl, IMPL_OFFSET>,
            SetAutoRedraw::<Impl, IMPL_OFFSET>,
            CollectingInk::<Impl, IMPL_OFFSET>,
            CollectionMode::<Impl, IMPL_OFFSET>,
            SetCollectionMode::<Impl, IMPL_OFFSET>,
            DynamicRendering::<Impl, IMPL_OFFSET>,
            SetDynamicRendering::<Impl, IMPL_OFFSET>,
            DesiredPacketDescription::<Impl, IMPL_OFFSET>,
            SetDesiredPacketDescription::<Impl, IMPL_OFFSET>,
            MouseIcon::<Impl, IMPL_OFFSET>,
            SetMouseIcon::<Impl, IMPL_OFFSET>,
            putref_MouseIcon::<Impl, IMPL_OFFSET>,
            MousePointer::<Impl, IMPL_OFFSET>,
            SetMousePointer::<Impl, IMPL_OFFSET>,
            EditingMode::<Impl, IMPL_OFFSET>,
            SetEditingMode::<Impl, IMPL_OFFSET>,
            Selection::<Impl, IMPL_OFFSET>,
            SetSelection::<Impl, IMPL_OFFSET>,
            EraserMode::<Impl, IMPL_OFFSET>,
            SetEraserMode::<Impl, IMPL_OFFSET>,
            EraserWidth::<Impl, IMPL_OFFSET>,
            SetEraserWidth::<Impl, IMPL_OFFSET>,
            AttachMode::<Impl, IMPL_OFFSET>,
            SetAttachMode::<Impl, IMPL_OFFSET>,
            Cursors::<Impl, IMPL_OFFSET>,
            MarginX::<Impl, IMPL_OFFSET>,
            SetMarginX::<Impl, IMPL_OFFSET>,
            MarginY::<Impl, IMPL_OFFSET>,
            SetMarginY::<Impl, IMPL_OFFSET>,
            Tablet::<Impl, IMPL_OFFSET>,
            SupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SetSupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SupportHighContrastSelectionUI::<Impl, IMPL_OFFSET>,
            SetSupportHighContrastSelectionUI::<Impl, IMPL_OFFSET>,
            HitTestSelection::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            SetGestureStatus::<Impl, IMPL_OFFSET>,
            GetGestureStatus::<Impl, IMPL_OFFSET>,
            GetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetAllTabletsMode::<Impl, IMPL_OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, IMPL_OFFSET>,
            GetEventInterest::<Impl, IMPL_OFFSET>,
            SetEventInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkOverlay as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPictureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPictureVtbl {
        unsafe extern "system" fn hWnd<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Renderer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Renderer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Ink<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoRedraw<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoRedraw<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectingInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectionMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCollectionMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DynamicRendering<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDynamicRendering<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DesiredPacketDescription<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_MouseIcon<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MousePointer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMousePointer<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditingMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEditingMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Selection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EraserMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEraserMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EraserWidth<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEraserWidth<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Picture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPicture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Picture<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppicture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSizeMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SizeMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackColor<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackColor<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cursors<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginX<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginX<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarginY<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarginY<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tablet<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportHighContrastInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestSelection<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGestureStatus<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGestureStatus<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowInputRectangle<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventInterest<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventInterest<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InkEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInkEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IInkPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbool: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            hWnd::<Impl, IMPL_OFFSET>,
            DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DefaultDrawingAttributes::<Impl, IMPL_OFFSET>,
            Renderer::<Impl, IMPL_OFFSET>,
            putref_Renderer::<Impl, IMPL_OFFSET>,
            Ink::<Impl, IMPL_OFFSET>,
            putref_Ink::<Impl, IMPL_OFFSET>,
            AutoRedraw::<Impl, IMPL_OFFSET>,
            SetAutoRedraw::<Impl, IMPL_OFFSET>,
            CollectingInk::<Impl, IMPL_OFFSET>,
            CollectionMode::<Impl, IMPL_OFFSET>,
            SetCollectionMode::<Impl, IMPL_OFFSET>,
            DynamicRendering::<Impl, IMPL_OFFSET>,
            SetDynamicRendering::<Impl, IMPL_OFFSET>,
            DesiredPacketDescription::<Impl, IMPL_OFFSET>,
            SetDesiredPacketDescription::<Impl, IMPL_OFFSET>,
            MouseIcon::<Impl, IMPL_OFFSET>,
            SetMouseIcon::<Impl, IMPL_OFFSET>,
            putref_MouseIcon::<Impl, IMPL_OFFSET>,
            MousePointer::<Impl, IMPL_OFFSET>,
            SetMousePointer::<Impl, IMPL_OFFSET>,
            EditingMode::<Impl, IMPL_OFFSET>,
            SetEditingMode::<Impl, IMPL_OFFSET>,
            Selection::<Impl, IMPL_OFFSET>,
            SetSelection::<Impl, IMPL_OFFSET>,
            EraserMode::<Impl, IMPL_OFFSET>,
            SetEraserMode::<Impl, IMPL_OFFSET>,
            EraserWidth::<Impl, IMPL_OFFSET>,
            SetEraserWidth::<Impl, IMPL_OFFSET>,
            putref_Picture::<Impl, IMPL_OFFSET>,
            SetPicture::<Impl, IMPL_OFFSET>,
            Picture::<Impl, IMPL_OFFSET>,
            SetSizeMode::<Impl, IMPL_OFFSET>,
            SizeMode::<Impl, IMPL_OFFSET>,
            SetBackColor::<Impl, IMPL_OFFSET>,
            BackColor::<Impl, IMPL_OFFSET>,
            Cursors::<Impl, IMPL_OFFSET>,
            MarginX::<Impl, IMPL_OFFSET>,
            SetMarginX::<Impl, IMPL_OFFSET>,
            MarginY::<Impl, IMPL_OFFSET>,
            SetMarginY::<Impl, IMPL_OFFSET>,
            Tablet::<Impl, IMPL_OFFSET>,
            SupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SetSupportHighContrastInk::<Impl, IMPL_OFFSET>,
            SupportHighContrastSelectionUI::<Impl, IMPL_OFFSET>,
            SetSupportHighContrastSelectionUI::<Impl, IMPL_OFFSET>,
            HitTestSelection::<Impl, IMPL_OFFSET>,
            SetGestureStatus::<Impl, IMPL_OFFSET>,
            GetGestureStatus::<Impl, IMPL_OFFSET>,
            GetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetAllTabletsMode::<Impl, IMPL_OFFSET>,
            SetSingleTabletIntegratedMode::<Impl, IMPL_OFFSET>,
            GetEventInterest::<Impl, IMPL_OFFSET>,
            SetEventInterest::<Impl, IMPL_OFFSET>,
            InkEnabled::<Impl, IMPL_OFFSET>,
            SetInkEnabled::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPicture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognitionAlternateVtbl {
        unsafe extern "system" fn String<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Confidence<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Baseline<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Midline<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ascender<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ascender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Descender<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LineNumber<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LineAlternates<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfidenceAlternates<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidencealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokesFromStrokeRanges<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, getstrokesfromstrokeranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokesFromTextRange<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTextRangeFromStrokes<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternatesWithConstantPropertyValues<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IInkRecognitionAlternateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            String::<Impl, IMPL_OFFSET>,
            Confidence::<Impl, IMPL_OFFSET>,
            Baseline::<Impl, IMPL_OFFSET>,
            Midline::<Impl, IMPL_OFFSET>,
            Ascender::<Impl, IMPL_OFFSET>,
            Descender::<Impl, IMPL_OFFSET>,
            LineNumber::<Impl, IMPL_OFFSET>,
            Strokes::<Impl, IMPL_OFFSET>,
            LineAlternates::<Impl, IMPL_OFFSET>,
            ConfidenceAlternates::<Impl, IMPL_OFFSET>,
            GetStrokesFromStrokeRanges::<Impl, IMPL_OFFSET>,
            GetStrokesFromTextRange::<Impl, IMPL_OFFSET>,
            GetTextRangeFromStrokes::<Impl, IMPL_OFFSET>,
            AlternatesWithConstantPropertyValues::<Impl, IMPL_OFFSET>,
            GetPropertyValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionAlternate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognitionAlternatesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Strokes();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognitionAlternatesVtbl {
        unsafe extern "system" fn Count<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkRecognitionAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecoalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Strokes::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionAlternates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognitionResultImpl: Sized + IDispatchImpl {
    fn TopString();
    fn TopAlternate();
    fn TopConfidence();
    fn Strokes();
    fn AlternatesFromSelection();
    fn ModifyTopAlternate();
    fn SetResultOnStrokes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognitionResultVtbl {
        unsafe extern "system" fn TopString<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TopAlternate<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TopConfidence<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Strokes<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternatesFromSelection<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyTopAlternate<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alternate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResultOnStrokes<Impl: IInkRecognitionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            TopString::<Impl, IMPL_OFFSET>,
            TopAlternate::<Impl, IMPL_OFFSET>,
            TopConfidence::<Impl, IMPL_OFFSET>,
            Strokes::<Impl, IMPL_OFFSET>,
            AlternatesFromSelection::<Impl, IMPL_OFFSET>,
            ModifyTopAlternate::<Impl, IMPL_OFFSET>,
            SetResultOnStrokes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizerImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Capabilities();
    fn Languages();
    fn SupportedProperties();
    fn PreferredPacketDescription();
    fn CreateRecognizerContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerVtbl {
        unsafe extern "system" fn Name<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Vendor<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Capabilities<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Languages<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedProperties<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedproperties: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredPacketDescription<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredpacketdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRecognizerContext<Impl: IInkRecognizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Vendor::<Impl, IMPL_OFFSET>,
            Capabilities::<Impl, IMPL_OFFSET>,
            Languages::<Impl, IMPL_OFFSET>,
            SupportedProperties::<Impl, IMPL_OFFSET>,
            PreferredPacketDescription::<Impl, IMPL_OFFSET>,
            CreateRecognizerContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizer2Impl: Sized + IDispatchImpl {
    fn Id();
    fn UnicodeRanges();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizer2Vtbl {
        unsafe extern "system" fn Id<Impl: IInkRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnicodeRanges<Impl: IInkRecognizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, UnicodeRanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerContextVtbl {
        unsafe extern "system" fn Strokes<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Strokes<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CharacterAutoCompletionMode<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCharacterAutoCompletionMode<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Factoid<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFactoid<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Guide<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Guide<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrefixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrefixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuffixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSuffixText<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecognitionFlags<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: *mut InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecognitionFlags<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WordList<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_WordList<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recognizer<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recognize<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopBackgroundRecognition<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndInkInput<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundRecognize<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundRecognizeWithAlternates<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsStringSupported<Impl: IInkRecognizerContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Strokes::<Impl, IMPL_OFFSET>,
            putref_Strokes::<Impl, IMPL_OFFSET>,
            CharacterAutoCompletionMode::<Impl, IMPL_OFFSET>,
            SetCharacterAutoCompletionMode::<Impl, IMPL_OFFSET>,
            Factoid::<Impl, IMPL_OFFSET>,
            SetFactoid::<Impl, IMPL_OFFSET>,
            Guide::<Impl, IMPL_OFFSET>,
            putref_Guide::<Impl, IMPL_OFFSET>,
            PrefixText::<Impl, IMPL_OFFSET>,
            SetPrefixText::<Impl, IMPL_OFFSET>,
            SuffixText::<Impl, IMPL_OFFSET>,
            SetSuffixText::<Impl, IMPL_OFFSET>,
            RecognitionFlags::<Impl, IMPL_OFFSET>,
            SetRecognitionFlags::<Impl, IMPL_OFFSET>,
            WordList::<Impl, IMPL_OFFSET>,
            putref_WordList::<Impl, IMPL_OFFSET>,
            Recognizer::<Impl, IMPL_OFFSET>,
            Recognize::<Impl, IMPL_OFFSET>,
            StopBackgroundRecognition::<Impl, IMPL_OFFSET>,
            EndInkInput::<Impl, IMPL_OFFSET>,
            BackgroundRecognize::<Impl, IMPL_OFFSET>,
            BackgroundRecognizeWithAlternates::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            IsStringSupported::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizerContext2Impl: Sized + IDispatchImpl {
    fn EnabledUnicodeRanges();
    fn SetEnabledUnicodeRanges();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerContext2Vtbl {
        unsafe extern "system" fn EnabledUnicodeRanges<Impl: IInkRecognizerContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabledUnicodeRanges<Impl: IInkRecognizerContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, EnabledUnicodeRanges::<Impl, IMPL_OFFSET>, SetEnabledUnicodeRanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerGuideVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuideImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizerGuideVtbl {
        unsafe extern "system" fn WritingBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWritingBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawnBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDrawnBox<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rows<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRows<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Columns<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColumns<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Midline<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMidline<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GuideData<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoguide: *mut InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGuideData<Impl: IInkRecognizerGuideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoguide: InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            WritingBox::<Impl, IMPL_OFFSET>,
            SetWritingBox::<Impl, IMPL_OFFSET>,
            DrawnBox::<Impl, IMPL_OFFSET>,
            SetDrawnBox::<Impl, IMPL_OFFSET>,
            Rows::<Impl, IMPL_OFFSET>,
            SetRows::<Impl, IMPL_OFFSET>,
            Columns::<Impl, IMPL_OFFSET>,
            SetColumns::<Impl, IMPL_OFFSET>,
            Midline::<Impl, IMPL_OFFSET>,
            SetMidline::<Impl, IMPL_OFFSET>,
            GuideData::<Impl, IMPL_OFFSET>,
            SetGuideData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerGuide as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn GetDefaultRecognizer();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRecognizersVtbl {
        unsafe extern "system" fn Count<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultRecognizer<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: i32, defaultrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkRecognizersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, GetDefaultRecognizer::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRectangleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRectangleVtbl {
        unsafe extern "system" fn Top<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTop<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Left<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLeft<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bottom<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBottom<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Right<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRight<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRectangle<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRectangle<Impl: IInkRectangleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Top::<Impl, IMPL_OFFSET>,
            SetTop::<Impl, IMPL_OFFSET>,
            Left::<Impl, IMPL_OFFSET>,
            SetLeft::<Impl, IMPL_OFFSET>,
            Bottom::<Impl, IMPL_OFFSET>,
            SetBottom::<Impl, IMPL_OFFSET>,
            Right::<Impl, IMPL_OFFSET>,
            SetRight::<Impl, IMPL_OFFSET>,
            Data::<Impl, IMPL_OFFSET>,
            SetData::<Impl, IMPL_OFFSET>,
            GetRectangle::<Impl, IMPL_OFFSET>,
            SetRectangle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRectangle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkRendererVtbl {
        unsafe extern "system" fn GetViewTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawStroke<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PixelToInkSpace<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InkSpaceToPixel<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PixelToInkSpaceFromPoints<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InkSpaceToPixelFromPoints<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Measure<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MeasureStroke<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetViewTransform::<Impl, IMPL_OFFSET>,
            SetViewTransform::<Impl, IMPL_OFFSET>,
            GetObjectTransform::<Impl, IMPL_OFFSET>,
            SetObjectTransform::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            DrawStroke::<Impl, IMPL_OFFSET>,
            PixelToInkSpace::<Impl, IMPL_OFFSET>,
            InkSpaceToPixel::<Impl, IMPL_OFFSET>,
            PixelToInkSpaceFromPoints::<Impl, IMPL_OFFSET>,
            InkSpaceToPixelFromPoints::<Impl, IMPL_OFFSET>,
            Measure::<Impl, IMPL_OFFSET>,
            MeasureStroke::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            Rotate::<Impl, IMPL_OFFSET>,
            ScaleTransform::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokeDispVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDispImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokeDispVtbl {
        unsafe extern "system" fn ID<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BezierPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawingAttributes<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_DrawingAttributes<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolylineCusps<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BezierCusps<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelfIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PacketCount<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PacketSize<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PacketDescription<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deleted<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRectangleIntersections<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clip<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HitTestCircle<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NearestPoint<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Split<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, splitat: f32, newstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPacketDescriptionPropertyMetrics<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPacketData<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, packetdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPacketValuesByProperty<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPacketValuesByProperty<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlattenedBezierPoints<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Transform<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleToRectangle<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shear<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkStrokeDispImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            BezierPoints::<Impl, IMPL_OFFSET>,
            DrawingAttributes::<Impl, IMPL_OFFSET>,
            putref_DrawingAttributes::<Impl, IMPL_OFFSET>,
            Ink::<Impl, IMPL_OFFSET>,
            ExtendedProperties::<Impl, IMPL_OFFSET>,
            PolylineCusps::<Impl, IMPL_OFFSET>,
            BezierCusps::<Impl, IMPL_OFFSET>,
            SelfIntersections::<Impl, IMPL_OFFSET>,
            PacketCount::<Impl, IMPL_OFFSET>,
            PacketSize::<Impl, IMPL_OFFSET>,
            PacketDescription::<Impl, IMPL_OFFSET>,
            Deleted::<Impl, IMPL_OFFSET>,
            GetBoundingBox::<Impl, IMPL_OFFSET>,
            FindIntersections::<Impl, IMPL_OFFSET>,
            GetRectangleIntersections::<Impl, IMPL_OFFSET>,
            Clip::<Impl, IMPL_OFFSET>,
            HitTestCircle::<Impl, IMPL_OFFSET>,
            NearestPoint::<Impl, IMPL_OFFSET>,
            Split::<Impl, IMPL_OFFSET>,
            GetPacketDescriptionPropertyMetrics::<Impl, IMPL_OFFSET>,
            GetPoints::<Impl, IMPL_OFFSET>,
            SetPoints::<Impl, IMPL_OFFSET>,
            GetPacketData::<Impl, IMPL_OFFSET>,
            GetPacketValuesByProperty::<Impl, IMPL_OFFSET>,
            SetPacketValuesByProperty::<Impl, IMPL_OFFSET>,
            GetFlattenedBezierPoints::<Impl, IMPL_OFFSET>,
            Transform::<Impl, IMPL_OFFSET>,
            ScaleToRectangle::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            Rotate::<Impl, IMPL_OFFSET>,
            Shear::<Impl, IMPL_OFFSET>,
            ScaleTransform::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeDisp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkStrokesVtbl {
        unsafe extern "system" fn Count<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecognitionResult<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ToString<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStrokes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStrokes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyDrawingAttributes<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundingBox<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Transform<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleToRectangle<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shear<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clip<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveRecognitionResult<Impl: IInkStrokesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Ink::<Impl, IMPL_OFFSET>,
            RecognitionResult::<Impl, IMPL_OFFSET>,
            ToString::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            AddStrokes::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            RemoveStrokes::<Impl, IMPL_OFFSET>,
            ModifyDrawingAttributes::<Impl, IMPL_OFFSET>,
            GetBoundingBox::<Impl, IMPL_OFFSET>,
            Transform::<Impl, IMPL_OFFSET>,
            ScaleToRectangle::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            Rotate::<Impl, IMPL_OFFSET>,
            Shear::<Impl, IMPL_OFFSET>,
            ScaleTransform::<Impl, IMPL_OFFSET>,
            Clip::<Impl, IMPL_OFFSET>,
            RemoveRecognitionResult::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTabletImpl: Sized + IDispatchImpl {
    fn Name();
    fn PlugAndPlayId();
    fn MaximumInputRectangle();
    fn HardwareCapabilities();
    fn IsPacketPropertySupported();
    fn GetPropertyMetrics();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTabletVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTabletImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkTabletVtbl {
        unsafe extern "system" fn Name<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlugAndPlayId<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaximumInputRectangle<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HardwareCapabilities<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPacketPropertySupported<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyMetrics<Impl: IInkTabletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            PlugAndPlayId::<Impl, IMPL_OFFSET>,
            MaximumInputRectangle::<Impl, IMPL_OFFSET>,
            HardwareCapabilities::<Impl, IMPL_OFFSET>,
            IsPacketPropertySupported::<Impl, IMPL_OFFSET>,
            GetPropertyMetrics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet2Impl: Sized + IDispatchImpl {
    fn DeviceKind();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkTablet2Vtbl {
        unsafe extern "system" fn DeviceKind<Impl: IInkTablet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: *mut TabletDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DeviceKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet3Impl: Sized + IDispatchImpl {
    fn IsMultiTouch();
    fn MaximumCursors();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkTablet3Vtbl {
        unsafe extern "system" fn IsMultiTouch<Impl: IInkTablet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismultitouch: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaximumCursors<Impl: IInkTablet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaximumcursors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, IsMultiTouch::<Impl, IMPL_OFFSET>, MaximumCursors::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTabletsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn DefaultTablet();
    fn Item();
    fn IsPacketPropertySupported();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTabletsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTabletsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkTabletsVtbl {
        unsafe extern "system" fn Count<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultTablet<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPacketPropertySupported<Impl: IInkTabletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, DefaultTablet::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, IsPacketPropertySupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablets as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkTransformVtbl {
        unsafe extern "system" fn Reset<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Translate<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reflect<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontally: i16, vertically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shear<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScaleTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eM11<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteM11<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eM12<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteM12<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eM21<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteM21<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eM22<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteM22<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eDx<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteDx<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn eDy<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeteDy<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IInkTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            Translate::<Impl, IMPL_OFFSET>,
            Rotate::<Impl, IMPL_OFFSET>,
            Reflect::<Impl, IMPL_OFFSET>,
            Shear::<Impl, IMPL_OFFSET>,
            ScaleTransform::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            eM11::<Impl, IMPL_OFFSET>,
            SeteM11::<Impl, IMPL_OFFSET>,
            eM12::<Impl, IMPL_OFFSET>,
            SeteM12::<Impl, IMPL_OFFSET>,
            eM21::<Impl, IMPL_OFFSET>,
            SeteM21::<Impl, IMPL_OFFSET>,
            eM22::<Impl, IMPL_OFFSET>,
            SeteM22::<Impl, IMPL_OFFSET>,
            eDx::<Impl, IMPL_OFFSET>,
            SeteDx::<Impl, IMPL_OFFSET>,
            eDy::<Impl, IMPL_OFFSET>,
            SeteDy::<Impl, IMPL_OFFSET>,
            Data::<Impl, IMPL_OFFSET>,
            SetData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkWordListImpl: Sized + IDispatchImpl {
    fn AddWord();
    fn RemoveWord();
    fn Merge();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWordListVtbl {
        unsafe extern "system" fn AddWord<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveWord<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, removeword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Merge<Impl: IInkWordListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergewordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, AddWord::<Impl, IMPL_OFFSET>, RemoveWord::<Impl, IMPL_OFFSET>, Merge::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWordList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkWordList2Impl: Sized + IDispatchImpl {
    fn AddWords();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWordList2Vtbl {
        unsafe extern "system" fn AddWords<Impl: IInkWordList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwords: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, AddWords::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWordList2 as ::windows::core::Interface>::IID
    }
}
pub trait IInputPanelWindowHandleImpl: Sized {
    fn AttachedEditWindow32();
    fn SetAttachedEditWindow32();
    fn AttachedEditWindow64();
    fn SetAttachedEditWindow64();
}
impl IInputPanelWindowHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPanelWindowHandleVtbl {
        unsafe extern "system" fn AttachedEditWindow32<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachedEditWindow64<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Impl: IInputPanelWindowHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AttachedEditWindow32::<Impl, IMPL_OFFSET>, SetAttachedEditWindow32::<Impl, IMPL_OFFSET>, AttachedEditWindow64::<Impl, IMPL_OFFSET>, SetAttachedEditWindow64::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPanelWindowHandle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMathInputControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMathInputControlVtbl {
        unsafe extern "system" fn Show<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hide<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsVisible<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshown: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomPaint<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: i32, paint: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaptionText<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captiontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadInk<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwnerWindow<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownerwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableExtendedButtons<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviewHeight<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreviewHeight<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableAutoGrow<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autogrow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFunctionName<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFunctionName<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHoverIcon<Impl: IMathInputControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hoverimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Show::<Impl, IMPL_OFFSET>,
            Hide::<Impl, IMPL_OFFSET>,
            IsVisible::<Impl, IMPL_OFFSET>,
            GetPosition::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            SetCustomPaint::<Impl, IMPL_OFFSET>,
            SetCaptionText::<Impl, IMPL_OFFSET>,
            LoadInk::<Impl, IMPL_OFFSET>,
            SetOwnerWindow::<Impl, IMPL_OFFSET>,
            EnableExtendedButtons::<Impl, IMPL_OFFSET>,
            GetPreviewHeight::<Impl, IMPL_OFFSET>,
            SetPreviewHeight::<Impl, IMPL_OFFSET>,
            EnableAutoGrow::<Impl, IMPL_OFFSET>,
            AddFunctionName::<Impl, IMPL_OFFSET>,
            RemoveFunctionName::<Impl, IMPL_OFFSET>,
            GetHoverIcon::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMathInputControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPenInputPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPenInputPanelVtbl {
        unsafe extern "system" fn Busy<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busy: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Factoid<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFactoid<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachedEditWindow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachedEditWindow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultPanel<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Visible<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVisible<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Top<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Left<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Width<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Height<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoShow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pautoshow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoShow<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoshow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveTo<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitPendingInput<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableTsf<Impl: IPenInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Busy::<Impl, IMPL_OFFSET>,
            Factoid::<Impl, IMPL_OFFSET>,
            SetFactoid::<Impl, IMPL_OFFSET>,
            AttachedEditWindow::<Impl, IMPL_OFFSET>,
            SetAttachedEditWindow::<Impl, IMPL_OFFSET>,
            CurrentPanel::<Impl, IMPL_OFFSET>,
            SetCurrentPanel::<Impl, IMPL_OFFSET>,
            DefaultPanel::<Impl, IMPL_OFFSET>,
            SetDefaultPanel::<Impl, IMPL_OFFSET>,
            Visible::<Impl, IMPL_OFFSET>,
            SetVisible::<Impl, IMPL_OFFSET>,
            Top::<Impl, IMPL_OFFSET>,
            Left::<Impl, IMPL_OFFSET>,
            Width::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            VerticalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset::<Impl, IMPL_OFFSET>,
            HorizontalOffset::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            AutoShow::<Impl, IMPL_OFFSET>,
            SetAutoShow::<Impl, IMPL_OFFSET>,
            MoveTo::<Impl, IMPL_OFFSET>,
            CommitPendingInput::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            EnableTsf::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenInputPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRealTimeStylusVtbl {
        unsafe extern "system" fn Enabled<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HWND<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHWND<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowInputRectangle<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllStylusSyncPlugins<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStylusSyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStylusSyncPluginCount<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllStylusAsyncPlugins<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStylusAsyncPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStylusAsyncPluginCount<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChildRealTimeStylusPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppirts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ChildRealTimeStylusPlugin<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCustomStylusDataToQueue<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sq: StylusQueue, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStylusQueues<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllTabletsMode<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSingleTabletMode<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTablet<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisingletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTabletContextIdFromTablet<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr, ptcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTabletFromTabletContextId<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, ppitablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllTabletContextIds<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyluses<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkcursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStylusForId<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u32, ppiinkcursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropertyguids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesiredPacketDescription<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPacketDescriptionData<Impl: IRealTimeStylusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            HWND::<Impl, IMPL_OFFSET>,
            SetHWND::<Impl, IMPL_OFFSET>,
            WindowInputRectangle::<Impl, IMPL_OFFSET>,
            SetWindowInputRectangle::<Impl, IMPL_OFFSET>,
            AddStylusSyncPlugin::<Impl, IMPL_OFFSET>,
            RemoveStylusSyncPlugin::<Impl, IMPL_OFFSET>,
            RemoveAllStylusSyncPlugins::<Impl, IMPL_OFFSET>,
            GetStylusSyncPlugin::<Impl, IMPL_OFFSET>,
            GetStylusSyncPluginCount::<Impl, IMPL_OFFSET>,
            AddStylusAsyncPlugin::<Impl, IMPL_OFFSET>,
            RemoveStylusAsyncPlugin::<Impl, IMPL_OFFSET>,
            RemoveAllStylusAsyncPlugins::<Impl, IMPL_OFFSET>,
            GetStylusAsyncPlugin::<Impl, IMPL_OFFSET>,
            GetStylusAsyncPluginCount::<Impl, IMPL_OFFSET>,
            ChildRealTimeStylusPlugin::<Impl, IMPL_OFFSET>,
            putref_ChildRealTimeStylusPlugin::<Impl, IMPL_OFFSET>,
            AddCustomStylusDataToQueue::<Impl, IMPL_OFFSET>,
            ClearStylusQueues::<Impl, IMPL_OFFSET>,
            SetAllTabletsMode::<Impl, IMPL_OFFSET>,
            SetSingleTabletMode::<Impl, IMPL_OFFSET>,
            GetTablet::<Impl, IMPL_OFFSET>,
            GetTabletContextIdFromTablet::<Impl, IMPL_OFFSET>,
            GetTabletFromTabletContextId::<Impl, IMPL_OFFSET>,
            GetAllTabletContextIds::<Impl, IMPL_OFFSET>,
            GetStyluses::<Impl, IMPL_OFFSET>,
            GetStylusForId::<Impl, IMPL_OFFSET>,
            SetDesiredPacketDescription::<Impl, IMPL_OFFSET>,
            GetDesiredPacketDescription::<Impl, IMPL_OFFSET>,
            GetPacketDescriptionData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRealTimeStylus2Impl: Sized {
    fn FlicksEnabled();
    fn SetFlicksEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRealTimeStylus2Vtbl {
        unsafe extern "system" fn FlicksEnabled<Impl: IRealTimeStylus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlicksEnabled<Impl: IRealTimeStylus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FlicksEnabled::<Impl, IMPL_OFFSET>, SetFlicksEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRealTimeStylus3Impl: Sized {
    fn MultiTouchEnabled();
    fn SetMultiTouchEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRealTimeStylus3Vtbl {
        unsafe extern "system" fn MultiTouchEnabled<Impl: IRealTimeStylus3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultiTouchEnabled<Impl: IRealTimeStylus3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MultiTouchEnabled::<Impl, IMPL_OFFSET>, SetMultiTouchEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus3 as ::windows::core::Interface>::IID
    }
}
pub trait IRealTimeStylusSynchronizationImpl: Sized {
    fn AcquireLock();
    fn ReleaseLock();
}
impl IRealTimeStylusSynchronizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusSynchronizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRealTimeStylusSynchronizationVtbl {
        unsafe extern "system" fn AcquireLock<Impl: IRealTimeStylusSynchronizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseLock<Impl: IRealTimeStylusSynchronizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AcquireLock::<Impl, IMPL_OFFSET>, ReleaseLock::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylusSynchronization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISketchInkImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISketchInkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISketchInkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISketchInkVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISketchInk as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStrokeBuilderImpl: Sized {
    fn CreateStroke();
    fn BeginStroke();
    fn AppendPackets();
    fn EndStroke();
    fn Ink();
    fn putref_Ink();
}
#[cfg(feature = "Win32_Foundation")]
impl IStrokeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStrokeBuilderVtbl {
        unsafe extern "system" fn CreateStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppendPackets<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndStroke<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut ::windows::core::RawPtr, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ink<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Ink<Impl: IStrokeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStroke::<Impl, IMPL_OFFSET>, BeginStroke::<Impl, IMPL_OFFSET>, AppendPackets::<Impl, IMPL_OFFSET>, EndStroke::<Impl, IMPL_OFFSET>, Ink::<Impl, IMPL_OFFSET>, putref_Ink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStrokeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStylusAsyncPluginImpl: Sized + IStylusPluginImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IStylusAsyncPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusAsyncPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylusAsyncPluginVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RealTimeStylusEnabled::<Impl, IMPL_OFFSET>,
            RealTimeStylusDisabled::<Impl, IMPL_OFFSET>,
            StylusInRange::<Impl, IMPL_OFFSET>,
            StylusOutOfRange::<Impl, IMPL_OFFSET>,
            StylusDown::<Impl, IMPL_OFFSET>,
            StylusUp::<Impl, IMPL_OFFSET>,
            StylusButtonDown::<Impl, IMPL_OFFSET>,
            StylusButtonUp::<Impl, IMPL_OFFSET>,
            InAirPackets::<Impl, IMPL_OFFSET>,
            Packets::<Impl, IMPL_OFFSET>,
            CustomStylusDataAdded::<Impl, IMPL_OFFSET>,
            SystemEvent::<Impl, IMPL_OFFSET>,
            TabletAdded::<Impl, IMPL_OFFSET>,
            TabletRemoved::<Impl, IMPL_OFFSET>,
            Error::<Impl, IMPL_OFFSET>,
            UpdateMapping::<Impl, IMPL_OFFSET>,
            DataInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusAsyncPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IStylusPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylusPluginVtbl {
        unsafe extern "system" fn RealTimeStylusEnabled<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RealTimeStylusDisabled<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusInRange<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusOutOfRange<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusDown<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusUp<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusButtonDown<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StylusButtonUp<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InAirPackets<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Packets<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CustomStylusDataAdded<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SystemEvent<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TabletAdded<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TabletRemoved<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, itabletindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, piplugin: ::windows::core::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateMapping<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataInterest<Impl: IStylusPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RealTimeStylusEnabled::<Impl, IMPL_OFFSET>,
            RealTimeStylusDisabled::<Impl, IMPL_OFFSET>,
            StylusInRange::<Impl, IMPL_OFFSET>,
            StylusOutOfRange::<Impl, IMPL_OFFSET>,
            StylusDown::<Impl, IMPL_OFFSET>,
            StylusUp::<Impl, IMPL_OFFSET>,
            StylusButtonDown::<Impl, IMPL_OFFSET>,
            StylusButtonUp::<Impl, IMPL_OFFSET>,
            InAirPackets::<Impl, IMPL_OFFSET>,
            Packets::<Impl, IMPL_OFFSET>,
            CustomStylusDataAdded::<Impl, IMPL_OFFSET>,
            SystemEvent::<Impl, IMPL_OFFSET>,
            TabletAdded::<Impl, IMPL_OFFSET>,
            TabletRemoved::<Impl, IMPL_OFFSET>,
            Error::<Impl, IMPL_OFFSET>,
            UpdateMapping::<Impl, IMPL_OFFSET>,
            DataInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStylusSyncPluginImpl: Sized + IStylusPluginImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IStylusSyncPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusSyncPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylusSyncPluginVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RealTimeStylusEnabled::<Impl, IMPL_OFFSET>,
            RealTimeStylusDisabled::<Impl, IMPL_OFFSET>,
            StylusInRange::<Impl, IMPL_OFFSET>,
            StylusOutOfRange::<Impl, IMPL_OFFSET>,
            StylusDown::<Impl, IMPL_OFFSET>,
            StylusUp::<Impl, IMPL_OFFSET>,
            StylusButtonDown::<Impl, IMPL_OFFSET>,
            StylusButtonUp::<Impl, IMPL_OFFSET>,
            InAirPackets::<Impl, IMPL_OFFSET>,
            Packets::<Impl, IMPL_OFFSET>,
            CustomStylusDataAdded::<Impl, IMPL_OFFSET>,
            SystemEvent::<Impl, IMPL_OFFSET>,
            TabletAdded::<Impl, IMPL_OFFSET>,
            TabletRemoved::<Impl, IMPL_OFFSET>,
            Error::<Impl, IMPL_OFFSET>,
            UpdateMapping::<Impl, IMPL_OFFSET>,
            DataInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusSyncPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextInputPanelVtbl {
        unsafe extern "system" fn AttachedEditWindow<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachedEditWindow<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentInteractionMode<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentInPlaceState<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentInputArea<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCorrectionMode<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: *mut InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitPendingInput<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInPlaceVisibility<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInPlacePosition<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr, eventmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: ITextInputPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AttachedEditWindow::<Impl, IMPL_OFFSET>,
            SetAttachedEditWindow::<Impl, IMPL_OFFSET>,
            CurrentInteractionMode::<Impl, IMPL_OFFSET>,
            DefaultInPlaceState::<Impl, IMPL_OFFSET>,
            SetDefaultInPlaceState::<Impl, IMPL_OFFSET>,
            CurrentInPlaceState::<Impl, IMPL_OFFSET>,
            DefaultInputArea::<Impl, IMPL_OFFSET>,
            SetDefaultInputArea::<Impl, IMPL_OFFSET>,
            CurrentInputArea::<Impl, IMPL_OFFSET>,
            CurrentCorrectionMode::<Impl, IMPL_OFFSET>,
            PreferredInPlaceDirection::<Impl, IMPL_OFFSET>,
            SetPreferredInPlaceDirection::<Impl, IMPL_OFFSET>,
            ExpandPostInsertionCorrection::<Impl, IMPL_OFFSET>,
            SetExpandPostInsertionCorrection::<Impl, IMPL_OFFSET>,
            InPlaceVisibleOnFocus::<Impl, IMPL_OFFSET>,
            SetInPlaceVisibleOnFocus::<Impl, IMPL_OFFSET>,
            InPlaceBoundingRectangle::<Impl, IMPL_OFFSET>,
            PopUpCorrectionHeight::<Impl, IMPL_OFFSET>,
            PopDownCorrectionHeight::<Impl, IMPL_OFFSET>,
            CommitPendingInput::<Impl, IMPL_OFFSET>,
            SetInPlaceVisibility::<Impl, IMPL_OFFSET>,
            SetInPlacePosition::<Impl, IMPL_OFFSET>,
            SetInPlaceHoverTargetPosition::<Impl, IMPL_OFFSET>,
            Advise::<Impl, IMPL_OFFSET>,
            Unadvise::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextInputPanelEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextInputPanelEventSinkVtbl {
        unsafe extern "system" fn InPlaceStateChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceStateChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceSizeChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceSizeChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InputAreaChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InputAreaChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrectionModeChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrectionModeChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TextInserting<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TextInserted<Impl: ITextInputPanelEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InPlaceStateChanging::<Impl, IMPL_OFFSET>,
            InPlaceStateChanged::<Impl, IMPL_OFFSET>,
            InPlaceSizeChanging::<Impl, IMPL_OFFSET>,
            InPlaceSizeChanged::<Impl, IMPL_OFFSET>,
            InputAreaChanging::<Impl, IMPL_OFFSET>,
            InputAreaChanged::<Impl, IMPL_OFFSET>,
            CorrectionModeChanging::<Impl, IMPL_OFFSET>,
            CorrectionModeChanged::<Impl, IMPL_OFFSET>,
            InPlaceVisibilityChanging::<Impl, IMPL_OFFSET>,
            InPlaceVisibilityChanged::<Impl, IMPL_OFFSET>,
            TextInserting::<Impl, IMPL_OFFSET>,
            TextInserted::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanelEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextInputPanelRunInfoImpl: Sized {
    fn IsTipRunning();
}
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanelRunInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelRunInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextInputPanelRunInfoVtbl {
        unsafe extern "system" fn IsTipRunning<Impl: ITextInputPanelRunInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsTipRunning::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanelRunInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipAutoCompleteClientImpl: Sized {
    fn AdviseProvider();
    fn UnadviseProvider();
    fn UserSelection();
    fn PreferredRects();
    fn RequestShowUI();
}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipAutoCompleteClientVtbl {
        unsafe extern "system" fn AdviseProvider<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnadviseProvider<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSelection<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredRects<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestShowUI<Impl: ITipAutoCompleteClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AdviseProvider::<Impl, IMPL_OFFSET>, UnadviseProvider::<Impl, IMPL_OFFSET>, UserSelection::<Impl, IMPL_OFFSET>, PreferredRects::<Impl, IMPL_OFFSET>, RequestShowUI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipAutoCompleteClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipAutoCompleteProviderImpl: Sized {
    fn UpdatePendingText();
    fn Show();
}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITipAutoCompleteProviderVtbl {
        unsafe extern "system" fn UpdatePendingText<Impl: ITipAutoCompleteProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpendingtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Show<Impl: ITipAutoCompleteProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, UpdatePendingText::<Impl, IMPL_OFFSET>, Show::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipAutoCompleteProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkCollectorEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkCollectorEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkCollectorEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkCollectorEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkCollectorEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEditEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEditEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEditEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkEditEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEditEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkOverlayEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkOverlayEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkOverlayEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkOverlayEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkOverlayEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkPictureEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkPictureEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkPictureEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkPictureEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkPictureEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkRecognitionEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkRecognitionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkRecognitionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkRecognitionEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkRecognitionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkStrokesEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkStrokesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkStrokesEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IInkStrokesEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkStrokesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IMathInputControlEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IMathInputControlEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IMathInputControlEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IMathInputControlEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IMathInputControlEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IPenInputPanelEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IPenInputPanelEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IPenInputPanelEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IPenInputPanelEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as ::windows::core::Interface>::IID
    }
}
