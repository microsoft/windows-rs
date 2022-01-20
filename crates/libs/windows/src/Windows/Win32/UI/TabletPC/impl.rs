#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDynamicRenderer_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, benabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HWND(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&mut self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn ClipRectangle(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetClipRectangle(&mut self, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ClipRegion(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetClipRegion(&mut self, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&mut self, pida: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn DataCacheEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetDataCacheEnabled(&mut self, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseCachedData(&mut self, strokeid: u32) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Draw(&mut self, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDynamicRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>() -> IDynamicRenderer_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *benabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HWND() {
                ::core::result::Result::Ok(ok__) => {
                    *hwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn ClipRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClipRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *prccliprect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipRectangle(::core::mem::transmute_copy(&prccliprect)).into()
        }
        unsafe extern "system" fn ClipRegion<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClipRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *phcliprgn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRegion<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClipRegion(::core::mem::transmute_copy(&hcliprgn)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppida: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppida = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pida: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DrawingAttributes(::core::mem::transmute(&pida)).into()
        }
        unsafe extern "system" fn DataCacheEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataCacheEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcachedata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCacheEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataCacheEnabled(::core::mem::transmute_copy(&fcachedata)).into()
        }
        unsafe extern "system" fn ReleaseCachedData<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseCachedData(::core::mem::transmute_copy(&strokeid)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            HWND: HWND::<Identity, Impl, OFFSET>,
            SetHWND: SetHWND::<Identity, Impl, OFFSET>,
            ClipRectangle: ClipRectangle::<Identity, Impl, OFFSET>,
            SetClipRectangle: SetClipRectangle::<Identity, Impl, OFFSET>,
            ClipRegion: ClipRegion::<Identity, Impl, OFFSET>,
            SetClipRegion: SetClipRegion::<Identity, Impl, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, Impl, OFFSET>,
            DataCacheEnabled: DataCacheEnabled::<Identity, Impl, OFFSET>,
            SetDataCacheEnabled: SetDataCacheEnabled::<Identity, Impl, OFFSET>,
            ReleaseCachedData: ReleaseCachedData::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGestureRecognizer_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, fenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MaxStrokeCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxStrokeCount(&mut self, cstrokes: i32) -> ::windows::core::Result<()>;
    fn EnableGestures(&mut self, cgestures: u32, pgestures: *const i32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGestureRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>() -> IGestureRecognizer_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn MaxStrokeCount<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstrokes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxStrokeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcstrokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStrokeCount<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstrokes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxStrokeCount(::core::mem::transmute_copy(&cstrokes)).into()
        }
        unsafe extern "system" fn EnableGestures<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableGestures(::core::mem::transmute_copy(&cgestures), ::core::mem::transmute_copy(&pgestures)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            MaxStrokeCount: MaxStrokeCount::<Identity, Impl, OFFSET>,
            SetMaxStrokeCount: SetMaxStrokeCount::<Identity, Impl, OFFSET>,
            EnableGestures: EnableGestures::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGestureRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IHandwrittenTextInsertion_Impl: Sized {
    fn InsertRecognitionResultsArray(&mut self, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InsertInkRecognitionResult(&mut self, piinkrecoresult: &::core::option::Option<IInkRecognitionResult>, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IHandwrittenTextInsertion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>() -> IHandwrittenTextInsertion_Vtbl {
        unsafe extern "system" fn InsertRecognitionResultsArray<Identity: ::windows::core::IUnknownImpl, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertRecognitionResultsArray(::core::mem::transmute_copy(&psaalternates), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Identity: ::windows::core::IUnknownImpl, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkrecoresult: ::windows::core::RawPtr, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertInkRecognitionResult(::core::mem::transmute(&piinkrecoresult), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InsertRecognitionResultsArray: InsertRecognitionResultsArray::<Identity, Impl, OFFSET>,
            InsertInkRecognitionResult: InsertInkRecognitionResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHandwrittenTextInsertion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInk_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInk_Impl, const OFFSET: isize>() -> IInk_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCollector_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&mut self) -> ::windows::core::Result<isize>;
    fn SethWnd(&mut self, newwindow: isize) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, collecting: i16) -> ::windows::core::Result<()>;
    fn DefaultDrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&mut self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&mut self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&mut self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&mut self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&mut self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&mut self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&mut self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&mut self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&mut self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&mut self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&mut self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&mut self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn Cursors(&mut self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&mut self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&mut self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&mut self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&mut self, support: i16) -> ::windows::core::Result<()>;
    fn SetGestureStatus(&mut self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&mut self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&mut self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&mut self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&mut self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&mut self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&mut self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&mut self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>() -> IInkCollector_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *currentwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SethWnd(::core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *currentattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    *currentinkrenderer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    *autoredraw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *packetguids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredPacketDescription(::core::mem::transmute_copy(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *mouseicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    *mousepointer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    *cursors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    *marginx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    *marginy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    *singletablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    *support = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    *listening = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *listen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            SethWnd: SethWnd::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            Renderer: Renderer::<Identity, Impl, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            putref_Ink: putref_Ink::<Identity, Impl, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, Impl, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, Impl, OFFSET>,
            CollectingInk: CollectingInk::<Identity, Impl, OFFSET>,
            CollectionMode: CollectionMode::<Identity, Impl, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, Impl, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, Impl, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, Impl, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, Impl, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, Impl, OFFSET>,
            MouseIcon: MouseIcon::<Identity, Impl, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, Impl, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, Impl, OFFSET>,
            MousePointer: MousePointer::<Identity, Impl, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, Impl, OFFSET>,
            Cursors: Cursors::<Identity, Impl, OFFSET>,
            MarginX: MarginX::<Identity, Impl, OFFSET>,
            SetMarginX: SetMarginX::<Identity, Impl, OFFSET>,
            MarginY: MarginY::<Identity, Impl, OFFSET>,
            SetMarginY: SetMarginY::<Identity, Impl, OFFSET>,
            Tablet: Tablet::<Identity, Impl, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, Impl, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, Impl, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, Impl, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, Impl, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, Impl, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCollector as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Inverted(&mut self) -> ::windows::core::Result<i16>;
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&mut self, attributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Tablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn Buttons(&mut self) -> ::windows::core::Result<IInkCursorButtons>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>() -> IInkCursor_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverted<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Inverted() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DrawingAttributes(::core::mem::transmute(&attributes)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    *tablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buttons<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttons: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Buttons() {
                ::core::result::Result::Ok(ok__) => {
                    *buttons = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Inverted: Inverted::<Identity, Impl, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, Impl, OFFSET>,
            Tablet: Tablet::<Identity, Impl, OFFSET>,
            Buttons: Buttons::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursor as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorButton_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<InkCursorButtonState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButton_Impl, const OFFSET: isize>() -> IInkCursorButton_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *currentstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursorButton as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursorButtons_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkCursorButton>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButtons_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtons_Impl, const OFFSET: isize>() -> IInkCursorButtons_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *button = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursorButtons as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCursors_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkCursor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursors_Impl, const OFFSET: isize>() -> IInkCursors_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *cursor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCursors as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCustomStrokes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokes>;
    fn Add(&mut self, name: &super::super::Foundation::BSTR, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCustomStrokes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>() -> IInkCustomStrokes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&name), ::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCustomStrokes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDisp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn Dirty(&mut self) -> ::windows::core::Result<i16>;
    fn SetDirty(&mut self, dirty: i16) -> ::windows::core::Result<()>;
    fn CustomStrokes(&mut self) -> ::windows::core::Result<IInkCustomStrokes>;
    fn GetBoundingBox(&mut self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn DeleteStrokes(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn DeleteStroke(&mut self, stroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn ExtractStrokes(&mut self, strokes: &::core::option::Option<IInkStrokes>, extractflags: InkExtractFlags) -> ::windows::core::Result<IInkDisp>;
    fn ExtractWithRectangle(&mut self, rectangle: &::core::option::Option<IInkRectangle>, extractflags: InkExtractFlags) -> ::windows::core::Result<IInkDisp>;
    fn Clip(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn HitTestCircle(&mut self, x: i32, y: i32, radius: f32) -> ::windows::core::Result<IInkStrokes>;
    fn HitTestWithRectangle(&mut self, selectionrectangle: &::core::option::Option<IInkRectangle>, intersectpercent: f32) -> ::windows::core::Result<IInkStrokes>;
    fn HitTestWithLasso(&mut self, points: &super::super::System::Com::VARIANT, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn NearestPoint(&mut self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn CreateStrokes(&mut self, strokeids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokes>;
    fn AddStrokesAtRectangle(&mut self, sourcestrokes: &::core::option::Option<IInkStrokes>, targetrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Save(&mut self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Load(&mut self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CreateStroke(&mut self, packetdata: &super::super::System::Com::VARIANT, packetdescription: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokeDisp>;
    fn ClipboardCopyWithRectangle(&mut self, rectangle: &::core::option::Option<IInkRectangle>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn ClipboardCopy(&mut self, strokes: &::core::option::Option<IInkStrokes>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn CanPaste(&mut self, dataobject: &::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<i16>;
    fn ClipboardPaste(&mut self, x: i32, y: i32, dataobject: &::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<IInkStrokes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>() -> IInkDisp_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dirty<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Dirty() {
                ::core::result::Result::Ok(ok__) => {
                    *dirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirty<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDirty(::core::mem::transmute_copy(&dirty)).into()
        }
        unsafe extern "system" fn CustomStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkinkcustomstrokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CustomStrokes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkinkcustomstrokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteStrokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn DeleteStroke<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteStroke(::core::mem::transmute(&stroke)).into()
        }
        unsafe extern "system" fn ExtractStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtractStrokes(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&extractflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *extractedink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractWithRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtractWithRectangle(::core::mem::transmute(&rectangle), ::core::mem::transmute_copy(&extractflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *extractedink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *newink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestCircle<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HitTestCircle(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&radius)) {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionrectangle: ::windows::core::RawPtr, intersectpercent: f32, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HitTestWithRectangle(::core::mem::transmute(&selectionrectangle), ::core::mem::transmute_copy(&intersectpercent)) {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithLasso<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HitTestWithLasso(::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&intersectpercent), ::core::mem::transmute_copy(&lassopoints), ::core::mem::transmute_copy(&strokes)).into()
        }
        unsafe extern "system" fn NearestPoint<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NearestPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&pointonstroke), ::core::mem::transmute_copy(&distancefrompacket), ::core::mem::transmute_copy(&stroke)).into()
        }
        unsafe extern "system" fn CreateStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStrokes(::core::mem::transmute_copy(&strokeids)) {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStrokesAtRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestrokes: ::windows::core::RawPtr, targetrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStrokesAtRectangle(::core::mem::transmute(&sourcestrokes), ::core::mem::transmute(&targetrectangle)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Save(::core::mem::transmute_copy(&persistenceformat), ::core::mem::transmute_copy(&compressionmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn CreateStroke<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStroke(::core::mem::transmute_copy(&packetdata), ::core::mem::transmute_copy(&packetdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *stroke = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopyWithRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClipboardCopyWithRectangle(::core::mem::transmute(&rectangle), ::core::mem::transmute_copy(&clipboardformats), ::core::mem::transmute_copy(&clipboardmodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *dataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopy<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClipboardCopy(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&clipboardformats), ::core::mem::transmute_copy(&clipboardmodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *dataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataobject: ::windows::core::RawPtr, canpaste: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanPaste(::core::mem::transmute(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *canpaste = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardPaste<Identity: ::windows::core::IUnknownImpl, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, dataobject: ::windows::core::RawPtr, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClipboardPaste(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
            Dirty: Dirty::<Identity, Impl, OFFSET>,
            SetDirty: SetDirty::<Identity, Impl, OFFSET>,
            CustomStrokes: CustomStrokes::<Identity, Impl, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, Impl, OFFSET>,
            DeleteStrokes: DeleteStrokes::<Identity, Impl, OFFSET>,
            DeleteStroke: DeleteStroke::<Identity, Impl, OFFSET>,
            ExtractStrokes: ExtractStrokes::<Identity, Impl, OFFSET>,
            ExtractWithRectangle: ExtractWithRectangle::<Identity, Impl, OFFSET>,
            Clip: Clip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            HitTestCircle: HitTestCircle::<Identity, Impl, OFFSET>,
            HitTestWithRectangle: HitTestWithRectangle::<Identity, Impl, OFFSET>,
            HitTestWithLasso: HitTestWithLasso::<Identity, Impl, OFFSET>,
            NearestPoint: NearestPoint::<Identity, Impl, OFFSET>,
            CreateStrokes: CreateStrokes::<Identity, Impl, OFFSET>,
            AddStrokesAtRectangle: AddStrokesAtRectangle::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            CreateStroke: CreateStroke::<Identity, Impl, OFFSET>,
            ClipboardCopyWithRectangle: ClipboardCopyWithRectangle::<Identity, Impl, OFFSET>,
            ClipboardCopy: ClipboardCopy::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            ClipboardPaste: ClipboardPaste::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDisp as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivider_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn putref_Strokes(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn RecognizerContext(&mut self) -> ::windows::core::Result<IInkRecognizerContext>;
    fn putref_RecognizerContext(&mut self, recognizercontext: &::core::option::Option<IInkRecognizerContext>) -> ::windows::core::Result<()>;
    fn LineHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetLineHeight(&mut self, lineheight: i32) -> ::windows::core::Result<()>;
    fn Divide(&mut self) -> ::windows::core::Result<IInkDivisionResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>() -> IInkDivider_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Strokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn RecognizerContext<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecognizerContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizercontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RecognizerContext<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_RecognizerContext(::core::mem::transmute(&recognizercontext)).into()
        }
        unsafe extern "system" fn LineHeight<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *lineheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineHeight(::core::mem::transmute_copy(&lineheight)).into()
        }
        unsafe extern "system" fn Divide<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdivisionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Divide() {
                ::core::result::Result::Ok(ok__) => {
                    *inkdivisionresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            putref_Strokes: putref_Strokes::<Identity, Impl, OFFSET>,
            RecognizerContext: RecognizerContext::<Identity, Impl, OFFSET>,
            putref_RecognizerContext: putref_RecognizerContext::<Identity, Impl, OFFSET>,
            LineHeight: LineHeight::<Identity, Impl, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, Impl, OFFSET>,
            Divide: Divide::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivider as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn ResultByType(&mut self, divisiontype: InkDivisionType) -> ::windows::core::Result<IInkDivisionUnits>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionResult_Impl, const OFFSET: isize>() -> IInkDivisionResult_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultByType<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultByType(::core::mem::transmute_copy(&divisiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *inkdivisionunits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            ResultByType: ResultByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionResult as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionUnit_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn DivisionType(&mut self) -> ::windows::core::Result<InkDivisionType>;
    fn RecognizedString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RotationTransform(&mut self) -> ::windows::core::Result<IInkTransform>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>() -> IInkDivisionUnit_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DivisionType<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: *mut InkDivisionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DivisionType() {
                ::core::result::Result::Ok(ok__) => {
                    *divisiontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizedString<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecognizedString() {
                ::core::result::Result::Ok(ok__) => {
                    *recostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotationtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RotationTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *rotationtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            DivisionType: DivisionType::<Identity, Impl, OFFSET>,
            RecognizedString: RecognizedString::<Identity, Impl, OFFSET>,
            RotationTransform: RotationTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionUnit as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDivisionUnits_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkDivisionUnit>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>() -> IInkDivisionUnits_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkdivisionunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *inkdivisionunit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDivisionUnits as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkDrawingAttributes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Color(&mut self) -> ::windows::core::Result<i32>;
    fn SetColor(&mut self, newcolor: i32) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<f32>;
    fn SetWidth(&mut self, newwidth: f32) -> ::windows::core::Result<()>;
    fn Height(&mut self) -> ::windows::core::Result<f32>;
    fn SetHeight(&mut self, newheight: f32) -> ::windows::core::Result<()>;
    fn FitToCurve(&mut self) -> ::windows::core::Result<i16>;
    fn SetFitToCurve(&mut self, flag: i16) -> ::windows::core::Result<()>;
    fn IgnorePressure(&mut self) -> ::windows::core::Result<i16>;
    fn SetIgnorePressure(&mut self, flag: i16) -> ::windows::core::Result<()>;
    fn AntiAliased(&mut self) -> ::windows::core::Result<i16>;
    fn SetAntiAliased(&mut self, flag: i16) -> ::windows::core::Result<()>;
    fn Transparency(&mut self) -> ::windows::core::Result<i32>;
    fn SetTransparency(&mut self, newtransparency: i32) -> ::windows::core::Result<()>;
    fn RasterOperation(&mut self) -> ::windows::core::Result<InkRasterOperation>;
    fn SetRasterOperation(&mut self, newrasteroperation: InkRasterOperation) -> ::windows::core::Result<()>;
    fn PenTip(&mut self) -> ::windows::core::Result<InkPenTip>;
    fn SetPenTip(&mut self, newpentip: InkPenTip) -> ::windows::core::Result<()>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn Clone(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDrawingAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>() -> IInkDrawingAttributes_Vtbl {
        unsafe extern "system" fn Color<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcolor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *currentcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *currentwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&newwidth)).into()
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentheight: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *currentheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHeight(::core::mem::transmute_copy(&newheight)).into()
        }
        unsafe extern "system" fn FitToCurve<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FitToCurve() {
                ::core::result::Result::Ok(ok__) => {
                    *flag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFitToCurve<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFitToCurve(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn IgnorePressure<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IgnorePressure() {
                ::core::result::Result::Ok(ok__) => {
                    *flag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnorePressure<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIgnorePressure(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn AntiAliased<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AntiAliased() {
                ::core::result::Result::Ok(ok__) => {
                    *flag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiAliased<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAntiAliased(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn Transparency<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currenttransparency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Transparency() {
                ::core::result::Result::Ok(ok__) => {
                    *currenttransparency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransparency<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newtransparency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransparency(::core::mem::transmute_copy(&newtransparency)).into()
        }
        unsafe extern "system" fn RasterOperation<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RasterOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *currentrasteroperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterOperation<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newrasteroperation: InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRasterOperation(::core::mem::transmute_copy(&newrasteroperation)).into()
        }
        unsafe extern "system" fn PenTip<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpentip: *mut InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PenTip() {
                ::core::result::Result::Ok(ok__) => {
                    *currentpentip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTip<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpentip: InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPenTip(::core::mem::transmute_copy(&newpentip)).into()
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *drawingattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Color: Color::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            SetWidth: SetWidth::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            SetHeight: SetHeight::<Identity, Impl, OFFSET>,
            FitToCurve: FitToCurve::<Identity, Impl, OFFSET>,
            SetFitToCurve: SetFitToCurve::<Identity, Impl, OFFSET>,
            IgnorePressure: IgnorePressure::<Identity, Impl, OFFSET>,
            SetIgnorePressure: SetIgnorePressure::<Identity, Impl, OFFSET>,
            AntiAliased: AntiAliased::<Identity, Impl, OFFSET>,
            SetAntiAliased: SetAntiAliased::<Identity, Impl, OFFSET>,
            Transparency: Transparency::<Identity, Impl, OFFSET>,
            SetTransparency: SetTransparency::<Identity, Impl, OFFSET>,
            RasterOperation: RasterOperation::<Identity, Impl, OFFSET>,
            SetRasterOperation: SetRasterOperation::<Identity, Impl, OFFSET>,
            PenTip: PenTip::<Identity, Impl, OFFSET>,
            SetPenTip: SetPenTip::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDrawingAttributes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkEdit_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&mut self) -> ::windows::core::Result<InkEditStatus>;
    fn UseMouseForInput(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseMouseForInput(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn InkMode(&mut self) -> ::windows::core::Result<InkMode>;
    fn SetInkMode(&mut self, newval: InkMode) -> ::windows::core::Result<()>;
    fn InkInsertMode(&mut self) -> ::windows::core::Result<InkInsertMode>;
    fn SetInkInsertMode(&mut self, newval: InkInsertMode) -> ::windows::core::Result<()>;
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&mut self, newval: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn RecognitionTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetRecognitionTimeout(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn Recognizer(&mut self) -> ::windows::core::Result<IInkRecognizer>;
    fn putref_Recognizer(&mut self, newval: &::core::option::Option<IInkRecognizer>) -> ::windows::core::Result<()>;
    fn Factoid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelInks(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelInks(&mut self, selink: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelInksDisplayMode(&mut self) -> ::windows::core::Result<InkDisplayMode>;
    fn SetSelInksDisplayMode(&mut self, inkdisplaymode: InkDisplayMode) -> ::windows::core::Result<()>;
    fn Recognize(&mut self) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&mut self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn SetGestureStatus(&mut self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn SetBackColor(&mut self, clr: u32) -> ::windows::core::Result<()>;
    fn BackColor(&mut self) -> ::windows::core::Result<u32>;
    fn Appearance(&mut self) -> ::windows::core::Result<AppearanceConstants>;
    fn SetAppearance(&mut self, pappearance: AppearanceConstants) -> ::windows::core::Result<()>;
    fn BorderStyle(&mut self) -> ::windows::core::Result<BorderStyleConstants>;
    fn SetBorderStyle(&mut self, pborderstyle: BorderStyleConstants) -> ::windows::core::Result<()>;
    fn Hwnd(&mut self) -> ::windows::core::Result<u32>;
    fn Font(&mut self) -> ::windows::core::Result<super::super::System::Ole::IFontDisp>;
    fn putref_Font(&mut self, ppfont: &::core::option::Option<super::super::System::Ole::IFontDisp>) -> ::windows::core::Result<()>;
    fn Text(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetText(&mut self, pbstrtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MouseIcon(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&mut self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&mut self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn Locked(&mut self) -> ::windows::core::Result<i16>;
    fn SetLocked(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn MaxLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxLength(&mut self, lmaxlength: i32) -> ::windows::core::Result<()>;
    fn MultiLine(&mut self) -> ::windows::core::Result<i16>;
    fn SetMultiLine(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn ScrollBars(&mut self) -> ::windows::core::Result<ScrollBarsConstants>;
    fn SetScrollBars(&mut self, newval: ScrollBarsConstants) -> ::windows::core::Result<()>;
    fn DisableNoScroll(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisableNoScroll(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn SelAlignment(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelAlignment(&mut self, pvarselalignment: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelBold(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelBold(&mut self, pvarselbold: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelItalic(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelItalic(&mut self, pvarselitalic: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelUnderline(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelUnderline(&mut self, pvarselunderline: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelColor(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelColor(&mut self, pvarselcolor: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelFontName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelFontName(&mut self, pvarselfontname: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelFontSize(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelFontSize(&mut self, pvarselfontsize: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelCharOffset(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelCharOffset(&mut self, pvarselcharoffset: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TextRTF(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTextRTF(&mut self, pbstrtextrtf: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelStart(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelStart(&mut self, plselstart: i32) -> ::windows::core::Result<()>;
    fn SelLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelLength(&mut self, plsellength: i32) -> ::windows::core::Result<()>;
    fn SelText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSelText(&mut self, pbstrseltext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelRTF(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSelRTF(&mut self, pbstrselrtf: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkEdit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>() -> IInkEdit_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut InkEditStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseMouseForInput<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseMouseForInput() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseMouseForInput<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseMouseForInput(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InkMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInkMode(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkInsertMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InkInsertMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkInsertMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInkInsertMode(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DrawingAttributes(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn RecognitionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecognitionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecognitionTimeout(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Recognizer(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFactoid(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SelInks<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselink: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelInks() {
                ::core::result::Result::Ok(ok__) => {
                    *pselink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInks<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selink: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelInks(::core::mem::transmute_copy(&selink)).into()
        }
        unsafe extern "system" fn SelInksDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelInksDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pinkdisplaymode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInksDisplayMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelInksDisplayMode(::core::mem::transmute_copy(&inkdisplaymode)).into()
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Recognize().into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    *plisten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&clr)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pclr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: *mut AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *pappearance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppearance(::core::mem::transmute_copy(&pappearance)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pborderstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderStyle(::core::mem::transmute_copy(&pborderstyle)).into()
        }
        unsafe extern "system" fn Hwnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pohhwnd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pohhwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Font<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Font() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Font(::core::mem::transmute(&ppfont)).into()
        }
        unsafe extern "system" fn Text<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *mouseicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    *mousepointer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Locked<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Locked() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocked<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocked(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxLength<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLength<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxLength(::core::mem::transmute_copy(&lmaxlength)).into()
        }
        unsafe extern "system" fn MultiLine<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultiLine() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiLine<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiLine(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ScrollBars<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScrollBars() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollBars<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScrollBars(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DisableNoScroll<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisableNoScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableNoScroll<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisableNoScroll(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SelAlignment<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselalignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelAlignment<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelAlignment(::core::mem::transmute_copy(&pvarselalignment)).into()
        }
        unsafe extern "system" fn SelBold<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelBold() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselbold = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelBold<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelBold(::core::mem::transmute_copy(&pvarselbold)).into()
        }
        unsafe extern "system" fn SelItalic<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelItalic() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselitalic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelItalic<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelItalic(::core::mem::transmute_copy(&pvarselitalic)).into()
        }
        unsafe extern "system" fn SelUnderline<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelUnderline() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselunderline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelUnderline<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelUnderline(::core::mem::transmute_copy(&pvarselunderline)).into()
        }
        unsafe extern "system" fn SelColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelColor(::core::mem::transmute_copy(&pvarselcolor)).into()
        }
        unsafe extern "system" fn SelFontName<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelFontName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselfontname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontName<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelFontName(::core::mem::transmute_copy(&pvarselfontname)).into()
        }
        unsafe extern "system" fn SelFontSize<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelFontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselfontsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontSize<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelFontSize(::core::mem::transmute_copy(&pvarselfontsize)).into()
        }
        unsafe extern "system" fn SelCharOffset<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelCharOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselcharoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelCharOffset<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelCharOffset(::core::mem::transmute_copy(&pvarselcharoffset)).into()
        }
        unsafe extern "system" fn TextRTF<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextRTF() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtextrtf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRTF<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextRTF(::core::mem::transmute_copy(&pbstrtextrtf)).into()
        }
        unsafe extern "system" fn SelStart<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelStart() {
                ::core::result::Result::Ok(ok__) => {
                    *plselstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelStart<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelStart(::core::mem::transmute_copy(&plselstart)).into()
        }
        unsafe extern "system" fn SelLength<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelLength() {
                ::core::result::Result::Ok(ok__) => {
                    *plsellength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelLength<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelLength(::core::mem::transmute_copy(&plsellength)).into()
        }
        unsafe extern "system" fn SelText<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrseltext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelText<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelText(::core::mem::transmute_copy(&pbstrseltext)).into()
        }
        unsafe extern "system" fn SelRTF<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelRTF() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrselrtf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelRTF<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelRTF(::core::mem::transmute_copy(&pbstrselrtf)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            UseMouseForInput: UseMouseForInput::<Identity, Impl, OFFSET>,
            SetUseMouseForInput: SetUseMouseForInput::<Identity, Impl, OFFSET>,
            InkMode: InkMode::<Identity, Impl, OFFSET>,
            SetInkMode: SetInkMode::<Identity, Impl, OFFSET>,
            InkInsertMode: InkInsertMode::<Identity, Impl, OFFSET>,
            SetInkInsertMode: SetInkInsertMode::<Identity, Impl, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, Impl, OFFSET>,
            RecognitionTimeout: RecognitionTimeout::<Identity, Impl, OFFSET>,
            SetRecognitionTimeout: SetRecognitionTimeout::<Identity, Impl, OFFSET>,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            putref_Recognizer: putref_Recognizer::<Identity, Impl, OFFSET>,
            Factoid: Factoid::<Identity, Impl, OFFSET>,
            SetFactoid: SetFactoid::<Identity, Impl, OFFSET>,
            SelInks: SelInks::<Identity, Impl, OFFSET>,
            SetSelInks: SetSelInks::<Identity, Impl, OFFSET>,
            SelInksDisplayMode: SelInksDisplayMode::<Identity, Impl, OFFSET>,
            SetSelInksDisplayMode: SetSelInksDisplayMode::<Identity, Impl, OFFSET>,
            Recognize: Recognize::<Identity, Impl, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, Impl, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            BackColor: BackColor::<Identity, Impl, OFFSET>,
            Appearance: Appearance::<Identity, Impl, OFFSET>,
            SetAppearance: SetAppearance::<Identity, Impl, OFFSET>,
            BorderStyle: BorderStyle::<Identity, Impl, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, Impl, OFFSET>,
            Hwnd: Hwnd::<Identity, Impl, OFFSET>,
            Font: Font::<Identity, Impl, OFFSET>,
            putref_Font: putref_Font::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            MouseIcon: MouseIcon::<Identity, Impl, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, Impl, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, Impl, OFFSET>,
            MousePointer: MousePointer::<Identity, Impl, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, Impl, OFFSET>,
            Locked: Locked::<Identity, Impl, OFFSET>,
            SetLocked: SetLocked::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            MaxLength: MaxLength::<Identity, Impl, OFFSET>,
            SetMaxLength: SetMaxLength::<Identity, Impl, OFFSET>,
            MultiLine: MultiLine::<Identity, Impl, OFFSET>,
            SetMultiLine: SetMultiLine::<Identity, Impl, OFFSET>,
            ScrollBars: ScrollBars::<Identity, Impl, OFFSET>,
            SetScrollBars: SetScrollBars::<Identity, Impl, OFFSET>,
            DisableNoScroll: DisableNoScroll::<Identity, Impl, OFFSET>,
            SetDisableNoScroll: SetDisableNoScroll::<Identity, Impl, OFFSET>,
            SelAlignment: SelAlignment::<Identity, Impl, OFFSET>,
            SetSelAlignment: SetSelAlignment::<Identity, Impl, OFFSET>,
            SelBold: SelBold::<Identity, Impl, OFFSET>,
            SetSelBold: SetSelBold::<Identity, Impl, OFFSET>,
            SelItalic: SelItalic::<Identity, Impl, OFFSET>,
            SetSelItalic: SetSelItalic::<Identity, Impl, OFFSET>,
            SelUnderline: SelUnderline::<Identity, Impl, OFFSET>,
            SetSelUnderline: SetSelUnderline::<Identity, Impl, OFFSET>,
            SelColor: SelColor::<Identity, Impl, OFFSET>,
            SetSelColor: SetSelColor::<Identity, Impl, OFFSET>,
            SelFontName: SelFontName::<Identity, Impl, OFFSET>,
            SetSelFontName: SetSelFontName::<Identity, Impl, OFFSET>,
            SelFontSize: SelFontSize::<Identity, Impl, OFFSET>,
            SetSelFontSize: SetSelFontSize::<Identity, Impl, OFFSET>,
            SelCharOffset: SelCharOffset::<Identity, Impl, OFFSET>,
            SetSelCharOffset: SetSelCharOffset::<Identity, Impl, OFFSET>,
            TextRTF: TextRTF::<Identity, Impl, OFFSET>,
            SetTextRTF: SetTextRTF::<Identity, Impl, OFFSET>,
            SelStart: SelStart::<Identity, Impl, OFFSET>,
            SetSelStart: SetSelStart::<Identity, Impl, OFFSET>,
            SelLength: SelLength::<Identity, Impl, OFFSET>,
            SetSelLength: SetSelLength::<Identity, Impl, OFFSET>,
            SelText: SelText::<Identity, Impl, OFFSET>,
            SetSelText: SetSelText::<Identity, Impl, OFFSET>,
            SelRTF: SelRTF::<Identity, Impl, OFFSET>,
            SetSelRTF: SetSelRTF::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkEdit as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkExtendedProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkExtendedProperty>;
    fn Add(&mut self, guid: &super::super::Foundation::BSTR, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkExtendedProperty>;
    fn Remove(&mut self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn DoesPropertyExist(&mut self, guid: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>() -> IInkExtendedProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *inkextendedproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn DoesPropertyExist<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoesPropertyExist(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    *doespropertyexist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            DoesPropertyExist: DoesPropertyExist::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkExtendedProperties as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkExtendedProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Guid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetData(&mut self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>() -> IInkExtendedProperty_Vtbl {
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Guid: Guid::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkExtendedProperty as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkGesture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Confidence(&mut self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Id(&mut self) -> ::windows::core::Result<InkApplicationGesture>;
    fn GetHotPoint(&mut self, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkGesture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkGesture_Impl, const OFFSET: isize>() -> IInkGesture_Vtbl {
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut InkApplicationGesture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotPoint<Identity: ::windows::core::IUnknownImpl, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHotPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            GetHotPoint: GetHotPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkGesture as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInkLineInfo_Impl: Sized {
    fn SetFormat(&mut self, pim: *const INKMETRIC) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, pim: *const INKMETRIC) -> ::windows::core::Result<()>;
    fn GetInkExtent(&mut self, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::Result<()>;
    fn GetCandidate(&mut self, ncandidatenum: u32, pwcrecogword: super::super::Foundation::PWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetCandidate(&mut self, ncandidatenum: u32, strrecogword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Recognize(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInkLineInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>() -> IInkLineInfo_Vtbl {
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetInkExtent<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInkExtent(::core::mem::transmute_copy(&pim), ::core::mem::transmute_copy(&pnwidth)).into()
        }
        unsafe extern "system" fn GetCandidate<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, pwcrecogword: super::super::Foundation::PWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidate(::core::mem::transmute_copy(&ncandidatenum), ::core::mem::transmute_copy(&pwcrecogword), ::core::mem::transmute_copy(&pcwcrecogword), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCandidate<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, strrecogword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCandidate(::core::mem::transmute_copy(&ncandidatenum), ::core::mem::transmute_copy(&strrecogword)).into()
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Recognize().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetInkExtent: GetInkExtent::<Identity, Impl, OFFSET>,
            GetCandidate: GetCandidate::<Identity, Impl, OFFSET>,
            SetCandidate: SetCandidate::<Identity, Impl, OFFSET>,
            Recognize: Recognize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkLineInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkOverlay_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&mut self) -> ::windows::core::Result<isize>;
    fn SethWnd(&mut self, newwindow: isize) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, collecting: i16) -> ::windows::core::Result<()>;
    fn DefaultDrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&mut self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&mut self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&mut self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&mut self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&mut self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&mut self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&mut self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&mut self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&mut self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&mut self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&mut self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&mut self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn EditingMode(&mut self) -> ::windows::core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&mut self, editingmode: InkOverlayEditingMode) -> ::windows::core::Result<()>;
    fn Selection(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn SetSelection(&mut self, selection: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn EraserMode(&mut self) -> ::windows::core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&mut self, erasermode: InkOverlayEraserMode) -> ::windows::core::Result<()>;
    fn EraserWidth(&mut self) -> ::windows::core::Result<i32>;
    fn SetEraserWidth(&mut self, neweraserwidth: i32) -> ::windows::core::Result<()>;
    fn AttachMode(&mut self) -> ::windows::core::Result<InkOverlayAttachMode>;
    fn SetAttachMode(&mut self, attachmode: InkOverlayAttachMode) -> ::windows::core::Result<()>;
    fn Cursors(&mut self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&mut self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&mut self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&mut self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&mut self, support: i16) -> ::windows::core::Result<()>;
    fn SupportHighContrastSelectionUI(&mut self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastSelectionUI(&mut self, support: i16) -> ::windows::core::Result<()>;
    fn HitTestSelection(&mut self, x: i32, y: i32) -> ::windows::core::Result<SelectionHitResult>;
    fn Draw(&mut self, rect: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetGestureStatus(&mut self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&mut self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&mut self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&mut self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&mut self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&mut self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&mut self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&mut self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkOverlay_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>() -> IInkOverlay_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *currentwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SethWnd(::core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *currentattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    *currentinkrenderer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    *autoredraw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *packetguids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredPacketDescription(::core::mem::transmute_copy(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *mouseicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    *mousepointer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EditingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *editingmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEditingMode(::core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *selection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelection(::core::mem::transmute(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EraserMode() {
                ::core::result::Result::Ok(ok__) => {
                    *erasermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEraserMode(::core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EraserWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *eraserwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEraserWidth(::core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn AttachMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttachMode() {
                ::core::result::Result::Ok(ok__) => {
                    *attachmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachMode(::core::mem::transmute_copy(&attachmode)).into()
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    *cursors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    *marginx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    *marginy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    *singletablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    *support = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportHighContrastSelectionUI() {
                ::core::result::Result::Ok(ok__) => {
                    *support = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HitTestSelection(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *selarea = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute(&rect)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    *listening = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *listen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            SethWnd: SethWnd::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            Renderer: Renderer::<Identity, Impl, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            putref_Ink: putref_Ink::<Identity, Impl, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, Impl, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, Impl, OFFSET>,
            CollectingInk: CollectingInk::<Identity, Impl, OFFSET>,
            CollectionMode: CollectionMode::<Identity, Impl, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, Impl, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, Impl, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, Impl, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, Impl, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, Impl, OFFSET>,
            MouseIcon: MouseIcon::<Identity, Impl, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, Impl, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, Impl, OFFSET>,
            MousePointer: MousePointer::<Identity, Impl, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, Impl, OFFSET>,
            EditingMode: EditingMode::<Identity, Impl, OFFSET>,
            SetEditingMode: SetEditingMode::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            EraserMode: EraserMode::<Identity, Impl, OFFSET>,
            SetEraserMode: SetEraserMode::<Identity, Impl, OFFSET>,
            EraserWidth: EraserWidth::<Identity, Impl, OFFSET>,
            SetEraserWidth: SetEraserWidth::<Identity, Impl, OFFSET>,
            AttachMode: AttachMode::<Identity, Impl, OFFSET>,
            SetAttachMode: SetAttachMode::<Identity, Impl, OFFSET>,
            Cursors: Cursors::<Identity, Impl, OFFSET>,
            MarginX: MarginX::<Identity, Impl, OFFSET>,
            SetMarginX: SetMarginX::<Identity, Impl, OFFSET>,
            MarginY: MarginY::<Identity, Impl, OFFSET>,
            SetMarginY: SetMarginY::<Identity, Impl, OFFSET>,
            Tablet: Tablet::<Identity, Impl, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, Impl, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, Impl, OFFSET>,
            SupportHighContrastSelectionUI: SupportHighContrastSelectionUI::<Identity, Impl, OFFSET>,
            SetSupportHighContrastSelectionUI: SetSupportHighContrastSelectionUI::<Identity, Impl, OFFSET>,
            HitTestSelection: HitTestSelection::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, Impl, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, Impl, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, Impl, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkOverlay as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkPicture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&mut self) -> ::windows::core::Result<isize>;
    fn DefaultDrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&mut self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&mut self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&mut self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&mut self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&mut self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&mut self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&mut self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&mut self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&mut self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&mut self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&mut self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&mut self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&mut self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn EditingMode(&mut self) -> ::windows::core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&mut self, editingmode: InkOverlayEditingMode) -> ::windows::core::Result<()>;
    fn Selection(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn SetSelection(&mut self, selection: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn EraserMode(&mut self) -> ::windows::core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&mut self, erasermode: InkOverlayEraserMode) -> ::windows::core::Result<()>;
    fn EraserWidth(&mut self) -> ::windows::core::Result<i32>;
    fn SetEraserWidth(&mut self, neweraserwidth: i32) -> ::windows::core::Result<()>;
    fn putref_Picture(&mut self, ppicture: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn SetPicture(&mut self, ppicture: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn Picture(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetSizeMode(&mut self, smnewsizemode: InkPictureSizeMode) -> ::windows::core::Result<()>;
    fn SizeMode(&mut self) -> ::windows::core::Result<InkPictureSizeMode>;
    fn SetBackColor(&mut self, newcolor: u32) -> ::windows::core::Result<()>;
    fn BackColor(&mut self) -> ::windows::core::Result<u32>;
    fn Cursors(&mut self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&mut self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&mut self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&mut self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&mut self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&mut self, support: i16) -> ::windows::core::Result<()>;
    fn SupportHighContrastSelectionUI(&mut self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastSelectionUI(&mut self, support: i16) -> ::windows::core::Result<()>;
    fn HitTestSelection(&mut self, x: i32, y: i32) -> ::windows::core::Result<SelectionHitResult>;
    fn SetGestureStatus(&mut self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&mut self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&mut self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&mut self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&mut self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&mut self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&mut self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&mut self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
    fn InkEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetInkEnabled(&mut self, collecting: i16) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, vbool: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkPicture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>() -> IInkPicture_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *currentwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *currentattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    *currentinkrenderer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    *autoredraw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *packetguids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredPacketDescription(::core::mem::transmute_copy(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *mouseicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    *mousepointer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EditingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *editingmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEditingMode(::core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *selection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelection(::core::mem::transmute(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EraserMode() {
                ::core::result::Result::Ok(ok__) => {
                    *erasermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEraserMode(::core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EraserWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *eraserwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEraserWidth(::core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn putref_Picture<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Picture(::core::mem::transmute(&ppicture)).into()
        }
        unsafe extern "system" fn SetPicture<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPicture(::core::mem::transmute(&ppicture)).into()
        }
        unsafe extern "system" fn Picture<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppicture: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Picture() {
                ::core::result::Result::Ok(ok__) => {
                    *pppicture = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSizeMode(::core::mem::transmute_copy(&smnewsizemode)).into()
        }
        unsafe extern "system" fn SizeMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SizeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *smsizemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    *cursors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    *marginx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    *marginy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    *singletablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    *support = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportHighContrastSelectionUI() {
                ::core::result::Result::Ok(ok__) => {
                    *support = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HitTestSelection(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *selarea = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    *listening = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *listen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn InkEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InkEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *collecting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInkEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbool: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&vbool)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, Impl, OFFSET>,
            Renderer: Renderer::<Identity, Impl, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            putref_Ink: putref_Ink::<Identity, Impl, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, Impl, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, Impl, OFFSET>,
            CollectingInk: CollectingInk::<Identity, Impl, OFFSET>,
            CollectionMode: CollectionMode::<Identity, Impl, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, Impl, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, Impl, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, Impl, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, Impl, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, Impl, OFFSET>,
            MouseIcon: MouseIcon::<Identity, Impl, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, Impl, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, Impl, OFFSET>,
            MousePointer: MousePointer::<Identity, Impl, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, Impl, OFFSET>,
            EditingMode: EditingMode::<Identity, Impl, OFFSET>,
            SetEditingMode: SetEditingMode::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            EraserMode: EraserMode::<Identity, Impl, OFFSET>,
            SetEraserMode: SetEraserMode::<Identity, Impl, OFFSET>,
            EraserWidth: EraserWidth::<Identity, Impl, OFFSET>,
            SetEraserWidth: SetEraserWidth::<Identity, Impl, OFFSET>,
            putref_Picture: putref_Picture::<Identity, Impl, OFFSET>,
            SetPicture: SetPicture::<Identity, Impl, OFFSET>,
            Picture: Picture::<Identity, Impl, OFFSET>,
            SetSizeMode: SetSizeMode::<Identity, Impl, OFFSET>,
            SizeMode: SizeMode::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            BackColor: BackColor::<Identity, Impl, OFFSET>,
            Cursors: Cursors::<Identity, Impl, OFFSET>,
            MarginX: MarginX::<Identity, Impl, OFFSET>,
            SetMarginX: SetMarginX::<Identity, Impl, OFFSET>,
            MarginY: MarginY::<Identity, Impl, OFFSET>,
            SetMarginY: SetMarginY::<Identity, Impl, OFFSET>,
            Tablet: Tablet::<Identity, Impl, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, Impl, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, Impl, OFFSET>,
            SupportHighContrastSelectionUI: SupportHighContrastSelectionUI::<Identity, Impl, OFFSET>,
            SetSupportHighContrastSelectionUI: SetSupportHighContrastSelectionUI::<Identity, Impl, OFFSET>,
            HitTestSelection: HitTestSelection::<Identity, Impl, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, Impl, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, Impl, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, Impl, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, Impl, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, Impl, OFFSET>,
            InkEnabled: InkEnabled::<Identity, Impl, OFFSET>,
            SetInkEnabled: SetInkEnabled::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPicture as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognitionAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn String(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Confidence(&mut self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Baseline(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Midline(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Ascender(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Descender(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LineNumber(&mut self) -> ::windows::core::Result<i32>;
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn LineAlternates(&mut self) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn ConfidenceAlternates(&mut self) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn GetStrokesFromStrokeRanges(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<IInkStrokes>;
    fn GetStrokesFromTextRange(&mut self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn GetTextRangeFromStrokes(&mut self, strokes: &::core::option::Option<IInkStrokes>, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::Result<()>;
    fn AlternatesWithConstantPropertyValues(&mut self, propertytype: &super::super::Foundation::BSTR) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn GetPropertyValue(&mut self, propertytype: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>() -> IInkRecognitionAlternate_Vtbl {
        unsafe extern "system" fn String<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).String() {
                ::core::result::Result::Ok(ok__) => {
                    *recostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Baseline() {
                ::core::result::Result::Ok(ok__) => {
                    *baseline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Midline<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Midline() {
                ::core::result::Result::Ok(ok__) => {
                    *midline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascender<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ascender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ascender() {
                ::core::result::Result::Ok(ok__) => {
                    *ascender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descender<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Descender() {
                ::core::result::Result::Ok(ok__) => {
                    *descender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *linenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineAlternates<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *linealternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfidenceAlternates<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidencealternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConfidenceAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *confidencealternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromStrokeRanges<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, getstrokesfromstrokeranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStrokesFromStrokeRanges(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *getstrokesfromstrokeranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromTextRange<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStrokesFromTextRange(::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength), ::core::mem::transmute_copy(&getstrokesfromtextrange)).into()
        }
        unsafe extern "system" fn GetTextRangeFromStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTextRangeFromStrokes(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn AlternatesWithConstantPropertyValues<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AlternatesWithConstantPropertyValues(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternateswithconstantpropertyvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyValue(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            String: String::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Baseline: Baseline::<Identity, Impl, OFFSET>,
            Midline: Midline::<Identity, Impl, OFFSET>,
            Ascender: Ascender::<Identity, Impl, OFFSET>,
            Descender: Descender::<Identity, Impl, OFFSET>,
            LineNumber: LineNumber::<Identity, Impl, OFFSET>,
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            LineAlternates: LineAlternates::<Identity, Impl, OFFSET>,
            ConfidenceAlternates: ConfidenceAlternates::<Identity, Impl, OFFSET>,
            GetStrokesFromStrokeRanges: GetStrokesFromStrokeRanges::<Identity, Impl, OFFSET>,
            GetStrokesFromTextRange: GetStrokesFromTextRange::<Identity, Impl, OFFSET>,
            GetTextRangeFromStrokes: GetTextRangeFromStrokes::<Identity, Impl, OFFSET>,
            AlternatesWithConstantPropertyValues: AlternatesWithConstantPropertyValues::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionAlternate as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognitionAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkRecognitionAlternate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>() -> IInkRecognitionAlternates_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecoalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *inkrecoalternate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionAlternates as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognitionResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TopString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TopAlternate(&mut self) -> ::windows::core::Result<IInkRecognitionAlternate>;
    fn TopConfidence(&mut self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn AlternatesFromSelection(&mut self, selectionstart: i32, selectionlength: i32, maximumalternates: i32) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn ModifyTopAlternate(&mut self, alternate: &::core::option::Option<IInkRecognitionAlternate>) -> ::windows::core::Result<()>;
    fn SetResultOnStrokes(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>() -> IInkRecognitionResult_Vtbl {
        unsafe extern "system" fn TopString<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TopString() {
                ::core::result::Result::Ok(ok__) => {
                    *topstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopAlternate<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topalternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TopAlternate() {
                ::core::result::Result::Ok(ok__) => {
                    *topalternate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopConfidence<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TopConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *topconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternatesFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AlternatesFromSelection(::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength), ::core::mem::transmute_copy(&maximumalternates)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternatesfromselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTopAlternate<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alternate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyTopAlternate(::core::mem::transmute(&alternate)).into()
        }
        unsafe extern "system" fn SetResultOnStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResultOnStrokes().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TopString: TopString::<Identity, Impl, OFFSET>,
            TopAlternate: TopAlternate::<Identity, Impl, OFFSET>,
            TopConfidence: TopConfidence::<Identity, Impl, OFFSET>,
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            AlternatesFromSelection: AlternatesFromSelection::<Identity, Impl, OFFSET>,
            ModifyTopAlternate: ModifyTopAlternate::<Identity, Impl, OFFSET>,
            SetResultOnStrokes: SetResultOnStrokes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognitionResult as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Vendor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Capabilities(&mut self) -> ::windows::core::Result<InkRecognizerCapabilities>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SupportedProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PreferredPacketDescription(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreateRecognizerContext(&mut self) -> ::windows::core::Result<IInkRecognizerContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>() -> IInkRecognizer_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    *vendor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *capabilitiesflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *languages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedproperties: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredpacketdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreferredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *preferredpacketdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecognizerContext<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRecognizerContext() {
                ::core::result::Result::Ok(ok__) => {
                    *context = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Vendor: Vendor::<Identity, Impl, OFFSET>,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            SupportedProperties: SupportedProperties::<Identity, Impl, OFFSET>,
            PreferredPacketDescription: PreferredPacketDescription::<Identity, Impl, OFFSET>,
            CreateRecognizerContext: CreateRecognizerContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizer2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UnicodeRanges(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer2_Impl, const OFFSET: isize>() -> IInkRecognizer2_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeRanges<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnicodeRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *unicoderanges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            UnicodeRanges: UnicodeRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizer2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizerContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&mut self) -> ::windows::core::Result<IInkStrokes>;
    fn putref_Strokes(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn CharacterAutoCompletionMode(&mut self) -> ::windows::core::Result<InkRecognizerCharacterAutoCompletionMode>;
    fn SetCharacterAutoCompletionMode(&mut self, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::Result<()>;
    fn Factoid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&mut self, factoid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Guide(&mut self) -> ::windows::core::Result<IInkRecognizerGuide>;
    fn putref_Guide(&mut self, recognizerguide: &::core::option::Option<IInkRecognizerGuide>) -> ::windows::core::Result<()>;
    fn PrefixText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrefixText(&mut self, prefix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SuffixText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSuffixText(&mut self, suffix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RecognitionFlags(&mut self) -> ::windows::core::Result<InkRecognitionModes>;
    fn SetRecognitionFlags(&mut self, modes: InkRecognitionModes) -> ::windows::core::Result<()>;
    fn WordList(&mut self) -> ::windows::core::Result<IInkWordList>;
    fn putref_WordList(&mut self, wordlist: &::core::option::Option<IInkWordList>) -> ::windows::core::Result<()>;
    fn Recognizer(&mut self) -> ::windows::core::Result<IInkRecognizer>;
    fn Recognize(&mut self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::core::option::Option<IInkRecognitionResult>) -> ::windows::core::Result<()>;
    fn StopBackgroundRecognition(&mut self) -> ::windows::core::Result<()>;
    fn EndInkInput(&mut self) -> ::windows::core::Result<()>;
    fn BackgroundRecognize(&mut self, customdata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BackgroundRecognizeWithAlternates(&mut self, customdata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IInkRecognizerContext>;
    fn IsStringSupported(&mut self, string: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>() -> IInkRecognizerContext_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Strokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn CharacterAutoCompletionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CharacterAutoCompletionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterAutoCompletionMode<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCharacterAutoCompletionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    *factoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFactoid(::core::mem::transmute_copy(&factoid)).into()
        }
        unsafe extern "system" fn Guide<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guide() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizerguide = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Guide<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Guide(::core::mem::transmute(&recognizerguide)).into()
        }
        unsafe extern "system" fn PrefixText<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrefixText() {
                ::core::result::Result::Ok(ok__) => {
                    *prefix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixText<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrefixText(::core::mem::transmute_copy(&prefix)).into()
        }
        unsafe extern "system" fn SuffixText<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SuffixText() {
                ::core::result::Result::Ok(ok__) => {
                    *suffix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuffixText<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSuffixText(::core::mem::transmute_copy(&suffix)).into()
        }
        unsafe extern "system" fn RecognitionFlags<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: *mut InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecognitionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *modes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionFlags<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecognitionFlags(::core::mem::transmute_copy(&modes)).into()
        }
        unsafe extern "system" fn WordList<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WordList() {
                ::core::result::Result::Ok(ok__) => {
                    *wordlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_WordList<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_WordList(::core::mem::transmute(&wordlist)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Recognize(::core::mem::transmute_copy(&recognitionstatus), ::core::mem::transmute_copy(&recognitionresult)).into()
        }
        unsafe extern "system" fn StopBackgroundRecognition<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopBackgroundRecognition().into()
        }
        unsafe extern "system" fn EndInkInput<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndInkInput().into()
        }
        unsafe extern "system" fn BackgroundRecognize<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundRecognize(::core::mem::transmute_copy(&customdata)).into()
        }
        unsafe extern "system" fn BackgroundRecognizeWithAlternates<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundRecognizeWithAlternates(::core::mem::transmute_copy(&customdata)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStringSupported<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsStringSupported(::core::mem::transmute_copy(&string)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Strokes: Strokes::<Identity, Impl, OFFSET>,
            putref_Strokes: putref_Strokes::<Identity, Impl, OFFSET>,
            CharacterAutoCompletionMode: CharacterAutoCompletionMode::<Identity, Impl, OFFSET>,
            SetCharacterAutoCompletionMode: SetCharacterAutoCompletionMode::<Identity, Impl, OFFSET>,
            Factoid: Factoid::<Identity, Impl, OFFSET>,
            SetFactoid: SetFactoid::<Identity, Impl, OFFSET>,
            Guide: Guide::<Identity, Impl, OFFSET>,
            putref_Guide: putref_Guide::<Identity, Impl, OFFSET>,
            PrefixText: PrefixText::<Identity, Impl, OFFSET>,
            SetPrefixText: SetPrefixText::<Identity, Impl, OFFSET>,
            SuffixText: SuffixText::<Identity, Impl, OFFSET>,
            SetSuffixText: SetSuffixText::<Identity, Impl, OFFSET>,
            RecognitionFlags: RecognitionFlags::<Identity, Impl, OFFSET>,
            SetRecognitionFlags: SetRecognitionFlags::<Identity, Impl, OFFSET>,
            WordList: WordList::<Identity, Impl, OFFSET>,
            putref_WordList: putref_WordList::<Identity, Impl, OFFSET>,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            Recognize: Recognize::<Identity, Impl, OFFSET>,
            StopBackgroundRecognition: StopBackgroundRecognition::<Identity, Impl, OFFSET>,
            EndInkInput: EndInkInput::<Identity, Impl, OFFSET>,
            BackgroundRecognize: BackgroundRecognize::<Identity, Impl, OFFSET>,
            BackgroundRecognizeWithAlternates: BackgroundRecognizeWithAlternates::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            IsStringSupported: IsStringSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizerContext2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnabledUnicodeRanges(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetEnabledUnicodeRanges(&mut self, unicoderanges: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>() -> IInkRecognizerContext2_Vtbl {
        unsafe extern "system" fn EnabledUnicodeRanges<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnabledUnicodeRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *unicoderanges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledUnicodeRanges<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabledUnicodeRanges(::core::mem::transmute_copy(&unicoderanges)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnabledUnicodeRanges: EnabledUnicodeRanges::<Identity, Impl, OFFSET>,
            SetEnabledUnicodeRanges: SetEnabledUnicodeRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerContext2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizerGuide_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WritingBox(&mut self) -> ::windows::core::Result<IInkRectangle>;
    fn SetWritingBox(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn DrawnBox(&mut self) -> ::windows::core::Result<IInkRectangle>;
    fn SetDrawnBox(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Rows(&mut self) -> ::windows::core::Result<i32>;
    fn SetRows(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Columns(&mut self) -> ::windows::core::Result<i32>;
    fn SetColumns(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Midline(&mut self) -> ::windows::core::Result<i32>;
    fn SetMidline(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn GuideData(&mut self) -> ::windows::core::Result<InkRecoGuide>;
    fn SetGuideData(&mut self, recoguide: &InkRecoGuide) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerGuide_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>() -> IInkRecognizerGuide_Vtbl {
        unsafe extern "system" fn WritingBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WritingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWritingBox(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn DrawnBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DrawnBox() {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawnBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDrawnBox(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Rows<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rows() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRows<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRows(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Columns<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Columns() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumns<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColumns(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Midline<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Midline() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMidline<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMidline(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn GuideData<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoguide: *mut InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GuideData() {
                ::core::result::Result::Ok(ok__) => {
                    *precoguide = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuideData<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoguide: InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuideData(::core::mem::transmute_copy(&recoguide)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WritingBox: WritingBox::<Identity, Impl, OFFSET>,
            SetWritingBox: SetWritingBox::<Identity, Impl, OFFSET>,
            DrawnBox: DrawnBox::<Identity, Impl, OFFSET>,
            SetDrawnBox: SetDrawnBox::<Identity, Impl, OFFSET>,
            Rows: Rows::<Identity, Impl, OFFSET>,
            SetRows: SetRows::<Identity, Impl, OFFSET>,
            Columns: Columns::<Identity, Impl, OFFSET>,
            SetColumns: SetColumns::<Identity, Impl, OFFSET>,
            Midline: Midline::<Identity, Impl, OFFSET>,
            SetMidline: SetMidline::<Identity, Impl, OFFSET>,
            GuideData: GuideData::<Identity, Impl, OFFSET>,
            SetGuideData: SetGuideData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizerGuide as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRecognizers_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDefaultRecognizer(&mut self, lcid: i32) -> ::windows::core::Result<IInkRecognizer>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkRecognizer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizers_Impl, const OFFSET: isize>() -> IInkRecognizers_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultRecognizer<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: i32, defaultrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultRecognizer(::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *defaultrecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *inkrecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            GetDefaultRecognizer: GetDefaultRecognizer::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRecognizers as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRectangle_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Top(&mut self) -> ::windows::core::Result<i32>;
    fn SetTop(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Left(&mut self) -> ::windows::core::Result<i32>;
    fn SetLeft(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Bottom(&mut self) -> ::windows::core::Result<i32>;
    fn SetBottom(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Right(&mut self) -> ::windows::core::Result<i32>;
    fn SetRight(&mut self, units: i32) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetData(&mut self, rect: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetRectangle(&mut self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::Result<()>;
    fn SetRectangle(&mut self, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRectangle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>() -> IInkRectangle_Vtbl {
        unsafe extern "system" fn Top<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Top() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTop(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Left<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Left() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLeft(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Bottom<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Bottom() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBottom(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Right<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Right() {
                ::core::result::Result::Ok(ok__) => {
                    *units = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRight(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *rect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn GetRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRectangle(::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRectangle(::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&right)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Top: Top::<Identity, Impl, OFFSET>,
            SetTop: SetTop::<Identity, Impl, OFFSET>,
            Left: Left::<Identity, Impl, OFFSET>,
            SetLeft: SetLeft::<Identity, Impl, OFFSET>,
            Bottom: Bottom::<Identity, Impl, OFFSET>,
            SetBottom: SetBottom::<Identity, Impl, OFFSET>,
            Right: Right::<Identity, Impl, OFFSET>,
            SetRight: SetRight::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetRectangle: GetRectangle::<Identity, Impl, OFFSET>,
            SetRectangle: SetRectangle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRectangle as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkRenderer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetViewTransform(&mut self, viewtransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn SetViewTransform(&mut self, viewtransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn GetObjectTransform(&mut self, objecttransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn SetObjectTransform(&mut self, objecttransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn Draw(&mut self, hdc: isize, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn DrawStroke(&mut self, hdc: isize, stroke: &::core::option::Option<IInkStrokeDisp>, drawingattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn PixelToInkSpace(&mut self, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
    fn InkSpaceToPixel(&mut self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
    fn PixelToInkSpaceFromPoints(&mut self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InkSpaceToPixelFromPoints(&mut self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Measure(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<IInkRectangle>;
    fn MeasureStroke(&mut self, stroke: &::core::option::Option<IInkStrokeDisp>, drawingattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<IInkRectangle>;
    fn Move(&mut self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>() -> IInkRenderer_Vtbl {
        unsafe extern "system" fn GetViewTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetViewTransform(::core::mem::transmute(&viewtransform)).into()
        }
        unsafe extern "system" fn SetViewTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewTransform(::core::mem::transmute(&viewtransform)).into()
        }
        unsafe extern "system" fn GetObjectTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectTransform(::core::mem::transmute(&objecttransform)).into()
        }
        unsafe extern "system" fn SetObjectTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectTransform(::core::mem::transmute(&objecttransform)).into()
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&hdc), ::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn DrawStroke<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawStroke(::core::mem::transmute_copy(&hdc), ::core::mem::transmute(&stroke), ::core::mem::transmute(&drawingattributes)).into()
        }
        unsafe extern "system" fn PixelToInkSpace<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PixelToInkSpace(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InkSpaceToPixel<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InkSpaceToPixel(::core::mem::transmute_copy(&hdcdisplay), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn PixelToInkSpaceFromPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PixelToInkSpaceFromPoints(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn InkSpaceToPixelFromPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InkSpaceToPixelFromPoints(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn Measure<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Measure(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureStroke<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MeasureStroke(::core::mem::transmute(&stroke), ::core::mem::transmute(&drawingattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetViewTransform: GetViewTransform::<Identity, Impl, OFFSET>,
            SetViewTransform: SetViewTransform::<Identity, Impl, OFFSET>,
            GetObjectTransform: GetObjectTransform::<Identity, Impl, OFFSET>,
            SetObjectTransform: SetObjectTransform::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            DrawStroke: DrawStroke::<Identity, Impl, OFFSET>,
            PixelToInkSpace: PixelToInkSpace::<Identity, Impl, OFFSET>,
            InkSpaceToPixel: InkSpaceToPixel::<Identity, Impl, OFFSET>,
            PixelToInkSpaceFromPoints: PixelToInkSpaceFromPoints::<Identity, Impl, OFFSET>,
            InkSpaceToPixelFromPoints: InkSpaceToPixelFromPoints::<Identity, Impl, OFFSET>,
            Measure: Measure::<Identity, Impl, OFFSET>,
            MeasureStroke: MeasureStroke::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkRenderer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkStrokeDisp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ID(&mut self) -> ::windows::core::Result<i32>;
    fn BezierPoints(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DrawingAttributes(&mut self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&mut self, drawattrs: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn PolylineCusps(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn BezierCusps(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SelfIntersections(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PacketCount(&mut self) -> ::windows::core::Result<i32>;
    fn PacketSize(&mut self) -> ::windows::core::Result<i32>;
    fn PacketDescription(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Deleted(&mut self) -> ::windows::core::Result<i16>;
    fn GetBoundingBox(&mut self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn FindIntersections(&mut self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetRectangleIntersections(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Clip(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn HitTestCircle(&mut self, x: i32, y: i32, radius: f32) -> ::windows::core::Result<i16>;
    fn NearestPoint(&mut self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::Result<()>;
    fn Split(&mut self, splitat: f32) -> ::windows::core::Result<IInkStrokeDisp>;
    fn GetPacketDescriptionPropertyMetrics(&mut self, propertyname: &super::super::Foundation::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::Result<()>;
    fn GetPoints(&mut self, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPoints(&mut self, points: &super::super::System::Com::VARIANT, index: i32, count: i32) -> ::windows::core::Result<i32>;
    fn GetPacketData(&mut self, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPacketValuesByProperty(&mut self, propertyname: &super::super::Foundation::BSTR, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPacketValuesByProperty(&mut self, bstrpropertyname: &super::super::Foundation::BSTR, packetvalues: &super::super::System::Com::VARIANT, index: i32, count: i32) -> ::windows::core::Result<i32>;
    fn GetFlattenedBezierPoints(&mut self, fittingerror: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Transform(&mut self, transform: &::core::option::Option<IInkTransform>, applyonpenwidth: i16) -> ::windows::core::Result<()>;
    fn ScaleToRectangle(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Move(&mut self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Shear(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokeDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>() -> IInkStrokeDisp_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BezierPoints() {
                ::core::result::Result::Ok(ok__) => {
                    *points = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *drawattrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_DrawingAttributes(::core::mem::transmute(&drawattrs)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolylineCusps<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolylineCusps() {
                ::core::result::Result::Ok(ok__) => {
                    *cusps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierCusps<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BezierCusps() {
                ::core::result::Result::Ok(ok__) => {
                    *cusps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelfIntersections<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelfIntersections() {
                ::core::result::Result::Ok(ok__) => {
                    *intersections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketCount<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PacketCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *packetdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deleted<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Deleted() {
                ::core::result::Result::Ok(ok__) => {
                    *deleted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindIntersections<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindIntersections(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    *intersections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectangleIntersections<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRectangleIntersections(::core::mem::transmute(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *intersections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn HitTestCircle<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HitTestCircle(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&radius)) {
                ::core::result::Result::Ok(ok__) => {
                    *intersects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearestPoint<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NearestPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&distance), ::core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn Split<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, splitat: f32, newstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Split(::core::mem::transmute_copy(&splitat)) {
                ::core::result::Result::Ok(ok__) => {
                    *newstroke = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketDescriptionPropertyMetrics<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPacketDescriptionPropertyMetrics(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)).into()
        }
        unsafe extern "system" fn GetPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPoints(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *points = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetPoints(::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *numberofpointsset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketData<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, packetdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPacketData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *packetdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketValuesByProperty<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPacketValuesByProperty(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *packetvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPacketValuesByProperty<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetPacketValuesByProperty(::core::mem::transmute_copy(&bstrpropertyname), ::core::mem::transmute_copy(&packetvalues), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *numberofpacketsset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlattenedBezierPoints<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlattenedBezierPoints(::core::mem::transmute_copy(&fittingerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *flattenedbezierpoints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Transform(::core::mem::transmute(&transform), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleToRectangle(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shear(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ID: ID::<Identity, Impl, OFFSET>,
            BezierPoints: BezierPoints::<Identity, Impl, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, Impl, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
            PolylineCusps: PolylineCusps::<Identity, Impl, OFFSET>,
            BezierCusps: BezierCusps::<Identity, Impl, OFFSET>,
            SelfIntersections: SelfIntersections::<Identity, Impl, OFFSET>,
            PacketCount: PacketCount::<Identity, Impl, OFFSET>,
            PacketSize: PacketSize::<Identity, Impl, OFFSET>,
            PacketDescription: PacketDescription::<Identity, Impl, OFFSET>,
            Deleted: Deleted::<Identity, Impl, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, Impl, OFFSET>,
            FindIntersections: FindIntersections::<Identity, Impl, OFFSET>,
            GetRectangleIntersections: GetRectangleIntersections::<Identity, Impl, OFFSET>,
            Clip: Clip::<Identity, Impl, OFFSET>,
            HitTestCircle: HitTestCircle::<Identity, Impl, OFFSET>,
            NearestPoint: NearestPoint::<Identity, Impl, OFFSET>,
            Split: Split::<Identity, Impl, OFFSET>,
            GetPacketDescriptionPropertyMetrics: GetPacketDescriptionPropertyMetrics::<Identity, Impl, OFFSET>,
            GetPoints: GetPoints::<Identity, Impl, OFFSET>,
            SetPoints: SetPoints::<Identity, Impl, OFFSET>,
            GetPacketData: GetPacketData::<Identity, Impl, OFFSET>,
            GetPacketValuesByProperty: GetPacketValuesByProperty::<Identity, Impl, OFFSET>,
            SetPacketValuesByProperty: SetPacketValuesByProperty::<Identity, Impl, OFFSET>,
            GetFlattenedBezierPoints: GetFlattenedBezierPoints::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            ScaleToRectangle: ScaleToRectangle::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            Shear: Shear::<Identity, Impl, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokeDisp as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkStrokes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn RecognitionResult(&mut self) -> ::windows::core::Result<IInkRecognitionResult>;
    fn ToString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkStrokeDisp>;
    fn Add(&mut self, inkstroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn AddStrokes(&mut self, inkstrokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, inkstroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn RemoveStrokes(&mut self, inkstrokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn ModifyDrawingAttributes(&mut self, drawattrs: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn GetBoundingBox(&mut self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn Transform(&mut self, transform: &::core::option::Option<IInkTransform>, applyonpenwidth: i16) -> ::windows::core::Result<()>;
    fn ScaleToRectangle(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Move(&mut self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Shear(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn Clip(&mut self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn RemoveRecognitionResult(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>() -> IInkStrokes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionResult<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecognitionResult() {
                ::core::result::Result::Ok(ok__) => {
                    *recognitionresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToString<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToString() {
                ::core::result::Result::Ok(ok__) => {
                    *tostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, stroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *stroke = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&inkstroke)).into()
        }
        unsafe extern "system" fn AddStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStrokes(::core::mem::transmute(&inkstrokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&inkstroke)).into()
        }
        unsafe extern "system" fn RemoveStrokes<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStrokes(::core::mem::transmute(&inkstrokes)).into()
        }
        unsafe extern "system" fn ModifyDrawingAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyDrawingAttributes(::core::mem::transmute(&drawattrs)).into()
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *boundingbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Transform(::core::mem::transmute(&transform), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleToRectangle(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shear(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn RemoveRecognitionResult<Identity: ::windows::core::IUnknownImpl, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveRecognitionResult().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            RecognitionResult: RecognitionResult::<Identity, Impl, OFFSET>,
            ToString: ToString::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddStrokes: AddStrokes::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveStrokes: RemoveStrokes::<Identity, Impl, OFFSET>,
            ModifyDrawingAttributes: ModifyDrawingAttributes::<Identity, Impl, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            ScaleToRectangle: ScaleToRectangle::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            Shear: Shear::<Identity, Impl, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, Impl, OFFSET>,
            Clip: Clip::<Identity, Impl, OFFSET>,
            RemoveRecognitionResult: RemoveRecognitionResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkStrokes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PlugAndPlayId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MaximumInputRectangle(&mut self) -> ::windows::core::Result<IInkRectangle>;
    fn HardwareCapabilities(&mut self) -> ::windows::core::Result<TabletHardwareCapabilities>;
    fn IsPacketPropertySupported(&mut self, packetpropertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetPropertyMetrics(&mut self, propertyname: &super::super::Foundation::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>() -> IInkTablet_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugAndPlayId<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlugAndPlayId() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaximumInputRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *rectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *capabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPacketPropertySupported(::core::mem::transmute_copy(&packetpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyMetrics<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyMetrics(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            PlugAndPlayId: PlugAndPlayId::<Identity, Impl, OFFSET>,
            MaximumInputRectangle: MaximumInputRectangle::<Identity, Impl, OFFSET>,
            HardwareCapabilities: HardwareCapabilities::<Identity, Impl, OFFSET>,
            IsPacketPropertySupported: IsPacketPropertySupported::<Identity, Impl, OFFSET>,
            GetPropertyMetrics: GetPropertyMetrics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeviceKind(&mut self) -> ::windows::core::Result<TabletDeviceKind>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet2_Impl, const OFFSET: isize>() -> IInkTablet2_Vtbl {
        unsafe extern "system" fn DeviceKind<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: *mut TabletDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *kind = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DeviceKind: DeviceKind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet3_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsMultiTouch(&mut self) -> ::windows::core::Result<i16>;
    fn MaximumCursors(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet3_Impl, const OFFSET: isize>() -> IInkTablet3_Vtbl {
        unsafe extern "system" fn IsMultiTouch<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismultitouch: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMultiTouch() {
                ::core::result::Result::Ok(ok__) => {
                    *pismultitouch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumCursors<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaximumcursors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaximumCursors() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaximumcursors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsMultiTouch: IsMultiTouch::<Identity, Impl, OFFSET>,
            MaximumCursors: MaximumCursors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablets_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn DefaultTablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IInkTablet>;
    fn IsPacketPropertySupported(&mut self, packetpropertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablets_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>() -> IInkTablets_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *_newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTablet<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultTablet() {
                ::core::result::Result::Ok(ok__) => {
                    *defaulttablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, tablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *tablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: ::windows::core::IUnknownImpl, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPacketPropertySupported(::core::mem::transmute_copy(&packetpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            DefaultTablet: DefaultTablet::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            IsPacketPropertySupported: IsPacketPropertySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablets as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTransform_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Translate(&mut self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&mut self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Reflect(&mut self, horizontally: i16, vertically: i16) -> ::windows::core::Result<()>;
    fn Shear(&mut self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&mut self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn GetTransform(&mut self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::Result<()>;
    fn SetTransform(&mut self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::Result<()>;
    fn eM11(&mut self) -> ::windows::core::Result<f32>;
    fn SeteM11(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn eM12(&mut self) -> ::windows::core::Result<f32>;
    fn SeteM12(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn eM21(&mut self) -> ::windows::core::Result<f32>;
    fn SeteM21(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn eM22(&mut self) -> ::windows::core::Result<f32>;
    fn SeteM22(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn eDx(&mut self) -> ::windows::core::Result<f32>;
    fn SeteDx(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn eDy(&mut self) -> ::windows::core::Result<f32>;
    fn SeteDy(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::XFORM>;
    fn SetData(&mut self, xform: &super::super::Graphics::Gdi::XFORM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>() -> IInkTransform_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Translate<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Translate(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Reflect<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontally: i16, vertically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reflect(::core::mem::transmute_copy(&horizontally), ::core::mem::transmute_copy(&vertically)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shear(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&em11), ::core::mem::transmute_copy(&em12), ::core::mem::transmute_copy(&em21), ::core::mem::transmute_copy(&em22), ::core::mem::transmute_copy(&edx), ::core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransform(::core::mem::transmute_copy(&em11), ::core::mem::transmute_copy(&em12), ::core::mem::transmute_copy(&em21), ::core::mem::transmute_copy(&em22), ::core::mem::transmute_copy(&edx), ::core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn eM11<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eM11() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM11<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteM11(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM12<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eM12() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM12<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteM12(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM21<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eM21() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM21<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteM21(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM22<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eM22() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM22<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteM22(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDx<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eDx() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDx<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteDx(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDy<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).eDy() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDy<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeteDy(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *xform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&xform)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Translate: Translate::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            Reflect: Reflect::<Identity, Impl, OFFSET>,
            Shear: Shear::<Identity, Impl, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            eM11: eM11::<Identity, Impl, OFFSET>,
            SeteM11: SeteM11::<Identity, Impl, OFFSET>,
            eM12: eM12::<Identity, Impl, OFFSET>,
            SeteM12: SeteM12::<Identity, Impl, OFFSET>,
            eM21: eM21::<Identity, Impl, OFFSET>,
            SeteM21: SeteM21::<Identity, Impl, OFFSET>,
            eM22: eM22::<Identity, Impl, OFFSET>,
            SeteM22: SeteM22::<Identity, Impl, OFFSET>,
            eDx: eDx::<Identity, Impl, OFFSET>,
            SeteDx: SeteDx::<Identity, Impl, OFFSET>,
            eDy: eDy::<Identity, Impl, OFFSET>,
            SeteDy: SeteDy::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTransform as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkWordList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddWord(&mut self, newword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveWord(&mut self, removeword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Merge(&mut self, mergewordlist: &::core::option::Option<IInkWordList>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList_Impl, const OFFSET: isize>() -> IInkWordList_Vtbl {
        unsafe extern "system" fn AddWord<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWord(::core::mem::transmute_copy(&newword)).into()
        }
        unsafe extern "system" fn RemoveWord<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, removeword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveWord(::core::mem::transmute_copy(&removeword)).into()
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergewordlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Merge(::core::mem::transmute(&mergewordlist)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddWord: AddWord::<Identity, Impl, OFFSET>,
            RemoveWord: RemoveWord::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWordList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkWordList2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddWords(&mut self, newwords: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList2_Impl, const OFFSET: isize>() -> IInkWordList2_Vtbl {
        unsafe extern "system" fn AddWords<Identity: ::windows::core::IUnknownImpl, Impl: IInkWordList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwords: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWords(::core::mem::transmute_copy(&newwords)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), AddWords: AddWords::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWordList2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IInputPanelWindowHandle_Impl: Sized {
    fn AttachedEditWindow32(&mut self) -> ::windows::core::Result<i32>;
    fn SetAttachedEditWindow32(&mut self, attachededitwindow: i32) -> ::windows::core::Result<()>;
    fn AttachedEditWindow64(&mut self) -> ::windows::core::Result<i64>;
    fn SetAttachedEditWindow64(&mut self, attachededitwindow: i64) -> ::windows::core::Result<()>;
}
impl IInputPanelWindowHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>() -> IInputPanelWindowHandle_Vtbl {
        unsafe extern "system" fn AttachedEditWindow32<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttachedEditWindow32() {
                ::core::result::Result::Ok(ok__) => {
                    *attachededitwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachedEditWindow32(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn AttachedEditWindow64<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttachedEditWindow64() {
                ::core::result::Result::Ok(ok__) => {
                    *attachededitwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Identity: ::windows::core::IUnknownImpl, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachedEditWindow64(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow32: AttachedEditWindow32::<Identity, Impl, OFFSET>,
            SetAttachedEditWindow32: SetAttachedEditWindow32::<Identity, Impl, OFFSET>,
            AttachedEditWindow64: AttachedEditWindow64::<Identity, Impl, OFFSET>,
            SetAttachedEditWindow64: SetAttachedEditWindow64::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPanelWindowHandle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMathInputControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Show(&mut self) -> ::windows::core::Result<()>;
    fn Hide(&mut self) -> ::windows::core::Result<()>;
    fn IsVisible(&mut self) -> ::windows::core::Result<i16>;
    fn GetPosition(&mut self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::Result<()>;
    fn SetPosition(&mut self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn SetCustomPaint(&mut self, element: i32, paint: i16) -> ::windows::core::Result<()>;
    fn SetCaptionText(&mut self, captiontext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoadInk(&mut self, ink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn SetOwnerWindow(&mut self, ownerwindow: isize) -> ::windows::core::Result<()>;
    fn EnableExtendedButtons(&mut self, extended: i16) -> ::windows::core::Result<()>;
    fn GetPreviewHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetPreviewHeight(&mut self, height: i32) -> ::windows::core::Result<()>;
    fn EnableAutoGrow(&mut self, autogrow: i16) -> ::windows::core::Result<()>;
    fn AddFunctionName(&mut self, functionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveFunctionName(&mut self, functionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetHoverIcon(&mut self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMathInputControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>() -> IMathInputControl_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Hide<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Hide().into()
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshown: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbshown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPosition(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetCustomPaint<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: i32, paint: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustomPaint(::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&paint)).into()
        }
        unsafe extern "system" fn SetCaptionText<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captiontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaptionText(::core::mem::transmute_copy(&captiontext)).into()
        }
        unsafe extern "system" fn LoadInk<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadInk(::core::mem::transmute(&ink)).into()
        }
        unsafe extern "system" fn SetOwnerWindow<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownerwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerWindow(::core::mem::transmute_copy(&ownerwindow)).into()
        }
        unsafe extern "system" fn EnableExtendedButtons<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableExtendedButtons(::core::mem::transmute_copy(&extended)).into()
        }
        unsafe extern "system" fn GetPreviewHeight<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviewHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *height = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewHeight<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreviewHeight(::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn EnableAutoGrow<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autogrow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableAutoGrow(::core::mem::transmute_copy(&autogrow)).into()
        }
        unsafe extern "system" fn AddFunctionName<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFunctionName(::core::mem::transmute_copy(&functionname)).into()
        }
        unsafe extern "system" fn RemoveFunctionName<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFunctionName(::core::mem::transmute_copy(&functionname)).into()
        }
        unsafe extern "system" fn GetHoverIcon<Identity: ::windows::core::IUnknownImpl, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hoverimage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHoverIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *hoverimage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Show: Show::<Identity, Impl, OFFSET>,
            Hide: Hide::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetCustomPaint: SetCustomPaint::<Identity, Impl, OFFSET>,
            SetCaptionText: SetCaptionText::<Identity, Impl, OFFSET>,
            LoadInk: LoadInk::<Identity, Impl, OFFSET>,
            SetOwnerWindow: SetOwnerWindow::<Identity, Impl, OFFSET>,
            EnableExtendedButtons: EnableExtendedButtons::<Identity, Impl, OFFSET>,
            GetPreviewHeight: GetPreviewHeight::<Identity, Impl, OFFSET>,
            SetPreviewHeight: SetPreviewHeight::<Identity, Impl, OFFSET>,
            EnableAutoGrow: EnableAutoGrow::<Identity, Impl, OFFSET>,
            AddFunctionName: AddFunctionName::<Identity, Impl, OFFSET>,
            RemoveFunctionName: RemoveFunctionName::<Identity, Impl, OFFSET>,
            GetHoverIcon: GetHoverIcon::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMathInputControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPenInputPanel_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Busy(&mut self) -> ::windows::core::Result<i16>;
    fn Factoid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&mut self, factoid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AttachedEditWindow(&mut self) -> ::windows::core::Result<i32>;
    fn SetAttachedEditWindow(&mut self, attachededitwindow: i32) -> ::windows::core::Result<()>;
    fn CurrentPanel(&mut self) -> ::windows::core::Result<PanelType>;
    fn SetCurrentPanel(&mut self, currentpanel: PanelType) -> ::windows::core::Result<()>;
    fn DefaultPanel(&mut self) -> ::windows::core::Result<PanelType>;
    fn SetDefaultPanel(&mut self, defaultpanel: PanelType) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<i16>;
    fn SetVisible(&mut self, visible: i16) -> ::windows::core::Result<()>;
    fn Top(&mut self) -> ::windows::core::Result<i32>;
    fn Left(&mut self) -> ::windows::core::Result<i32>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<i32>;
    fn SetVerticalOffset(&mut self, verticaloffset: i32) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<i32>;
    fn SetHorizontalOffset(&mut self, horizontaloffset: i32) -> ::windows::core::Result<()>;
    fn AutoShow(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoShow(&mut self, autoshow: i16) -> ::windows::core::Result<()>;
    fn MoveTo(&mut self, left: i32, top: i32) -> ::windows::core::Result<()>;
    fn CommitPendingInput(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn EnableTsf(&mut self, enable: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPenInputPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>() -> IPenInputPanel_Vtbl {
        unsafe extern "system" fn Busy<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busy: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Busy() {
                ::core::result::Result::Ok(ok__) => {
                    *busy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    *factoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFactoid(::core::mem::transmute_copy(&factoid)).into()
        }
        unsafe extern "system" fn AttachedEditWindow<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttachedEditWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *attachededitwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentPanel<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPanel() {
                ::core::result::Result::Ok(ok__) => {
                    *currentpanel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPanel<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentPanel(::core::mem::transmute_copy(&currentpanel)).into()
        }
        unsafe extern "system" fn DefaultPanel<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultPanel() {
                ::core::result::Result::Ok(ok__) => {
                    *pdefaultpanel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPanel<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultPanel(::core::mem::transmute_copy(&defaultpanel)).into()
        }
        unsafe extern "system" fn Visible<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *visible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn Top<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Top() {
                ::core::result::Result::Ok(ok__) => {
                    *top = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Left() {
                ::core::result::Result::Ok(ok__) => {
                    *left = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *width = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *height = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *verticaloffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVerticalOffset(::core::mem::transmute_copy(&verticaloffset)).into()
        }
        unsafe extern "system" fn HorizontalOffset<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *horizontaloffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHorizontalOffset(::core::mem::transmute_copy(&horizontaloffset)).into()
        }
        unsafe extern "system" fn AutoShow<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pautoshow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoShow() {
                ::core::result::Result::Ok(ok__) => {
                    *pautoshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoShow<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoshow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoShow(::core::mem::transmute_copy(&autoshow)).into()
        }
        unsafe extern "system" fn MoveTo<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveTo(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn CommitPendingInput<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitPendingInput().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn EnableTsf<Identity: ::windows::core::IUnknownImpl, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableTsf(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Busy: Busy::<Identity, Impl, OFFSET>,
            Factoid: Factoid::<Identity, Impl, OFFSET>,
            SetFactoid: SetFactoid::<Identity, Impl, OFFSET>,
            AttachedEditWindow: AttachedEditWindow::<Identity, Impl, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, Impl, OFFSET>,
            CurrentPanel: CurrentPanel::<Identity, Impl, OFFSET>,
            SetCurrentPanel: SetCurrentPanel::<Identity, Impl, OFFSET>,
            DefaultPanel: DefaultPanel::<Identity, Impl, OFFSET>,
            SetDefaultPanel: SetDefaultPanel::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            SetVisible: SetVisible::<Identity, Impl, OFFSET>,
            Top: Top::<Identity, Impl, OFFSET>,
            Left: Left::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            VerticalOffset: VerticalOffset::<Identity, Impl, OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Identity, Impl, OFFSET>,
            HorizontalOffset: HorizontalOffset::<Identity, Impl, OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Identity, Impl, OFFSET>,
            AutoShow: AutoShow::<Identity, Impl, OFFSET>,
            SetAutoShow: SetAutoShow::<Identity, Impl, OFFSET>,
            MoveTo: MoveTo::<Identity, Impl, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            EnableTsf: EnableTsf::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPenInputPanel as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRealTimeStylus_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HWND(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&mut self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn WindowInputRectangle(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetWindowInputRectangle(&mut self, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn AddStylusSyncPlugin(&mut self, iindex: u32, piplugin: &::core::option::Option<IStylusSyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveStylusSyncPlugin(&mut self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusSyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveAllStylusSyncPlugins(&mut self) -> ::windows::core::Result<()>;
    fn GetStylusSyncPlugin(&mut self, iindex: u32) -> ::windows::core::Result<IStylusSyncPlugin>;
    fn GetStylusSyncPluginCount(&mut self) -> ::windows::core::Result<u32>;
    fn AddStylusAsyncPlugin(&mut self, iindex: u32, piplugin: &::core::option::Option<IStylusAsyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveStylusAsyncPlugin(&mut self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusAsyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveAllStylusAsyncPlugins(&mut self) -> ::windows::core::Result<()>;
    fn GetStylusAsyncPlugin(&mut self, iindex: u32) -> ::windows::core::Result<IStylusAsyncPlugin>;
    fn GetStylusAsyncPluginCount(&mut self) -> ::windows::core::Result<u32>;
    fn ChildRealTimeStylusPlugin(&mut self) -> ::windows::core::Result<IRealTimeStylus>;
    fn putref_ChildRealTimeStylusPlugin(&mut self, pirts: &::core::option::Option<IRealTimeStylus>) -> ::windows::core::Result<()>;
    fn AddCustomStylusDataToQueue(&mut self, sq: StylusQueue, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::Result<()>;
    fn ClearStylusQueues(&mut self) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&mut self, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSingleTabletMode(&mut self, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetTablet(&mut self) -> ::windows::core::Result<IInkTablet>;
    fn GetTabletContextIdFromTablet(&mut self, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<u32>;
    fn GetTabletFromTabletContextId(&mut self, tcid: u32) -> ::windows::core::Result<IInkTablet>;
    fn GetAllTabletContextIds(&mut self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetStyluses(&mut self) -> ::windows::core::Result<IInkCursors>;
    fn GetStylusForId(&mut self, sid: u32) -> ::windows::core::Result<IInkCursor>;
    fn SetDesiredPacketDescription(&mut self, cproperties: u32, ppropertyguids: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetDesiredPacketDescription(&mut self, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPacketDescriptionData(&mut self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRealTimeStylus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>() -> IRealTimeStylus_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HWND() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn WindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowInputRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *prcwndinputrect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowInputRectangle(::core::mem::transmute_copy(&prcwndinputrect)).into()
        }
        unsafe extern "system" fn AddStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStylusSyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStylusSyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusSyncPlugins<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllStylusSyncPlugins().into()
        }
        unsafe extern "system" fn GetStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStylusSyncPlugin(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiplugin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusSyncPluginCount<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStylusSyncPluginCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcplugins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStylusAsyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStylusAsyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusAsyncPlugins<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllStylusAsyncPlugins().into()
        }
        unsafe extern "system" fn GetStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStylusAsyncPlugin(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiplugin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusAsyncPluginCount<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStylusAsyncPluginCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcplugins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildRealTimeStylusPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppirts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChildRealTimeStylusPlugin() {
                ::core::result::Result::Ok(ok__) => {
                    *ppirts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ChildRealTimeStylusPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ChildRealTimeStylusPlugin(::core::mem::transmute(&pirts)).into()
        }
        unsafe extern "system" fn AddCustomStylusDataToQueue<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sq: StylusQueue, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddCustomStylusDataToQueue(::core::mem::transmute_copy(&sq), ::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn ClearStylusQueues<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStylusQueues().into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllTabletsMode(::core::mem::transmute_copy(&fusemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletMode<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSingleTabletMode(::core::mem::transmute(&pitablet)).into()
        }
        unsafe extern "system" fn GetTablet<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisingletablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTablet() {
                ::core::result::Result::Ok(ok__) => {
                    *ppisingletablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletContextIdFromTablet<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: ::windows::core::RawPtr, ptcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTabletContextIdFromTablet(::core::mem::transmute(&pitablet)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptcid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletFromTabletContextId<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, ppitablet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTabletFromTabletContextId(::core::mem::transmute_copy(&tcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitablet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllTabletContextIds<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAllTabletContextIds(::core::mem::transmute_copy(&pctcidcount), ::core::mem::transmute_copy(&pptcids)).into()
        }
        unsafe extern "system" fn GetStyluses<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkcursors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStyluses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiinkcursors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusForId<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u32, ppiinkcursor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStylusForId(::core::mem::transmute_copy(&sid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiinkcursor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropertyguids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesiredPacketDescription(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropertyguids)).into()
        }
        unsafe extern "system" fn GetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesiredPacketDescription(::core::mem::transmute_copy(&pcproperties), ::core::mem::transmute_copy(&pppropertyguids)).into()
        }
        unsafe extern "system" fn GetPacketDescriptionData<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPacketDescriptionData(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&pfinktodevicescalex), ::core::mem::transmute_copy(&pfinktodevicescaley), ::core::mem::transmute_copy(&pcpacketproperties), ::core::mem::transmute_copy(&pppacketproperties)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            HWND: HWND::<Identity, Impl, OFFSET>,
            SetHWND: SetHWND::<Identity, Impl, OFFSET>,
            WindowInputRectangle: WindowInputRectangle::<Identity, Impl, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, Impl, OFFSET>,
            AddStylusSyncPlugin: AddStylusSyncPlugin::<Identity, Impl, OFFSET>,
            RemoveStylusSyncPlugin: RemoveStylusSyncPlugin::<Identity, Impl, OFFSET>,
            RemoveAllStylusSyncPlugins: RemoveAllStylusSyncPlugins::<Identity, Impl, OFFSET>,
            GetStylusSyncPlugin: GetStylusSyncPlugin::<Identity, Impl, OFFSET>,
            GetStylusSyncPluginCount: GetStylusSyncPluginCount::<Identity, Impl, OFFSET>,
            AddStylusAsyncPlugin: AddStylusAsyncPlugin::<Identity, Impl, OFFSET>,
            RemoveStylusAsyncPlugin: RemoveStylusAsyncPlugin::<Identity, Impl, OFFSET>,
            RemoveAllStylusAsyncPlugins: RemoveAllStylusAsyncPlugins::<Identity, Impl, OFFSET>,
            GetStylusAsyncPlugin: GetStylusAsyncPlugin::<Identity, Impl, OFFSET>,
            GetStylusAsyncPluginCount: GetStylusAsyncPluginCount::<Identity, Impl, OFFSET>,
            ChildRealTimeStylusPlugin: ChildRealTimeStylusPlugin::<Identity, Impl, OFFSET>,
            putref_ChildRealTimeStylusPlugin: putref_ChildRealTimeStylusPlugin::<Identity, Impl, OFFSET>,
            AddCustomStylusDataToQueue: AddCustomStylusDataToQueue::<Identity, Impl, OFFSET>,
            ClearStylusQueues: ClearStylusQueues::<Identity, Impl, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, Impl, OFFSET>,
            SetSingleTabletMode: SetSingleTabletMode::<Identity, Impl, OFFSET>,
            GetTablet: GetTablet::<Identity, Impl, OFFSET>,
            GetTabletContextIdFromTablet: GetTabletContextIdFromTablet::<Identity, Impl, OFFSET>,
            GetTabletFromTabletContextId: GetTabletFromTabletContextId::<Identity, Impl, OFFSET>,
            GetAllTabletContextIds: GetAllTabletContextIds::<Identity, Impl, OFFSET>,
            GetStyluses: GetStyluses::<Identity, Impl, OFFSET>,
            GetStylusForId: GetStylusForId::<Identity, Impl, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, Impl, OFFSET>,
            GetDesiredPacketDescription: GetDesiredPacketDescription::<Identity, Impl, OFFSET>,
            GetPacketDescriptionData: GetPacketDescriptionData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRealTimeStylus2_Impl: Sized {
    fn FlicksEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetFlicksEnabled(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>() -> IRealTimeStylus2_Vtbl {
        unsafe extern "system" fn FlicksEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FlicksEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlicksEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlicksEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FlicksEnabled: FlicksEnabled::<Identity, Impl, OFFSET>,
            SetFlicksEnabled: SetFlicksEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRealTimeStylus3_Impl: Sized {
    fn MultiTouchEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMultiTouchEnabled(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>() -> IRealTimeStylus3_Vtbl {
        unsafe extern "system" fn MultiTouchEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultiTouchEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiTouchEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiTouchEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MultiTouchEnabled: MultiTouchEnabled::<Identity, Impl, OFFSET>,
            SetMultiTouchEnabled: SetMultiTouchEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus3 as ::windows::core::Interface>::IID
    }
}
pub trait IRealTimeStylusSynchronization_Impl: Sized {
    fn AcquireLock(&mut self, lock: RealTimeStylusLockType) -> ::windows::core::Result<()>;
    fn ReleaseLock(&mut self, lock: RealTimeStylusLockType) -> ::windows::core::Result<()>;
}
impl IRealTimeStylusSynchronization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>() -> IRealTimeStylusSynchronization_Vtbl {
        unsafe extern "system" fn AcquireLock<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireLock(::core::mem::transmute_copy(&lock)).into()
        }
        unsafe extern "system" fn ReleaseLock<Identity: ::windows::core::IUnknownImpl, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseLock(::core::mem::transmute_copy(&lock)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireLock: AcquireLock::<Identity, Impl, OFFSET>,
            ReleaseLock: ReleaseLock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylusSynchronization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISketchInk_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISketchInk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISketchInk_Impl, const OFFSET: isize>() -> ISketchInk_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISketchInk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStrokeBuilder_Impl: Sized {
    fn CreateStroke(&mut self, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn BeginStroke(&mut self, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn AppendPackets(&mut self, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::Result<()>;
    fn EndStroke(&mut self, tcid: u32, sid: u32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Ink(&mut self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&mut self, piinkobj: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStrokeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>() -> IStrokeBuilder_Vtbl {
        unsafe extern "system" fn CreateStroke<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateStroke(::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&cpacketproperties), ::core::mem::transmute_copy(&ppacketproperties), ::core::mem::transmute_copy(&finktodevicescalex), ::core::mem::transmute_copy(&finktodevicescaley), ::core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn BeginStroke<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginStroke(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&cpacketproperties), ::core::mem::transmute_copy(&ppacketproperties), ::core::mem::transmute_copy(&finktodevicescalex), ::core::mem::transmute_copy(&finktodevicescaley), ::core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn AppendPackets<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AppendPackets(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets)).into()
        }
        unsafe extern "system" fn EndStroke<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut ::windows::core::RawPtr, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndStroke(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&ppiinkstroke), ::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiinkobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Ink(::core::mem::transmute(&piinkobj)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateStroke: CreateStroke::<Identity, Impl, OFFSET>,
            BeginStroke: BeginStroke::<Identity, Impl, OFFSET>,
            AppendPackets: AppendPackets::<Identity, Impl, OFFSET>,
            EndStroke: EndStroke::<Identity, Impl, OFFSET>,
            Ink: Ink::<Identity, Impl, OFFSET>,
            putref_Ink: putref_Ink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStrokeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStylusAsyncPlugin_Impl: Sized + IStylusPlugin_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusAsyncPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusAsyncPlugin_Impl, const OFFSET: isize>() -> IStylusAsyncPlugin_Vtbl {
        Self { base: IStylusPlugin_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusAsyncPlugin as ::windows::core::Interface>::IID || iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStylusPlugin_Impl: Sized {
    fn RealTimeStylusEnabled(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::Result<()>;
    fn RealTimeStylusDisabled(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::Result<()>;
    fn StylusInRange(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32) -> ::windows::core::Result<()>;
    fn StylusOutOfRange(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32) -> ::windows::core::Result<()>;
    fn StylusDown(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>;
    fn StylusUp(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>;
    fn StylusButtonDown(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn StylusButtonUp(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn InAirPackets(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>;
    fn Packets(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>;
    fn CustomStylusDataAdded(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::Result<()>;
    fn SystemEvent(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32, event: u16, eventdata: &SYSTEM_EVENT_DATA) -> ::windows::core::Result<()>;
    fn TabletAdded(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn TabletRemoved(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, itabletindex: i32) -> ::windows::core::Result<()>;
    fn Error(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>, piplugin: &::core::option::Option<IStylusPlugin>, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::Result<()>;
    fn UpdateMapping(&mut self, pirtssrc: &::core::option::Option<IRealTimeStylus>) -> ::windows::core::Result<()>;
    fn DataInterest(&mut self) -> ::windows::core::Result<RealTimeStylusDataInterest>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>() -> IStylusPlugin_Vtbl {
        unsafe extern "system" fn RealTimeStylusEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RealTimeStylusEnabled(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&ctcidcount), ::core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn RealTimeStylusDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RealTimeStylusDisabled(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&ctcidcount), ::core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn StylusInRange<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusInRange(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusOutOfRange<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusOutOfRange(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusDown<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusDown(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpropcountperpkt), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusUp<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusUp(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpropcountperpkt), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusButtonDown<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusButtonDown(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&pguidstylusbutton), ::core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn StylusButtonUp<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StylusButtonUp(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&pguidstylusbutton), ::core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn InAirPackets<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InAirPackets(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpktcount), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&pcinoutpkts), ::core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn Packets<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Packets(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpktcount), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&pcinoutpkts), ::core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn CustomStylusDataAdded<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CustomStylusDataAdded(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn SystemEvent<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SystemEvent(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventdata)).into()
        }
        unsafe extern "system" fn TabletAdded<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, pitablet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TabletAdded(::core::mem::transmute(&pirtssrc), ::core::mem::transmute(&pitablet)).into()
        }
        unsafe extern "system" fn TabletRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, itabletindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TabletRemoved(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&itabletindex)).into()
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr, piplugin: ::windows::core::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error(::core::mem::transmute(&pirtssrc), ::core::mem::transmute(&piplugin), ::core::mem::transmute_copy(&datainterest), ::core::mem::transmute_copy(&hrerrorcode), ::core::mem::transmute_copy(&lptrkey)).into()
        }
        unsafe extern "system" fn UpdateMapping<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateMapping(::core::mem::transmute(&pirtssrc)).into()
        }
        unsafe extern "system" fn DataInterest<Identity: ::windows::core::IUnknownImpl, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataInterest() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatainterest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RealTimeStylusEnabled: RealTimeStylusEnabled::<Identity, Impl, OFFSET>,
            RealTimeStylusDisabled: RealTimeStylusDisabled::<Identity, Impl, OFFSET>,
            StylusInRange: StylusInRange::<Identity, Impl, OFFSET>,
            StylusOutOfRange: StylusOutOfRange::<Identity, Impl, OFFSET>,
            StylusDown: StylusDown::<Identity, Impl, OFFSET>,
            StylusUp: StylusUp::<Identity, Impl, OFFSET>,
            StylusButtonDown: StylusButtonDown::<Identity, Impl, OFFSET>,
            StylusButtonUp: StylusButtonUp::<Identity, Impl, OFFSET>,
            InAirPackets: InAirPackets::<Identity, Impl, OFFSET>,
            Packets: Packets::<Identity, Impl, OFFSET>,
            CustomStylusDataAdded: CustomStylusDataAdded::<Identity, Impl, OFFSET>,
            SystemEvent: SystemEvent::<Identity, Impl, OFFSET>,
            TabletAdded: TabletAdded::<Identity, Impl, OFFSET>,
            TabletRemoved: TabletRemoved::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            UpdateMapping: UpdateMapping::<Identity, Impl, OFFSET>,
            DataInterest: DataInterest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStylusSyncPlugin_Impl: Sized + IStylusPlugin_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusSyncPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylusSyncPlugin_Impl, const OFFSET: isize>() -> IStylusSyncPlugin_Vtbl {
        Self { base: IStylusPlugin_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusSyncPlugin as ::windows::core::Interface>::IID || iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextInputPanel_Impl: Sized {
    fn AttachedEditWindow(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SetAttachedEditWindow(&mut self, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CurrentInteractionMode(&mut self) -> ::windows::core::Result<InteractionMode>;
    fn DefaultInPlaceState(&mut self) -> ::windows::core::Result<InPlaceState>;
    fn SetDefaultInPlaceState(&mut self, state: InPlaceState) -> ::windows::core::Result<()>;
    fn CurrentInPlaceState(&mut self) -> ::windows::core::Result<InPlaceState>;
    fn DefaultInputArea(&mut self) -> ::windows::core::Result<PanelInputArea>;
    fn SetDefaultInputArea(&mut self, area: PanelInputArea) -> ::windows::core::Result<()>;
    fn CurrentInputArea(&mut self) -> ::windows::core::Result<PanelInputArea>;
    fn CurrentCorrectionMode(&mut self) -> ::windows::core::Result<CorrectionMode>;
    fn PreferredInPlaceDirection(&mut self) -> ::windows::core::Result<InPlaceDirection>;
    fn SetPreferredInPlaceDirection(&mut self, direction: InPlaceDirection) -> ::windows::core::Result<()>;
    fn ExpandPostInsertionCorrection(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetExpandPostInsertionCorrection(&mut self, expand: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceVisibleOnFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetInPlaceVisibleOnFocus(&mut self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceBoundingRectangle(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn PopUpCorrectionHeight(&mut self) -> ::windows::core::Result<i32>;
    fn PopDownCorrectionHeight(&mut self) -> ::windows::core::Result<i32>;
    fn CommitPendingInput(&mut self) -> ::windows::core::Result<()>;
    fn SetInPlaceVisibility(&mut self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetInPlacePosition(&mut self, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::Result<()>;
    fn SetInPlaceHoverTargetPosition(&mut self, xposition: i32, yposition: i32) -> ::windows::core::Result<()>;
    fn Advise(&mut self, eventsink: &::core::option::Option<ITextInputPanelEventSink>, eventmask: u32) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, eventsink: &::core::option::Option<ITextInputPanelEventSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>() -> ITextInputPanel_Vtbl {
        unsafe extern "system" fn AttachedEditWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttachedEditWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *attachededitwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentInteractionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *currentinteractionmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInPlaceState<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultInPlaceState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultInPlaceState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentInPlaceState<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentInPlaceState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInputArea<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultInputArea() {
                ::core::result::Result::Ok(ok__) => {
                    *area = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInputArea<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultInputArea(::core::mem::transmute_copy(&area)).into()
        }
        unsafe extern "system" fn CurrentInputArea<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentInputArea() {
                ::core::result::Result::Ok(ok__) => {
                    *area = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCorrectionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCorrectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: *mut InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreferredInPlaceDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *direction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreferredInPlaceDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpandPostInsertionCorrection() {
                ::core::result::Result::Ok(ok__) => {
                    *expand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExpandPostInsertionCorrection(::core::mem::transmute_copy(&expand)).into()
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InPlaceVisibleOnFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *visible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInPlaceVisibleOnFocus(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InPlaceBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *boundingrectangle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PopUpCorrectionHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *height = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PopDownCorrectionHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *height = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitPendingInput<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitPendingInput().into()
        }
        unsafe extern "system" fn SetInPlaceVisibility<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInPlaceVisibility(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn SetInPlacePosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInPlacePosition(::core::mem::transmute_copy(&xposition), ::core::mem::transmute_copy(&yposition), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInPlaceHoverTargetPosition(::core::mem::transmute_copy(&xposition), ::core::mem::transmute_copy(&yposition)).into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr, eventmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Advise(::core::mem::transmute(&eventsink), ::core::mem::transmute_copy(&eventmask)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&eventsink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow: AttachedEditWindow::<Identity, Impl, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, Impl, OFFSET>,
            CurrentInteractionMode: CurrentInteractionMode::<Identity, Impl, OFFSET>,
            DefaultInPlaceState: DefaultInPlaceState::<Identity, Impl, OFFSET>,
            SetDefaultInPlaceState: SetDefaultInPlaceState::<Identity, Impl, OFFSET>,
            CurrentInPlaceState: CurrentInPlaceState::<Identity, Impl, OFFSET>,
            DefaultInputArea: DefaultInputArea::<Identity, Impl, OFFSET>,
            SetDefaultInputArea: SetDefaultInputArea::<Identity, Impl, OFFSET>,
            CurrentInputArea: CurrentInputArea::<Identity, Impl, OFFSET>,
            CurrentCorrectionMode: CurrentCorrectionMode::<Identity, Impl, OFFSET>,
            PreferredInPlaceDirection: PreferredInPlaceDirection::<Identity, Impl, OFFSET>,
            SetPreferredInPlaceDirection: SetPreferredInPlaceDirection::<Identity, Impl, OFFSET>,
            ExpandPostInsertionCorrection: ExpandPostInsertionCorrection::<Identity, Impl, OFFSET>,
            SetExpandPostInsertionCorrection: SetExpandPostInsertionCorrection::<Identity, Impl, OFFSET>,
            InPlaceVisibleOnFocus: InPlaceVisibleOnFocus::<Identity, Impl, OFFSET>,
            SetInPlaceVisibleOnFocus: SetInPlaceVisibleOnFocus::<Identity, Impl, OFFSET>,
            InPlaceBoundingRectangle: InPlaceBoundingRectangle::<Identity, Impl, OFFSET>,
            PopUpCorrectionHeight: PopUpCorrectionHeight::<Identity, Impl, OFFSET>,
            PopDownCorrectionHeight: PopDownCorrectionHeight::<Identity, Impl, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, Impl, OFFSET>,
            SetInPlaceVisibility: SetInPlaceVisibility::<Identity, Impl, OFFSET>,
            SetInPlacePosition: SetInPlacePosition::<Identity, Impl, OFFSET>,
            SetInPlaceHoverTargetPosition: SetInPlaceHoverTargetPosition::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextInputPanelEventSink_Impl: Sized {
    fn InPlaceStateChanging(&mut self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::Result<()>;
    fn InPlaceStateChanged(&mut self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::Result<()>;
    fn InPlaceSizeChanging(&mut self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InPlaceSizeChanged(&mut self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InputAreaChanging(&mut self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::Result<()>;
    fn InputAreaChanged(&mut self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::Result<()>;
    fn CorrectionModeChanging(&mut self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::Result<()>;
    fn CorrectionModeChanged(&mut self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::Result<()>;
    fn InPlaceVisibilityChanging(&mut self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceVisibilityChanged(&mut self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TextInserting(&mut self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn TextInserted(&mut self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextInputPanelEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>() -> ITextInputPanelEventSink_Vtbl {
        unsafe extern "system" fn InPlaceStateChanging<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceStateChanging(::core::mem::transmute_copy(&oldinplacestate), ::core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceStateChanged<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceStateChanged(::core::mem::transmute_copy(&oldinplacestate), ::core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanging<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceSizeChanging(::core::mem::transmute_copy(&oldboundingrectangle), ::core::mem::transmute_copy(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanged<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceSizeChanged(::core::mem::transmute_copy(&oldboundingrectangle), ::core::mem::transmute_copy(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InputAreaChanging<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InputAreaChanging(::core::mem::transmute_copy(&oldinputarea), ::core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn InputAreaChanged<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InputAreaChanged(::core::mem::transmute_copy(&oldinputarea), ::core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn CorrectionModeChanging<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CorrectionModeChanging(::core::mem::transmute_copy(&oldcorrectionmode), ::core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn CorrectionModeChanged<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CorrectionModeChanged(::core::mem::transmute_copy(&oldcorrectionmode), ::core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceVisibilityChanging(::core::mem::transmute_copy(&oldvisible), ::core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceVisibilityChanged(::core::mem::transmute_copy(&oldvisible), ::core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn TextInserting<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TextInserting(::core::mem::transmute_copy(&ink)).into()
        }
        unsafe extern "system" fn TextInserted<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TextInserted(::core::mem::transmute_copy(&ink)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InPlaceStateChanging: InPlaceStateChanging::<Identity, Impl, OFFSET>,
            InPlaceStateChanged: InPlaceStateChanged::<Identity, Impl, OFFSET>,
            InPlaceSizeChanging: InPlaceSizeChanging::<Identity, Impl, OFFSET>,
            InPlaceSizeChanged: InPlaceSizeChanged::<Identity, Impl, OFFSET>,
            InputAreaChanging: InputAreaChanging::<Identity, Impl, OFFSET>,
            InputAreaChanged: InputAreaChanged::<Identity, Impl, OFFSET>,
            CorrectionModeChanging: CorrectionModeChanging::<Identity, Impl, OFFSET>,
            CorrectionModeChanged: CorrectionModeChanged::<Identity, Impl, OFFSET>,
            InPlaceVisibilityChanging: InPlaceVisibilityChanging::<Identity, Impl, OFFSET>,
            InPlaceVisibilityChanged: InPlaceVisibilityChanged::<Identity, Impl, OFFSET>,
            TextInserting: TextInserting::<Identity, Impl, OFFSET>,
            TextInserted: TextInserted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanelEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextInputPanelRunInfo_Impl: Sized {
    fn IsTipRunning(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanelRunInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelRunInfo_Impl, const OFFSET: isize>() -> ITextInputPanelRunInfo_Vtbl {
        unsafe extern "system" fn IsTipRunning<Identity: ::windows::core::IUnknownImpl, Impl: ITextInputPanelRunInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTipRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrunning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsTipRunning: IsTipRunning::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanelRunInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipAutoCompleteClient_Impl: Sized {
    fn AdviseProvider(&mut self, hwndfield: super::super::Foundation::HWND, piprovider: &::core::option::Option<ITipAutoCompleteProvider>) -> ::windows::core::Result<()>;
    fn UnadviseProvider(&mut self, hwndfield: super::super::Foundation::HWND, piprovider: &::core::option::Option<ITipAutoCompleteProvider>) -> ::windows::core::Result<()>;
    fn UserSelection(&mut self) -> ::windows::core::Result<()>;
    fn PreferredRects(&mut self, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RequestShowUI(&mut self, hwndlist: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>() -> ITipAutoCompleteClient_Vtbl {
        unsafe extern "system" fn AdviseProvider<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdviseProvider(::core::mem::transmute_copy(&hwndfield), ::core::mem::transmute(&piprovider)).into()
        }
        unsafe extern "system" fn UnadviseProvider<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnadviseProvider(::core::mem::transmute_copy(&hwndfield), ::core::mem::transmute(&piprovider)).into()
        }
        unsafe extern "system" fn UserSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UserSelection().into()
        }
        unsafe extern "system" fn PreferredRects<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreferredRects(::core::mem::transmute_copy(&prcaclist), ::core::mem::transmute_copy(&prcfield), ::core::mem::transmute_copy(&prcmodifiedaclist), ::core::mem::transmute_copy(&pfshownabovetip)).into()
        }
        unsafe extern "system" fn RequestShowUI<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestShowUI(::core::mem::transmute_copy(&hwndlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowshowing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AdviseProvider: AdviseProvider::<Identity, Impl, OFFSET>,
            UnadviseProvider: UnadviseProvider::<Identity, Impl, OFFSET>,
            UserSelection: UserSelection::<Identity, Impl, OFFSET>,
            PreferredRects: PreferredRects::<Identity, Impl, OFFSET>,
            RequestShowUI: RequestShowUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipAutoCompleteClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipAutoCompleteProvider_Impl: Sized {
    fn UpdatePendingText(&mut self, bstrpendingtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Show(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>() -> ITipAutoCompleteProvider_Vtbl {
        unsafe extern "system" fn UpdatePendingText<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpendingtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdatePendingText(::core::mem::transmute_copy(&bstrpendingtext)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&fshow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            UpdatePendingText: UpdatePendingText::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITipAutoCompleteProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkCollectorEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkCollectorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkCollectorEvents_Impl, const OFFSET: isize>() -> _IInkCollectorEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkCollectorEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEditEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEditEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEditEvents_Impl, const OFFSET: isize>() -> _IInkEditEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEditEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkEvents_Impl, const OFFSET: isize>() -> _IInkEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkOverlayEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkOverlayEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkOverlayEvents_Impl, const OFFSET: isize>() -> _IInkOverlayEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkOverlayEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkPictureEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkPictureEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkPictureEvents_Impl, const OFFSET: isize>() -> _IInkPictureEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkPictureEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkRecognitionEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkRecognitionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkRecognitionEvents_Impl, const OFFSET: isize>() -> _IInkRecognitionEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkRecognitionEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkStrokesEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkStrokesEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IInkStrokesEvents_Impl, const OFFSET: isize>() -> _IInkStrokesEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkStrokesEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IMathInputControlEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IMathInputControlEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IMathInputControlEvents_Impl, const OFFSET: isize>() -> _IMathInputControlEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IMathInputControlEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IPenInputPanelEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IPenInputPanelEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IPenInputPanelEvents_Impl, const OFFSET: isize>() -> _IPenInputPanelEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
