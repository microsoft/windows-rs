#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDynamicRenderer_Impl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HWND(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn ClipRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetClipRectangle(&self, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ClipRegion(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetClipRegion(&self, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, pida: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn DataCacheEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetDataCacheEnabled(&self, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseCachedData(&self, strokeid: u32) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Draw(&self, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IDynamicRenderer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDynamicRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>() -> IDynamicRenderer_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(benabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HWND() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn ClipRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClipRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prccliprect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipRectangle(::core::mem::transmute_copy(&prccliprect)).into()
        }
        unsafe extern "system" fn ClipRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClipRegion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phcliprgn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipRegion(::core::mem::transmute_copy(&hcliprgn)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppida: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppida, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pida: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DrawingAttributes(::core::mem::transmute(&pida)).into()
        }
        unsafe extern "system" fn DataCacheEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataCacheEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcachedata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCacheEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcachedata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataCacheEnabled(::core::mem::transmute_copy(&fcachedata)).into()
        }
        unsafe extern "system" fn ReleaseCachedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseCachedData(::core::mem::transmute_copy(&strokeid)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDynamicRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MaxStrokeCount(&self) -> ::windows::core::Result<i32>;
    fn SetMaxStrokeCount(&self, cstrokes: i32) -> ::windows::core::Result<()>;
    fn EnableGestures(&self, cgestures: u32, pgestures: *const i32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGestureRecognizer {}
#[cfg(feature = "Win32_Foundation")]
impl IGestureRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>() -> IGestureRecognizer_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn MaxStrokeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstrokes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxStrokeCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcstrokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStrokeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstrokes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxStrokeCount(::core::mem::transmute_copy(&cstrokes)).into()
        }
        unsafe extern "system" fn EnableGestures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableGestures(::core::mem::transmute_copy(&cgestures), ::core::mem::transmute_copy(&pgestures)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn InsertRecognitionResultsArray(&self, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InsertInkRecognitionResult(&self, piinkrecoresult: &::core::option::Option<IInkRecognitionResult>, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IHandwrittenTextInsertion {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IHandwrittenTextInsertion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>() -> IHandwrittenTextInsertion_Vtbl {
        unsafe extern "system" fn InsertRecognitionResultsArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertRecognitionResultsArray(::core::mem::transmute_copy(&psaalternates), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkrecoresult: *mut ::core::ffi::c_void, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertInkRecognitionResult(::core::mem::transmute(&piinkrecoresult), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ::windows::core::RuntimeName for IInk {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInk_Impl, const OFFSET: isize>() -> IInk_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCollector_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&self) -> ::windows::core::Result<isize>;
    fn SethWnd(&self, newwindow: isize) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, collecting: i16) -> ::windows::core::Result<()>;
    fn DefaultDrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn Cursors(&self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::core::Result<()>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCollector {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>() -> IInkCollector_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SethWnd(::core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentinkrenderer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(autoredraw, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetguids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredPacketDescription(::core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mouseicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mousepointer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cursors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(singletablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(support, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listening, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Inverted(&self) -> ::windows::core::Result<i16>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, attributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Tablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn Buttons(&self) -> ::windows::core::Result<IInkCursorButtons>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCursor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>() -> IInkCursor_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Inverted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DrawingAttributes(::core::mem::transmute(&attributes)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buttons<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttons: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buttons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buttons, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<InkCursorButtonState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCursorButton {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButton_Impl, const OFFSET: isize>() -> IInkCursorButton_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkCursorButton>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCursorButtons {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursorButtons_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButtons_Impl, const OFFSET: isize>() -> IInkCursorButtons_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursorButtons_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(button, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkCursor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCursors {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCursors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursors_Impl, const OFFSET: isize>() -> IInkCursors_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCursors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cursor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cursor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokes>;
    fn Add(&self, name: &super::super::Foundation::BSTR, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn Remove(&self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkCustomStrokes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCustomStrokes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>() -> IInkCustomStrokes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&name), ::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCustomStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn Dirty(&self) -> ::windows::core::Result<i16>;
    fn SetDirty(&self, dirty: i16) -> ::windows::core::Result<()>;
    fn CustomStrokes(&self) -> ::windows::core::Result<IInkCustomStrokes>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn DeleteStrokes(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn DeleteStroke(&self, stroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn ExtractStrokes(&self, strokes: &::core::option::Option<IInkStrokes>, extractflags: InkExtractFlags) -> ::windows::core::Result<IInkDisp>;
    fn ExtractWithRectangle(&self, rectangle: &::core::option::Option<IInkRectangle>, extractflags: InkExtractFlags) -> ::windows::core::Result<IInkDisp>;
    fn Clip(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IInkDisp>;
    fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows::core::Result<IInkStrokes>;
    fn HitTestWithRectangle(&self, selectionrectangle: &::core::option::Option<IInkRectangle>, intersectpercent: f32) -> ::windows::core::Result<IInkStrokes>;
    fn HitTestWithLasso(&self, points: &super::super::System::Com::VARIANT, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn NearestPoint(&self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn CreateStrokes(&self, strokeids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokes>;
    fn AddStrokesAtRectangle(&self, sourcestrokes: &::core::option::Option<IInkStrokes>, targetrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Save(&self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Load(&self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CreateStroke(&self, packetdata: &super::super::System::Com::VARIANT, packetdescription: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkStrokeDisp>;
    fn ClipboardCopyWithRectangle(&self, rectangle: &::core::option::Option<IInkRectangle>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn ClipboardCopy(&self, strokes: &::core::option::Option<IInkStrokes>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn CanPaste(&self, dataobject: &::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<i16>;
    fn ClipboardPaste(&self, x: i32, y: i32, dataobject: &::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<IInkStrokes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDisp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>() -> IInkDisp_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dirty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dirty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirty: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDirty(::core::mem::transmute_copy(&dirty)).into()
        }
        unsafe extern "system" fn CustomStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkinkcustomstrokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CustomStrokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkinkcustomstrokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteStrokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn DeleteStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteStroke(::core::mem::transmute(&stroke)).into()
        }
        unsafe extern "system" fn ExtractStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtractStrokes(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&extractflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extractedink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractWithRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtractWithRectangle(::core::mem::transmute(&rectangle), ::core::mem::transmute_copy(&extractflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extractedink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestCircle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HitTestCircle(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&radius)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionrectangle: *mut ::core::ffi::c_void, intersectpercent: f32, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HitTestWithRectangle(::core::mem::transmute(&selectionrectangle), ::core::mem::transmute_copy(&intersectpercent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithLasso<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HitTestWithLasso(::core::mem::transmute(&points), ::core::mem::transmute_copy(&intersectpercent), ::core::mem::transmute_copy(&lassopoints), ::core::mem::transmute_copy(&strokes)).into()
        }
        unsafe extern "system" fn NearestPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NearestPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&pointonstroke), ::core::mem::transmute_copy(&distancefrompacket), ::core::mem::transmute_copy(&stroke)).into()
        }
        unsafe extern "system" fn CreateStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStrokes(::core::mem::transmute(&strokeids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStrokesAtRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestrokes: *mut ::core::ffi::c_void, targetrectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStrokesAtRectangle(::core::mem::transmute(&sourcestrokes), ::core::mem::transmute(&targetrectangle)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Save(::core::mem::transmute_copy(&persistenceformat), ::core::mem::transmute_copy(&compressionmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CreateStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStroke(::core::mem::transmute(&packetdata), ::core::mem::transmute(&packetdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stroke, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopyWithRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClipboardCopyWithRectangle(::core::mem::transmute(&rectangle), ::core::mem::transmute_copy(&clipboardformats), ::core::mem::transmute_copy(&clipboardmodes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dataobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClipboardCopy(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&clipboardformats), ::core::mem::transmute_copy(&clipboardmodes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dataobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataobject: *mut ::core::ffi::c_void, canpaste: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPaste(::core::mem::transmute(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canpaste, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardPaste<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, dataobject: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClipboardPaste(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute(&dataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn putref_Strokes(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn RecognizerContext(&self) -> ::windows::core::Result<IInkRecognizerContext>;
    fn putref_RecognizerContext(&self, recognizercontext: &::core::option::Option<IInkRecognizerContext>) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<i32>;
    fn SetLineHeight(&self, lineheight: i32) -> ::windows::core::Result<()>;
    fn Divide(&self) -> ::windows::core::Result<IInkDivisionResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDivider {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>() -> IInkDivider_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Strokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn RecognizerContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognizerContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizercontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RecognizerContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizercontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_RecognizerContext(::core::mem::transmute(&recognizercontext)).into()
        }
        unsafe extern "system" fn LineHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineheight, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineHeight(::core::mem::transmute_copy(&lineheight)).into()
        }
        unsafe extern "system" fn Divide<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdivisionresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Divide() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkdivisionresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn ResultByType(&self, divisiontype: InkDivisionType) -> ::windows::core::Result<IInkDivisionUnits>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDivisionResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionResult_Impl, const OFFSET: isize>() -> IInkDivisionResult_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultByType(::core::mem::transmute_copy(&divisiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkdivisionunits, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn DivisionType(&self) -> ::windows::core::Result<InkDivisionType>;
    fn RecognizedString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RotationTransform(&self) -> ::windows::core::Result<IInkTransform>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDivisionUnit {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>() -> IInkDivisionUnit_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DivisionType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, divisiontype: *mut InkDivisionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DivisionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(divisiontype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizedString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognizedString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recostring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotationtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RotationTransform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotationtransform, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkDivisionUnit>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDivisionUnits {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDivisionUnits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>() -> IInkDivisionUnits_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDivisionUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkdivisionunit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkdivisionunit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Color(&self) -> ::windows::core::Result<i32>;
    fn SetColor(&self, newcolor: i32) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<f32>;
    fn SetWidth(&self, newwidth: f32) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<f32>;
    fn SetHeight(&self, newheight: f32) -> ::windows::core::Result<()>;
    fn FitToCurve(&self) -> ::windows::core::Result<i16>;
    fn SetFitToCurve(&self, flag: i16) -> ::windows::core::Result<()>;
    fn IgnorePressure(&self) -> ::windows::core::Result<i16>;
    fn SetIgnorePressure(&self, flag: i16) -> ::windows::core::Result<()>;
    fn AntiAliased(&self) -> ::windows::core::Result<i16>;
    fn SetAntiAliased(&self, flag: i16) -> ::windows::core::Result<()>;
    fn Transparency(&self) -> ::windows::core::Result<i32>;
    fn SetTransparency(&self, newtransparency: i32) -> ::windows::core::Result<()>;
    fn RasterOperation(&self) -> ::windows::core::Result<InkRasterOperation>;
    fn SetRasterOperation(&self, newrasteroperation: InkRasterOperation) -> ::windows::core::Result<()>;
    fn PenTip(&self) -> ::windows::core::Result<InkPenTip>;
    fn SetPenTip(&self, newpentip: InkPenTip) -> ::windows::core::Result<()>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn Clone(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkDrawingAttributes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkDrawingAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>() -> IInkDrawingAttributes_Vtbl {
        unsafe extern "system" fn Color<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcolor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Color() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentcolor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColor(::core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwidth: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWidth(::core::mem::transmute_copy(&newwidth)).into()
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentheight: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentheight, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newheight: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeight(::core::mem::transmute_copy(&newheight)).into()
        }
        unsafe extern "system" fn FitToCurve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FitToCurve() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFitToCurve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFitToCurve(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn IgnorePressure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IgnorePressure() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnorePressure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIgnorePressure(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn AntiAliased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntiAliased() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiAliased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flag: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAntiAliased(::core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn Transparency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currenttransparency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transparency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currenttransparency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransparency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newtransparency: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransparency(::core::mem::transmute_copy(&newtransparency)).into()
        }
        unsafe extern "system" fn RasterOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RasterOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentrasteroperation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newrasteroperation: InkRasterOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRasterOperation(::core::mem::transmute_copy(&newrasteroperation)).into()
        }
        unsafe extern "system" fn PenTip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpentip: *mut InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PenTip() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentpentip, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpentip: InkPenTip) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPenTip(::core::mem::transmute_copy(&newpentip)).into()
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDrawingAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawingattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(drawingattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Status(&self) -> ::windows::core::Result<InkEditStatus>;
    fn UseMouseForInput(&self) -> ::windows::core::Result<i16>;
    fn SetUseMouseForInput(&self, newval: i16) -> ::windows::core::Result<()>;
    fn InkMode(&self) -> ::windows::core::Result<InkMode>;
    fn SetInkMode(&self, newval: InkMode) -> ::windows::core::Result<()>;
    fn InkInsertMode(&self) -> ::windows::core::Result<InkInsertMode>;
    fn SetInkInsertMode(&self, newval: InkInsertMode) -> ::windows::core::Result<()>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, newval: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn RecognitionTimeout(&self) -> ::windows::core::Result<i32>;
    fn SetRecognitionTimeout(&self, newval: i32) -> ::windows::core::Result<()>;
    fn Recognizer(&self) -> ::windows::core::Result<IInkRecognizer>;
    fn putref_Recognizer(&self, newval: &::core::option::Option<IInkRecognizer>) -> ::windows::core::Result<()>;
    fn Factoid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelInks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelInks(&self, selink: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelInksDisplayMode(&self) -> ::windows::core::Result<InkDisplayMode>;
    fn SetSelInksDisplayMode(&self, inkdisplaymode: InkDisplayMode) -> ::windows::core::Result<()>;
    fn Recognize(&self) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn SetBackColor(&self, clr: u32) -> ::windows::core::Result<()>;
    fn BackColor(&self) -> ::windows::core::Result<u32>;
    fn Appearance(&self) -> ::windows::core::Result<AppearanceConstants>;
    fn SetAppearance(&self, pappearance: AppearanceConstants) -> ::windows::core::Result<()>;
    fn BorderStyle(&self) -> ::windows::core::Result<BorderStyleConstants>;
    fn SetBorderStyle(&self, pborderstyle: BorderStyleConstants) -> ::windows::core::Result<()>;
    fn Hwnd(&self) -> ::windows::core::Result<u32>;
    fn Font(&self) -> ::windows::core::Result<super::super::System::Ole::IFontDisp>;
    fn putref_Font(&self, ppfont: &::core::option::Option<super::super::System::Ole::IFontDisp>) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetText(&self, pbstrtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MouseIcon(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn Locked(&self) -> ::windows::core::Result<i16>;
    fn SetLocked(&self, newval: i16) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<i32>;
    fn SetMaxLength(&self, lmaxlength: i32) -> ::windows::core::Result<()>;
    fn MultiLine(&self) -> ::windows::core::Result<i16>;
    fn SetMultiLine(&self, newval: i16) -> ::windows::core::Result<()>;
    fn ScrollBars(&self) -> ::windows::core::Result<ScrollBarsConstants>;
    fn SetScrollBars(&self, newval: ScrollBarsConstants) -> ::windows::core::Result<()>;
    fn DisableNoScroll(&self) -> ::windows::core::Result<i16>;
    fn SetDisableNoScroll(&self, newval: i16) -> ::windows::core::Result<()>;
    fn SelAlignment(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelAlignment(&self, pvarselalignment: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelBold(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelBold(&self, pvarselbold: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelItalic(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelItalic(&self, pvarselitalic: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelUnderline(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelUnderline(&self, pvarselunderline: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelColor(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelColor(&self, pvarselcolor: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelFontName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelFontName(&self, pvarselfontname: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelFontSize(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelFontSize(&self, pvarselfontsize: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SelCharOffset(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSelCharOffset(&self, pvarselcharoffset: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TextRTF(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTextRTF(&self, pbstrtextrtf: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelStart(&self) -> ::windows::core::Result<i32>;
    fn SetSelStart(&self, plselstart: i32) -> ::windows::core::Result<()>;
    fn SelLength(&self) -> ::windows::core::Result<i32>;
    fn SetSelLength(&self, plsellength: i32) -> ::windows::core::Result<()>;
    fn SelText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSelText(&self, pbstrseltext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SelRTF(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSelRTF(&self, pbstrselrtf: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkEdit {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkEdit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>() -> IInkEdit_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut InkEditStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseMouseForInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseMouseForInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseMouseForInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseMouseForInput(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InkMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInkMode(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkInsertMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InkInsertMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkInsertMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: InkInsertMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInkInsertMode(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DrawingAttributes(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn RecognitionTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognitionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecognitionTimeout(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Recognizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Recognizer(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFactoid(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SelInks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselink: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelInks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selink: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelInks(::core::mem::transmute(&selink)).into()
        }
        unsafe extern "system" fn SelInksDisplayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelInksDisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinkdisplaymode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInksDisplayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelInksDisplayMode(::core::mem::transmute_copy(&inkdisplaymode)).into()
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Recognize().into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plisten, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clr: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackColor(::core::mem::transmute_copy(&clr)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: *mut AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappearance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappearance: AppearanceConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppearance(::core::mem::transmute_copy(&pappearance)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pborderstyle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pborderstyle: BorderStyleConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBorderStyle(::core::mem::transmute_copy(&pborderstyle)).into()
        }
        unsafe extern "system" fn Hwnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pohhwnd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pohhwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Font<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Font() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfont, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Font(::core::mem::transmute(&ppfont)).into()
        }
        unsafe extern "system" fn Text<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute(&pbstrtext)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mouseicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mousepointer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Locked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Locked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocked(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxLength(::core::mem::transmute_copy(&lmaxlength)).into()
        }
        unsafe extern "system" fn MultiLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultiLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiLine(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ScrollBars<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScrollBars() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollBars<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ScrollBarsConstants) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScrollBars(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DisableNoScroll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisableNoScroll() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableNoScroll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisableNoScroll(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SelAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselalignment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselalignment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelAlignment(::core::mem::transmute(&pvarselalignment)).into()
        }
        unsafe extern "system" fn SelBold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelBold() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselbold, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelBold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselbold: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelBold(::core::mem::transmute(&pvarselbold)).into()
        }
        unsafe extern "system" fn SelItalic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelItalic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselitalic, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelItalic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselitalic: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelItalic(::core::mem::transmute(&pvarselitalic)).into()
        }
        unsafe extern "system" fn SelUnderline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelUnderline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselunderline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelUnderline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselunderline: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelUnderline(::core::mem::transmute(&pvarselunderline)).into()
        }
        unsafe extern "system" fn SelColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselcolor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcolor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelColor(::core::mem::transmute(&pvarselcolor)).into()
        }
        unsafe extern "system" fn SelFontName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelFontName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselfontname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontname: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelFontName(::core::mem::transmute(&pvarselfontname)).into()
        }
        unsafe extern "system" fn SelFontSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelFontSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselfontsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselfontsize: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelFontSize(::core::mem::transmute(&pvarselfontsize)).into()
        }
        unsafe extern "system" fn SelCharOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelCharOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselcharoffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelCharOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselcharoffset: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelCharOffset(::core::mem::transmute(&pvarselcharoffset)).into()
        }
        unsafe extern "system" fn TextRTF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextRTF() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtextrtf, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRTF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtextrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextRTF(::core::mem::transmute(&pbstrtextrtf)).into()
        }
        unsafe extern "system" fn SelStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plselstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plselstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelStart(::core::mem::transmute_copy(&plselstart)).into()
        }
        unsafe extern "system" fn SelLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsellength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsellength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelLength(::core::mem::transmute_copy(&plsellength)).into()
        }
        unsafe extern "system" fn SelText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrseltext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrseltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelText(::core::mem::transmute(&pbstrseltext)).into()
        }
        unsafe extern "system" fn SelRTF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelRTF() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrselrtf, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelRTF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrselrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelRTF(::core::mem::transmute(&pbstrselrtf)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkEdit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkExtendedProperty>;
    fn Add(&self, guid: &super::super::Foundation::BSTR, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IInkExtendedProperty>;
    fn Remove(&self, identifier: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn DoesPropertyExist(&self, guid: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkExtendedProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>() -> IInkExtendedProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&guid), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkextendedproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn DoesPropertyExist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoesPropertyExist(::core::mem::transmute(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(doespropertyexist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Guid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Data(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetData(&self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkExtendedProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkExtendedProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>() -> IInkExtendedProperty_Vtbl {
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Guid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(guid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkExtendedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&data)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Confidence(&self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Id(&self) -> ::windows::core::Result<InkApplicationGesture>;
    fn GetHotPoint(&self, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkGesture {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkGesture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkGesture_Impl, const OFFSET: isize>() -> IInkGesture_Vtbl {
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut InkApplicationGesture) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkGesture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHotPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            GetHotPoint: GetHotPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkGesture as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IInkLineInfo_Impl: Sized {
    fn SetFormat(&self, pim: *const INKMETRIC) -> ::windows::core::Result<()>;
    fn GetFormat(&self, pim: *const INKMETRIC) -> ::windows::core::Result<()>;
    fn GetInkExtent(&self, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::Result<()>;
    fn GetCandidate(&self, ncandidatenum: u32, pwcrecogword: &::windows::core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetCandidate(&self, ncandidatenum: u32, strrecogword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Recognize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkLineInfo {}
impl IInkLineInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>() -> IInkLineInfo_Vtbl {
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormat(::core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetInkExtent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInkExtent(::core::mem::transmute_copy(&pim), ::core::mem::transmute_copy(&pnwidth)).into()
        }
        unsafe extern "system" fn GetCandidate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, pwcrecogword: ::windows::core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCandidate(::core::mem::transmute_copy(&ncandidatenum), ::core::mem::transmute(&pwcrecogword), ::core::mem::transmute_copy(&pcwcrecogword), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCandidate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncandidatenum: u32, strrecogword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCandidate(::core::mem::transmute_copy(&ncandidatenum), ::core::mem::transmute(&strrecogword)).into()
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkLineInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Recognize().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn hWnd(&self) -> ::windows::core::Result<isize>;
    fn SethWnd(&self, newwindow: isize) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, collecting: i16) -> ::windows::core::Result<()>;
    fn DefaultDrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn EditingMode(&self) -> ::windows::core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows::core::Result<()>;
    fn Selection(&self) -> ::windows::core::Result<IInkStrokes>;
    fn SetSelection(&self, selection: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn EraserMode(&self) -> ::windows::core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows::core::Result<()>;
    fn EraserWidth(&self) -> ::windows::core::Result<i32>;
    fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows::core::Result<()>;
    fn AttachMode(&self) -> ::windows::core::Result<InkOverlayAttachMode>;
    fn SetAttachMode(&self, attachmode: InkOverlayAttachMode) -> ::windows::core::Result<()>;
    fn Cursors(&self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::core::Result<()>;
    fn SupportHighContrastSelectionUI(&self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows::core::Result<()>;
    fn HitTestSelection(&self, x: i32, y: i32) -> ::windows::core::Result<SelectionHitResult>;
    fn Draw(&self, rect: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkOverlay {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkOverlay_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>() -> IInkOverlay_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SethWnd(::core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentinkrenderer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(autoredraw, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetguids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredPacketDescription(::core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mouseicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mousepointer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EditingMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(editingmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEditingMode(::core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Selection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EraserMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(erasermode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEraserMode(::core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EraserWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eraserwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEraserWidth(::core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn AttachMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmode: InkOverlayAttachMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachMode(::core::mem::transmute_copy(&attachmode)).into()
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cursors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(singletablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(support, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportHighContrastSelectionUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(support, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HitTestSelection(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selarea, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute(&rect)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listening, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkOverlay_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn hWnd(&self) -> ::windows::core::Result<isize>;
    fn DefaultDrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Renderer(&self) -> ::windows::core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: &::core::option::Option<IInkRenderer>) -> ::windows::core::Result<()>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn AutoRedraw(&self) -> ::windows::core::Result<i16>;
    fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::core::Result<()>;
    fn CollectingInk(&self) -> ::windows::core::Result<i16>;
    fn CollectionMode(&self) -> ::windows::core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::core::Result<()>;
    fn DynamicRendering(&self) -> ::windows::core::Result<i16>;
    fn SetDynamicRendering(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn DesiredPacketDescription(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MouseIcon(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn MousePointer(&self) -> ::windows::core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::core::Result<()>;
    fn EditingMode(&self) -> ::windows::core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows::core::Result<()>;
    fn Selection(&self) -> ::windows::core::Result<IInkStrokes>;
    fn SetSelection(&self, selection: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn EraserMode(&self) -> ::windows::core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows::core::Result<()>;
    fn EraserWidth(&self) -> ::windows::core::Result<i32>;
    fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows::core::Result<()>;
    fn putref_Picture(&self, ppicture: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn SetPicture(&self, ppicture: &::core::option::Option<super::super::System::Ole::IPictureDisp>) -> ::windows::core::Result<()>;
    fn Picture(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetSizeMode(&self, smnewsizemode: InkPictureSizeMode) -> ::windows::core::Result<()>;
    fn SizeMode(&self) -> ::windows::core::Result<InkPictureSizeMode>;
    fn SetBackColor(&self, newcolor: u32) -> ::windows::core::Result<()>;
    fn BackColor(&self) -> ::windows::core::Result<u32>;
    fn Cursors(&self) -> ::windows::core::Result<IInkCursors>;
    fn MarginX(&self) -> ::windows::core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> ::windows::core::Result<()>;
    fn MarginY(&self) -> ::windows::core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> ::windows::core::Result<()>;
    fn Tablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::core::Result<()>;
    fn SupportHighContrastSelectionUI(&self) -> ::windows::core::Result<i16>;
    fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows::core::Result<()>;
    fn HitTestSelection(&self, x: i32, y: i32) -> ::windows::core::Result<SelectionHitResult>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::core::Result<i16>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::core::Result<i16>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::Result<()>;
    fn InkEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetInkEnabled(&self, collecting: i16) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, vbool: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkPicture {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkPicture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>() -> IInkPicture_Vtbl {
        unsafe extern "system" fn hWnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentwindow: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultDrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DefaultDrawingAttributes(::core::mem::transmute(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Renderer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentinkrenderer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Renderer(::core::mem::transmute(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Ink(::core::mem::transmute(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoRedraw() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(autoredraw, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoredraw: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoRedraw(::core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectingInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkCollectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCollectionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DynamicRendering() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDynamicRendering(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetguids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredPacketDescription(::core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MouseIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mouseicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mouseicon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_MouseIcon(::core::mem::transmute(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: *mut InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MousePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mousepointer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mousepointer: InkMousePointer) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMousePointer(::core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EditingMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(editingmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editingmode: InkOverlayEditingMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEditingMode(::core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Selection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EraserMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(erasermode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erasermode: InkOverlayEraserMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEraserMode(::core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eraserwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EraserWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eraserwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, neweraserwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEraserWidth(::core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn putref_Picture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Picture(::core::mem::transmute(&ppicture)).into()
        }
        unsafe extern "system" fn SetPicture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicture: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPicture(::core::mem::transmute(&ppicture)).into()
        }
        unsafe extern "system" fn Picture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppicture: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Picture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppicture, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSizeMode(::core::mem::transmute_copy(&smnewsizemode)).into()
        }
        unsafe extern "system" fn SizeMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsizemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcolor: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackColor(::core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cursors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cursors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cursors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginX() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginx: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginX(::core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MarginY() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(marginy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, marginy: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarginY(::core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(singletablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportHighContrastInk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(support, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportHighContrastInk(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportHighContrastSelectionUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(support, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, support: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportHighContrastSelectionUI(::core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HitTestSelection(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selarea, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGestureStatus(::core::mem::transmute_copy(&gesture), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGestureStatus(::core::mem::transmute_copy(&gesture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listening, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWindowInputRectangle(::core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowInputRectangle(::core::mem::transmute(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usemouseforinput: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllTabletsMode(::core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tablet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSingleTabletIntegratedMode(::core::mem::transmute(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventInterest(::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterest(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn InkEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InkEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collecting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collecting: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInkEnabled(::core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbool, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbool: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&vbool)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn String(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Confidence(&self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Baseline(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Midline(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Ascender(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Descender(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LineNumber(&self) -> ::windows::core::Result<i32>;
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn LineAlternates(&self) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn ConfidenceAlternates(&self) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn GetStrokesFromStrokeRanges(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<IInkStrokes>;
    fn GetStrokesFromTextRange(&self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn GetTextRangeFromStrokes(&self, strokes: &::core::option::Option<IInkStrokes>, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::Result<()>;
    fn AlternatesWithConstantPropertyValues(&self, propertytype: &super::super::Foundation::BSTR) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn GetPropertyValue(&self, propertytype: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognitionAlternate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>() -> IInkRecognitionAlternate_Vtbl {
        unsafe extern "system" fn String<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.String() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recostring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Baseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(baseline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Midline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midline: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Midline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(midline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ascender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ascender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ascender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descender: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Descender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineAlternates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linealternates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linealternates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfidenceAlternates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidencealternates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConfidenceAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidencealternates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromStrokeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, getstrokesfromstrokeranges: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokesFromStrokeRanges(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(getstrokesfromstrokeranges, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromTextRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStrokesFromTextRange(::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength), ::core::mem::transmute_copy(&getstrokesfromtextrange)).into()
        }
        unsafe extern "system" fn GetTextRangeFromStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextRangeFromStrokes(::core::mem::transmute(&strokes), ::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn AlternatesWithConstantPropertyValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AlternatesWithConstantPropertyValues(::core::mem::transmute(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternateswithconstantpropertyvalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyValue(::core::mem::transmute(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkRecognitionAlternate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognitionAlternates {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>() -> IInkRecognitionAlternates_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecoalternate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkrecoalternate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn TopString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TopAlternate(&self) -> ::windows::core::Result<IInkRecognitionAlternate>;
    fn TopConfidence(&self) -> ::windows::core::Result<InkRecognitionConfidence>;
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn AlternatesFromSelection(&self, selectionstart: i32, selectionlength: i32, maximumalternates: i32) -> ::windows::core::Result<IInkRecognitionAlternates>;
    fn ModifyTopAlternate(&self, alternate: &::core::option::Option<IInkRecognitionAlternate>) -> ::windows::core::Result<()>;
    fn SetResultOnStrokes(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognitionResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognitionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>() -> IInkRecognitionResult_Vtbl {
        unsafe extern "system" fn TopString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TopString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(topstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopAlternate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topalternate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TopAlternate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(topalternate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopConfidence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TopConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(topconfidence, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternatesFromSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AlternatesFromSelection(::core::mem::transmute_copy(&selectionstart), ::core::mem::transmute_copy(&selectionlength), ::core::mem::transmute_copy(&maximumalternates)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternatesfromselection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTopAlternate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alternate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyTopAlternate(::core::mem::transmute(&alternate)).into()
        }
        unsafe extern "system" fn SetResultOnStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognitionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResultOnStrokes().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Vendor(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Capabilities(&self) -> ::windows::core::Result<InkRecognizerCapabilities>;
    fn Languages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SupportedProperties(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PreferredPacketDescription(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreateRecognizerContext(&self) -> ::windows::core::Result<IInkRecognizerContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>() -> IInkRecognizer_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Vendor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vendor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilitiesflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Languages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedproperties: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredpacketdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredPacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preferredpacketdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecognizerContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRecognizerContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UnicodeRanges(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizer2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer2_Impl, const OFFSET: isize>() -> IInkRecognizer2_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnicodeRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicoderanges, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Strokes(&self) -> ::windows::core::Result<IInkStrokes>;
    fn putref_Strokes(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn CharacterAutoCompletionMode(&self) -> ::windows::core::Result<InkRecognizerCharacterAutoCompletionMode>;
    fn SetCharacterAutoCompletionMode(&self, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::Result<()>;
    fn Factoid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&self, factoid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Guide(&self) -> ::windows::core::Result<IInkRecognizerGuide>;
    fn putref_Guide(&self, recognizerguide: &::core::option::Option<IInkRecognizerGuide>) -> ::windows::core::Result<()>;
    fn PrefixText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrefixText(&self, prefix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SuffixText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSuffixText(&self, suffix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RecognitionFlags(&self) -> ::windows::core::Result<InkRecognitionModes>;
    fn SetRecognitionFlags(&self, modes: InkRecognitionModes) -> ::windows::core::Result<()>;
    fn WordList(&self) -> ::windows::core::Result<IInkWordList>;
    fn putref_WordList(&self, wordlist: &::core::option::Option<IInkWordList>) -> ::windows::core::Result<()>;
    fn Recognizer(&self) -> ::windows::core::Result<IInkRecognizer>;
    fn Recognize(&self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::core::option::Option<IInkRecognitionResult>) -> ::windows::core::Result<()>;
    fn StopBackgroundRecognition(&self) -> ::windows::core::Result<()>;
    fn EndInkInput(&self) -> ::windows::core::Result<()>;
    fn BackgroundRecognize(&self, customdata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BackgroundRecognizeWithAlternates(&self, customdata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IInkRecognizerContext>;
    fn IsStringSupported(&self, string: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizerContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>() -> IInkRecognizerContext_Vtbl {
        unsafe extern "system" fn Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strokes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Strokes(::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn CharacterAutoCompletionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CharacterAutoCompletionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterAutoCompletionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharacterAutoCompletionMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFactoid(::core::mem::transmute(&factoid)).into()
        }
        unsafe extern "system" fn Guide<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Guide() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizerguide, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Guide<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizerguide: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Guide(::core::mem::transmute(&recognizerguide)).into()
        }
        unsafe extern "system" fn PrefixText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrefixText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrefixText(::core::mem::transmute(&prefix)).into()
        }
        unsafe extern "system" fn SuffixText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SuffixText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(suffix, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuffixText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSuffixText(::core::mem::transmute(&suffix)).into()
        }
        unsafe extern "system" fn RecognitionFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: *mut InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognitionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modes: InkRecognitionModes) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecognitionFlags(::core::mem::transmute_copy(&modes)).into()
        }
        unsafe extern "system" fn WordList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WordList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wordlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_WordList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_WordList(::core::mem::transmute(&wordlist)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Recognize(::core::mem::transmute_copy(&recognitionstatus), ::core::mem::transmute_copy(&recognitionresult)).into()
        }
        unsafe extern "system" fn StopBackgroundRecognition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopBackgroundRecognition().into()
        }
        unsafe extern "system" fn EndInkInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndInkInput().into()
        }
        unsafe extern "system" fn BackgroundRecognize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackgroundRecognize(::core::mem::transmute(&customdata)).into()
        }
        unsafe extern "system" fn BackgroundRecognizeWithAlternates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackgroundRecognizeWithAlternates(::core::mem::transmute(&customdata)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStringSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsStringSupported(::core::mem::transmute(&string)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn EnabledUnicodeRanges(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetEnabledUnicodeRanges(&self, unicoderanges: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizerContext2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>() -> IInkRecognizerContext2_Vtbl {
        unsafe extern "system" fn EnabledUnicodeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnabledUnicodeRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicoderanges, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledUnicodeRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicoderanges: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabledUnicodeRanges(::core::mem::transmute(&unicoderanges)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn WritingBox(&self) -> ::windows::core::Result<IInkRectangle>;
    fn SetWritingBox(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn DrawnBox(&self) -> ::windows::core::Result<IInkRectangle>;
    fn SetDrawnBox(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Rows(&self) -> ::windows::core::Result<i32>;
    fn SetRows(&self, units: i32) -> ::windows::core::Result<()>;
    fn Columns(&self) -> ::windows::core::Result<i32>;
    fn SetColumns(&self, units: i32) -> ::windows::core::Result<()>;
    fn Midline(&self) -> ::windows::core::Result<i32>;
    fn SetMidline(&self, units: i32) -> ::windows::core::Result<()>;
    fn GuideData(&self) -> ::windows::core::Result<InkRecoGuide>;
    fn SetGuideData(&self, recoguide: &InkRecoGuide) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizerGuide {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizerGuide_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>() -> IInkRecognizerGuide_Vtbl {
        unsafe extern "system" fn WritingBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WritingBox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWritingBox(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn DrawnBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawnBox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawnBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDrawnBox(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Rows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rows() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRows(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Columns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Columns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColumns(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Midline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Midline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMidline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMidline(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn GuideData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoguide: *mut InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GuideData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(precoguide, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuideData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizerGuide_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoguide: InkRecoGuide) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGuideData(::core::mem::transmute(&recoguide)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDefaultRecognizer(&self, lcid: i32) -> ::windows::core::Result<IInkRecognizer>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkRecognizer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRecognizers {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRecognizers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizers_Impl, const OFFSET: isize>() -> IInkRecognizers_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultRecognizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: i32, defaultrecognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultRecognizer(::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultrecognizer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRecognizers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, inkrecognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkrecognizer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Top(&self) -> ::windows::core::Result<i32>;
    fn SetTop(&self, units: i32) -> ::windows::core::Result<()>;
    fn Left(&self) -> ::windows::core::Result<i32>;
    fn SetLeft(&self, units: i32) -> ::windows::core::Result<()>;
    fn Bottom(&self) -> ::windows::core::Result<i32>;
    fn SetBottom(&self, units: i32) -> ::windows::core::Result<()>;
    fn Right(&self) -> ::windows::core::Result<i32>;
    fn SetRight(&self, units: i32) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetData(&self, rect: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetRectangle(&self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::Result<()>;
    fn SetRectangle(&self, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRectangle {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRectangle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>() -> IInkRectangle_Vtbl {
        unsafe extern "system" fn Top<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Top() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTop(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Left<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Left() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLeft(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Bottom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bottom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBottom(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Right<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Right() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(units, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, units: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRight(::core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&rect)).into()
        }
        unsafe extern "system" fn GetRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRectangle(::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRectangle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRectangle(::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&right)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GetViewTransform(&self, viewtransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn SetViewTransform(&self, viewtransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn GetObjectTransform(&self, objecttransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn SetObjectTransform(&self, objecttransform: &::core::option::Option<IInkTransform>) -> ::windows::core::Result<()>;
    fn Draw(&self, hdc: isize, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn DrawStroke(&self, hdc: isize, stroke: &::core::option::Option<IInkStrokeDisp>, drawingattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn PixelToInkSpace(&self, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
    fn InkSpaceToPixel(&self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::Result<()>;
    fn PixelToInkSpaceFromPoints(&self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InkSpaceToPixelFromPoints(&self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Measure(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<IInkRectangle>;
    fn MeasureStroke(&self, stroke: &::core::option::Option<IInkStrokeDisp>, drawingattributes: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<IInkRectangle>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkRenderer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>() -> IInkRenderer_Vtbl {
        unsafe extern "system" fn GetViewTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViewTransform(::core::mem::transmute(&viewtransform)).into()
        }
        unsafe extern "system" fn SetViewTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtransform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewTransform(::core::mem::transmute(&viewtransform)).into()
        }
        unsafe extern "system" fn GetObjectTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectTransform(::core::mem::transmute(&objecttransform)).into()
        }
        unsafe extern "system" fn SetObjectTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objecttransform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectTransform(::core::mem::transmute(&objecttransform)).into()
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&hdc), ::core::mem::transmute(&strokes)).into()
        }
        unsafe extern "system" fn DrawStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, stroke: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawStroke(::core::mem::transmute_copy(&hdc), ::core::mem::transmute(&stroke), ::core::mem::transmute(&drawingattributes)).into()
        }
        unsafe extern "system" fn PixelToInkSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PixelToInkSpace(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InkSpaceToPixel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InkSpaceToPixel(::core::mem::transmute_copy(&hdcdisplay), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn PixelToInkSpaceFromPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PixelToInkSpaceFromPoints(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn InkSpaceToPixelFromPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InkSpaceToPixelFromPoints(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn Measure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Measure(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MeasureStroke(::core::mem::transmute(&stroke), ::core::mem::transmute(&drawingattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn ID(&self) -> ::windows::core::Result<i32>;
    fn BezierPoints(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DrawingAttributes(&self) -> ::windows::core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, drawattrs: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<IInkExtendedProperties>;
    fn PolylineCusps(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn BezierCusps(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SelfIntersections(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PacketCount(&self) -> ::windows::core::Result<i32>;
    fn PacketSize(&self) -> ::windows::core::Result<i32>;
    fn PacketDescription(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Deleted(&self) -> ::windows::core::Result<i16>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn FindIntersections(&self, strokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetRectangleIntersections(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Clip(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows::core::Result<i16>;
    fn NearestPoint(&self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::Result<()>;
    fn Split(&self, splitat: f32) -> ::windows::core::Result<IInkStrokeDisp>;
    fn GetPacketDescriptionPropertyMetrics(&self, propertyname: &super::super::Foundation::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::Result<()>;
    fn GetPoints(&self, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPoints(&self, points: &super::super::System::Com::VARIANT, index: i32, count: i32) -> ::windows::core::Result<i32>;
    fn GetPacketData(&self, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPacketValuesByProperty(&self, propertyname: &super::super::Foundation::BSTR, index: i32, count: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPacketValuesByProperty(&self, bstrpropertyname: &super::super::Foundation::BSTR, packetvalues: &super::super::System::Com::VARIANT, index: i32, count: i32) -> ::windows::core::Result<i32>;
    fn GetFlattenedBezierPoints(&self, fittingerror: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Transform(&self, transform: &::core::option::Option<IInkTransform>, applyonpenwidth: i16) -> ::windows::core::Result<()>;
    fn ScaleToRectangle(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkStrokeDisp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokeDisp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>() -> IInkStrokeDisp_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BezierPoints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(points, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawingAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(drawattrs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_DrawingAttributes(::core::mem::transmute(&drawattrs)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolylineCusps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolylineCusps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cusps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierCusps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusps: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BezierCusps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cusps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelfIntersections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelfIntersections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intersections, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PacketCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetdescription: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PacketDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Deleted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deleted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindIntersections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindIntersections(::core::mem::transmute(&strokes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intersections, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectangleIntersections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRectangleIntersections(::core::mem::transmute(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intersections, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn HitTestCircle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HitTestCircle(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&radius)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intersects, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearestPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NearestPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&distance), ::core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn Split<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, splitat: f32, newstroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Split(::core::mem::transmute_copy(&splitat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newstroke, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketDescriptionPropertyMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPacketDescriptionPropertyMetrics(::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)).into()
        }
        unsafe extern "system" fn GetPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, points: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPoints(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(points, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetPoints(::core::mem::transmute(&points), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofpointsset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, count: i32, packetdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPacketData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketValuesByProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPacketValuesByProperty(::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetvalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPacketValuesByProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetPacketValuesByProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute(&packetvalues), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofpacketsset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlattenedBezierPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlattenedBezierPoints(::core::mem::transmute_copy(&fittingerror)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flattenedbezierpoints, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Transform(::core::mem::transmute(&transform), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleToRectangle(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shear(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokeDisp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn RecognitionResult(&self) -> ::windows::core::Result<IInkRecognitionResult>;
    fn ToString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkStrokeDisp>;
    fn Add(&self, inkstroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn AddStrokes(&self, inkstrokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn Remove(&self, inkstroke: &::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn RemoveStrokes(&self, inkstrokes: &::core::option::Option<IInkStrokes>) -> ::windows::core::Result<()>;
    fn ModifyDrawingAttributes(&self, drawattrs: &::core::option::Option<IInkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::core::Result<IInkRectangle>;
    fn Transform(&self, transform: &::core::option::Option<IInkTransform>, applyonpenwidth: i16) -> ::windows::core::Result<()>;
    fn ScaleToRectangle(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn Clip(&self, rectangle: &::core::option::Option<IInkRectangle>) -> ::windows::core::Result<()>;
    fn RemoveRecognitionResult(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkStrokes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkStrokes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>() -> IInkStrokes_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognitionresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecognitionResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognitionresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tostring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, stroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stroke, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&inkstroke)).into()
        }
        unsafe extern "system" fn AddStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStrokes(::core::mem::transmute(&inkstrokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&inkstroke)).into()
        }
        unsafe extern "system" fn RemoveStrokes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inkstrokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStrokes(::core::mem::transmute(&inkstrokes)).into()
        }
        unsafe extern "system" fn ModifyDrawingAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drawattrs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyDrawingAttributes(::core::mem::transmute(&drawattrs)).into()
        }
        unsafe extern "system" fn GetBoundingBox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingBox(::core::mem::transmute_copy(&boundingboxmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundingbox, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void, applyonpenwidth: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Transform(::core::mem::transmute(&transform), ::core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleToRectangle(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shear(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn Clip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clip(::core::mem::transmute(&rectangle)).into()
        }
        unsafe extern "system" fn RemoveRecognitionResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkStrokes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveRecognitionResult().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PlugAndPlayId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MaximumInputRectangle(&self) -> ::windows::core::Result<IInkRectangle>;
    fn HardwareCapabilities(&self) -> ::windows::core::Result<TabletHardwareCapabilities>;
    fn IsPacketPropertySupported(&self, packetpropertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetPropertyMetrics(&self, propertyname: &super::super::Foundation::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkTablet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>() -> IInkTablet_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugAndPlayId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlugAndPlayId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaximumInputRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HardwareCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPacketPropertySupported(::core::mem::transmute(&packetpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyMetrics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyMetrics(::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&minimum), ::core::mem::transmute_copy(&maximum), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&resolution)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DeviceKind(&self) -> ::windows::core::Result<TabletDeviceKind>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkTablet2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet2_Impl, const OFFSET: isize>() -> IInkTablet2_Vtbl {
        unsafe extern "system" fn DeviceKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: *mut TabletDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(kind, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DeviceKind: DeviceKind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkTablet2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkTablet3_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsMultiTouch(&self) -> ::windows::core::Result<i16>;
    fn MaximumCursors(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkTablet3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablet3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet3_Impl, const OFFSET: isize>() -> IInkTablet3_Vtbl {
        unsafe extern "system" fn IsMultiTouch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pismultitouch: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMultiTouch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pismultitouch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumCursors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablet3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaximumcursors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaximumCursors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaximumcursors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn DefaultTablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn Item(&self, index: i32) -> ::windows::core::Result<IInkTablet>;
    fn IsPacketPropertySupported(&self, packetpropertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkTablets {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTablets_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>() -> IInkTablets_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_newenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultTablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaulttablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, tablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTablets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPacketPropertySupported(::core::mem::transmute(&packetpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Translate(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn Reflect(&self, horizontally: i16, vertically: i16) -> ::windows::core::Result<()>;
    fn Shear(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::Result<()>;
    fn GetTransform(&self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::Result<()>;
    fn SetTransform(&self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::Result<()>;
    fn eM11(&self) -> ::windows::core::Result<f32>;
    fn SeteM11(&self, value: f32) -> ::windows::core::Result<()>;
    fn eM12(&self) -> ::windows::core::Result<f32>;
    fn SeteM12(&self, value: f32) -> ::windows::core::Result<()>;
    fn eM21(&self) -> ::windows::core::Result<f32>;
    fn SeteM21(&self, value: f32) -> ::windows::core::Result<()>;
    fn eM22(&self) -> ::windows::core::Result<f32>;
    fn SeteM22(&self, value: f32) -> ::windows::core::Result<()>;
    fn eDx(&self) -> ::windows::core::Result<f32>;
    fn SeteDx(&self, value: f32) -> ::windows::core::Result<()>;
    fn eDy(&self) -> ::windows::core::Result<f32>;
    fn SeteDy(&self, value: f32) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::XFORM>;
    fn SetData(&self, xform: &super::super::Graphics::Gdi::XFORM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkTransform {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>() -> IInkTransform_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Translate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Translate(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rotate(::core::mem::transmute_copy(&degrees), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Reflect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontally: i16, vertically: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reflect(::core::mem::transmute_copy(&horizontally), ::core::mem::transmute_copy(&vertically)).into()
        }
        unsafe extern "system" fn Shear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shear(::core::mem::transmute_copy(&horizontalcomponent), ::core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleTransform(::core::mem::transmute_copy(&horizontalmultiplier), ::core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransform(::core::mem::transmute_copy(&em11), ::core::mem::transmute_copy(&em12), ::core::mem::transmute_copy(&em21), ::core::mem::transmute_copy(&em22), ::core::mem::transmute_copy(&edx), ::core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransform(::core::mem::transmute_copy(&em11), ::core::mem::transmute_copy(&em12), ::core::mem::transmute_copy(&em21), ::core::mem::transmute_copy(&em22), ::core::mem::transmute_copy(&edx), ::core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn eM11<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eM11() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM11<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteM11(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM12<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eM12() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM12<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteM12(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM21<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eM21() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM21<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteM21(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM22<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eM22() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM22<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteM22(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eDx() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteDx(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.eDy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeteDy(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xform, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xform: super::super::Graphics::Gdi::XFORM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&xform)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AddWord(&self, newword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveWord(&self, removeword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Merge(&self, mergewordlist: &::core::option::Option<IInkWordList>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkWordList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList_Impl, const OFFSET: isize>() -> IInkWordList_Vtbl {
        unsafe extern "system" fn AddWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWord(::core::mem::transmute(&newword)).into()
        }
        unsafe extern "system" fn RemoveWord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, removeword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveWord(::core::mem::transmute(&removeword)).into()
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergewordlist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Merge(::core::mem::transmute(&mergewordlist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AddWords(&self, newwords: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IInkWordList2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkWordList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList2_Impl, const OFFSET: isize>() -> IInkWordList2_Vtbl {
        unsafe extern "system" fn AddWords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkWordList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwords: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWords(::core::mem::transmute(&newwords)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), AddWords: AddWords::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWordList2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IInputPanelWindowHandle_Impl: Sized {
    fn AttachedEditWindow32(&self) -> ::windows::core::Result<i32>;
    fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> ::windows::core::Result<()>;
    fn AttachedEditWindow64(&self) -> ::windows::core::Result<i64>;
    fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInputPanelWindowHandle {}
impl IInputPanelWindowHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>() -> IInputPanelWindowHandle_Vtbl {
        unsafe extern "system" fn AttachedEditWindow32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachedEditWindow32() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachededitwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachedEditWindow32(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn AttachedEditWindow64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachedEditWindow64() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachededitwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachedEditWindow64(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Show(&self) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<i16>;
    fn GetPosition(&self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::Result<()>;
    fn SetPosition(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetCustomPaint(&self, element: i32, paint: i16) -> ::windows::core::Result<()>;
    fn SetCaptionText(&self, captiontext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoadInk(&self, ink: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
    fn SetOwnerWindow(&self, ownerwindow: isize) -> ::windows::core::Result<()>;
    fn EnableExtendedButtons(&self, extended: i16) -> ::windows::core::Result<()>;
    fn GetPreviewHeight(&self) -> ::windows::core::Result<i32>;
    fn SetPreviewHeight(&self, height: i32) -> ::windows::core::Result<()>;
    fn EnableAutoGrow(&self, autogrow: i16) -> ::windows::core::Result<()>;
    fn AddFunctionName(&self, functionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveFunctionName(&self, functionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetHoverIcon(&self) -> ::windows::core::Result<super::super::System::Ole::IPictureDisp>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMathInputControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMathInputControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>() -> IMathInputControl_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show().into()
        }
        unsafe extern "system" fn Hide<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hide().into()
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshown: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbshown, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPosition(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn SetCustomPaint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: i32, paint: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustomPaint(::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&paint)).into()
        }
        unsafe extern "system" fn SetCaptionText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, captiontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaptionText(::core::mem::transmute(&captiontext)).into()
        }
        unsafe extern "system" fn LoadInk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadInk(::core::mem::transmute(&ink)).into()
        }
        unsafe extern "system" fn SetOwnerWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownerwindow: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerWindow(::core::mem::transmute_copy(&ownerwindow)).into()
        }
        unsafe extern "system" fn EnableExtendedButtons<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extended: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableExtendedButtons(::core::mem::transmute_copy(&extended)).into()
        }
        unsafe extern "system" fn GetPreviewHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreviewHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(height, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreviewHeight(::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn EnableAutoGrow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autogrow: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableAutoGrow(::core::mem::transmute_copy(&autogrow)).into()
        }
        unsafe extern "system" fn AddFunctionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFunctionName(::core::mem::transmute(&functionname)).into()
        }
        unsafe extern "system" fn RemoveFunctionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFunctionName(::core::mem::transmute(&functionname)).into()
        }
        unsafe extern "system" fn GetHoverIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMathInputControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hoverimage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHoverIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hoverimage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Busy(&self) -> ::windows::core::Result<i16>;
    fn Factoid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFactoid(&self, factoid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AttachedEditWindow(&self) -> ::windows::core::Result<i32>;
    fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> ::windows::core::Result<()>;
    fn CurrentPanel(&self) -> ::windows::core::Result<PanelType>;
    fn SetCurrentPanel(&self, currentpanel: PanelType) -> ::windows::core::Result<()>;
    fn DefaultPanel(&self) -> ::windows::core::Result<PanelType>;
    fn SetDefaultPanel(&self, defaultpanel: PanelType) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<i16>;
    fn SetVisible(&self, visible: i16) -> ::windows::core::Result<()>;
    fn Top(&self) -> ::windows::core::Result<i32>;
    fn Left(&self) -> ::windows::core::Result<i32>;
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn VerticalOffset(&self) -> ::windows::core::Result<i32>;
    fn SetVerticalOffset(&self, verticaloffset: i32) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<i32>;
    fn SetHorizontalOffset(&self, horizontaloffset: i32) -> ::windows::core::Result<()>;
    fn AutoShow(&self) -> ::windows::core::Result<i16>;
    fn SetAutoShow(&self, autoshow: i16) -> ::windows::core::Result<()>;
    fn MoveTo(&self, left: i32, top: i32) -> ::windows::core::Result<()>;
    fn CommitPendingInput(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn EnableTsf(&self, enable: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPenInputPanel {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPenInputPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>() -> IPenInputPanel_Vtbl {
        unsafe extern "system" fn Busy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busy: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Busy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(busy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Factoid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFactoid(::core::mem::transmute(&factoid)).into()
        }
        unsafe extern "system" fn AttachedEditWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachedEditWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachededitwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPanel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentpanel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentPanel(::core::mem::transmute_copy(&currentpanel)).into()
        }
        unsafe extern "system" fn DefaultPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultpanel: *mut PanelType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultPanel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdefaultpanel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultpanel: PanelType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultPanel(::core::mem::transmute_copy(&defaultpanel)).into()
        }
        unsafe extern "system" fn Visible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Visible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVisible(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn Top<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Top() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Left() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(width, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(height, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(verticaloffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verticaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVerticalOffset(::core::mem::transmute_copy(&verticaloffset)).into()
        }
        unsafe extern "system" fn HorizontalOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(horizontaloffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHorizontalOffset(::core::mem::transmute_copy(&horizontaloffset)).into()
        }
        unsafe extern "system" fn AutoShow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pautoshow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoShow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pautoshow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoShow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoshow: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoShow(::core::mem::transmute_copy(&autoshow)).into()
        }
        unsafe extern "system" fn MoveTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveTo(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn CommitPendingInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitPendingInput().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn EnableTsf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableTsf(::core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HWND(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn WindowInputRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetWindowInputRectangle(&self, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn AddStylusSyncPlugin(&self, iindex: u32, piplugin: &::core::option::Option<IStylusSyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveStylusSyncPlugin(&self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusSyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveAllStylusSyncPlugins(&self) -> ::windows::core::Result<()>;
    fn GetStylusSyncPlugin(&self, iindex: u32) -> ::windows::core::Result<IStylusSyncPlugin>;
    fn GetStylusSyncPluginCount(&self) -> ::windows::core::Result<u32>;
    fn AddStylusAsyncPlugin(&self, iindex: u32, piplugin: &::core::option::Option<IStylusAsyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveStylusAsyncPlugin(&self, iindex: u32, ppiplugin: *mut ::core::option::Option<IStylusAsyncPlugin>) -> ::windows::core::Result<()>;
    fn RemoveAllStylusAsyncPlugins(&self) -> ::windows::core::Result<()>;
    fn GetStylusAsyncPlugin(&self, iindex: u32) -> ::windows::core::Result<IStylusAsyncPlugin>;
    fn GetStylusAsyncPluginCount(&self) -> ::windows::core::Result<u32>;
    fn ChildRealTimeStylusPlugin(&self) -> ::windows::core::Result<IRealTimeStylus>;
    fn putref_ChildRealTimeStylusPlugin(&self, pirts: &::core::option::Option<IRealTimeStylus>) -> ::windows::core::Result<()>;
    fn AddCustomStylusDataToQueue(&self, sq: StylusQueue, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::Result<()>;
    fn ClearStylusQueues(&self) -> ::windows::core::Result<()>;
    fn SetAllTabletsMode(&self, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSingleTabletMode(&self, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn GetTablet(&self) -> ::windows::core::Result<IInkTablet>;
    fn GetTabletContextIdFromTablet(&self, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<u32>;
    fn GetTabletFromTabletContextId(&self, tcid: u32) -> ::windows::core::Result<IInkTablet>;
    fn GetAllTabletContextIds(&self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::Result<()>;
    fn GetStyluses(&self) -> ::windows::core::Result<IInkCursors>;
    fn GetStylusForId(&self, sid: u32) -> ::windows::core::Result<IInkCursor>;
    fn SetDesiredPacketDescription(&self, cproperties: u32, ppropertyguids: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetDesiredPacketDescription(&self, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPacketDescriptionData(&self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IRealTimeStylus {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRealTimeStylus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>() -> IRealTimeStylus_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HWND() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn WindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowInputRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prcwndinputrect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowInputRectangle(::core::mem::transmute_copy(&prcwndinputrect)).into()
        }
        unsafe extern "system" fn AddStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStylusSyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStylusSyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusSyncPlugins<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllStylusSyncPlugins().into()
        }
        unsafe extern "system" fn GetStylusSyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStylusSyncPlugin(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiplugin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusSyncPluginCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStylusSyncPluginCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcplugins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, piplugin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStylusAsyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStylusAsyncPlugin(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusAsyncPlugins<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllStylusAsyncPlugins().into()
        }
        unsafe extern "system" fn GetStylusAsyncPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStylusAsyncPlugin(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiplugin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusAsyncPluginCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplugins: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStylusAsyncPluginCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcplugins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildRealTimeStylusPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppirts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChildRealTimeStylusPlugin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppirts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ChildRealTimeStylusPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirts: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ChildRealTimeStylusPlugin(::core::mem::transmute(&pirts)).into()
        }
        unsafe extern "system" fn AddCustomStylusDataToQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sq: StylusQueue, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddCustomStylusDataToQueue(::core::mem::transmute_copy(&sq), ::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn ClearStylusQueues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearStylusQueues().into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllTabletsMode(::core::mem::transmute_copy(&fusemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSingleTabletMode(::core::mem::transmute(&pitablet)).into()
        }
        unsafe extern "system" fn GetTablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppisingletablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTablet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppisingletablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletContextIdFromTablet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitablet: *mut ::core::ffi::c_void, ptcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTabletContextIdFromTablet(::core::mem::transmute(&pitablet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptcid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletFromTabletContextId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, ppitablet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTabletFromTabletContextId(::core::mem::transmute_copy(&tcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitablet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllTabletContextIds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllTabletContextIds(::core::mem::transmute_copy(&pctcidcount), ::core::mem::transmute_copy(&pptcids)).into()
        }
        unsafe extern "system" fn GetStyluses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkcursors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStyluses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiinkcursors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusForId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u32, ppiinkcursor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStylusForId(::core::mem::transmute_copy(&sid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiinkcursor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropertyguids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredPacketDescription(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropertyguids)).into()
        }
        unsafe extern "system" fn GetDesiredPacketDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesiredPacketDescription(::core::mem::transmute_copy(&pcproperties), ::core::mem::transmute_copy(&pppropertyguids)).into()
        }
        unsafe extern "system" fn GetPacketDescriptionData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPacketDescriptionData(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&pfinktodevicescalex), ::core::mem::transmute_copy(&pfinktodevicescaley), ::core::mem::transmute_copy(&pcpacketproperties), ::core::mem::transmute_copy(&pppacketproperties)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn FlicksEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetFlicksEnabled(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRealTimeStylus2 {}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>() -> IRealTimeStylus2_Vtbl {
        unsafe extern "system" fn FlicksEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FlicksEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlicksEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlicksEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn MultiTouchEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetMultiTouchEnabled(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRealTimeStylus3 {}
#[cfg(feature = "Win32_Foundation")]
impl IRealTimeStylus3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>() -> IRealTimeStylus3_Vtbl {
        unsafe extern "system" fn MultiTouchEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultiTouchEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiTouchEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylus3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiTouchEnabled(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MultiTouchEnabled: MultiTouchEnabled::<Identity, Impl, OFFSET>,
            SetMultiTouchEnabled: SetMultiTouchEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRealTimeStylus3 as ::windows::core::Interface>::IID
    }
}
pub trait IRealTimeStylusSynchronization_Impl: Sized {
    fn AcquireLock(&self, lock: RealTimeStylusLockType) -> ::windows::core::Result<()>;
    fn ReleaseLock(&self, lock: RealTimeStylusLockType) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRealTimeStylusSynchronization {}
impl IRealTimeStylusSynchronization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>() -> IRealTimeStylusSynchronization_Vtbl {
        unsafe extern "system" fn AcquireLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireLock(::core::mem::transmute_copy(&lock)).into()
        }
        unsafe extern "system" fn ReleaseLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRealTimeStylusSynchronization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lock: RealTimeStylusLockType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseLock(::core::mem::transmute_copy(&lock)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ::windows::core::RuntimeName for ISketchInk {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISketchInk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISketchInk_Impl, const OFFSET: isize>() -> ISketchInk_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISketchInk as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStrokeBuilder_Impl: Sized {
    fn CreateStroke(&self, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn BeginStroke(&self, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>) -> ::windows::core::Result<()>;
    fn AppendPackets(&self, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::Result<()>;
    fn EndStroke(&self, tcid: u32, sid: u32, ppiinkstroke: *mut ::core::option::Option<IInkStrokeDisp>, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn Ink(&self) -> ::windows::core::Result<IInkDisp>;
    fn putref_Ink(&self, piinkobj: &::core::option::Option<IInkDisp>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IStrokeBuilder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStrokeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>() -> IStrokeBuilder_Vtbl {
        unsafe extern "system" fn CreateStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStroke(::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&cpacketproperties), ::core::mem::transmute_copy(&ppacketproperties), ::core::mem::transmute_copy(&finktodevicescalex), ::core::mem::transmute_copy(&finktodevicescaley), ::core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn BeginStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginStroke(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&cpacketproperties), ::core::mem::transmute_copy(&ppacketproperties), ::core::mem::transmute_copy(&finktodevicescalex), ::core::mem::transmute_copy(&finktodevicescaley), ::core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn AppendPackets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppendPackets(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets)).into()
        }
        unsafe extern "system" fn EndStroke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut *mut ::core::ffi::c_void, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStroke(::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&ppiinkstroke), ::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        unsafe extern "system" fn Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiinkobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiinkobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStrokeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piinkobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Ink(::core::mem::transmute(&piinkobj)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ::windows::core::RuntimeName for IStylusAsyncPlugin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusAsyncPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusAsyncPlugin_Impl, const OFFSET: isize>() -> IStylusAsyncPlugin_Vtbl {
        Self { base__: IStylusPlugin_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusAsyncPlugin as ::windows::core::Interface>::IID || iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStylusPlugin_Impl: Sized {
    fn RealTimeStylusEnabled(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::Result<()>;
    fn RealTimeStylusDisabled(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::Result<()>;
    fn StylusInRange(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32) -> ::windows::core::Result<()>;
    fn StylusOutOfRange(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32) -> ::windows::core::Result<()>;
    fn StylusDown(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>;
    fn StylusUp(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::Result<()>;
    fn StylusButtonDown(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn StylusButtonUp(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn InAirPackets(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>;
    fn Packets(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::Result<()>;
    fn CustomStylusDataAdded(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::Result<()>;
    fn SystemEvent(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, tcid: u32, sid: u32, event: u16, eventdata: &SYSTEM_EVENT_DATA) -> ::windows::core::Result<()>;
    fn TabletAdded(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, pitablet: &::core::option::Option<IInkTablet>) -> ::windows::core::Result<()>;
    fn TabletRemoved(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, itabletindex: i32) -> ::windows::core::Result<()>;
    fn Error(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>, piplugin: &::core::option::Option<IStylusPlugin>, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::Result<()>;
    fn UpdateMapping(&self, pirtssrc: &::core::option::Option<IRealTimeStylus>) -> ::windows::core::Result<()>;
    fn DataInterest(&self) -> ::windows::core::Result<RealTimeStylusDataInterest>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IStylusPlugin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>() -> IStylusPlugin_Vtbl {
        unsafe extern "system" fn RealTimeStylusEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RealTimeStylusEnabled(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&ctcidcount), ::core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn RealTimeStylusDisabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RealTimeStylusDisabled(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&ctcidcount), ::core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn StylusInRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusInRange(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusOutOfRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusOutOfRange(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusDown(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpropcountperpkt), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusUp(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpropcountperpkt), ::core::mem::transmute_copy(&ppacket), ::core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusButtonDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusButtonDown(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&pguidstylusbutton), ::core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn StylusButtonUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, sid: u32, pguidstylusbutton: *const ::windows::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StylusButtonUp(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&pguidstylusbutton), ::core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn InAirPackets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InAirPackets(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpktcount), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&pcinoutpkts), ::core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn Packets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Packets(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pstylusinfo), ::core::mem::transmute_copy(&cpktcount), ::core::mem::transmute_copy(&cpktbufflength), ::core::mem::transmute_copy(&ppackets), ::core::mem::transmute_copy(&pcinoutpkts), ::core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn CustomStylusDataAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pguidid: *const ::windows::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CustomStylusDataAdded(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn SystemEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SystemEvent(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&tcid), ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&event), ::core::mem::transmute(&eventdata)).into()
        }
        unsafe extern "system" fn TabletAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, pitablet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TabletAdded(::core::mem::transmute(&pirtssrc), ::core::mem::transmute(&pitablet)).into()
        }
        unsafe extern "system" fn TabletRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, itabletindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TabletRemoved(::core::mem::transmute(&pirtssrc), ::core::mem::transmute_copy(&itabletindex)).into()
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void, piplugin: *mut ::core::ffi::c_void, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::core::HRESULT, lptrkey: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error(::core::mem::transmute(&pirtssrc), ::core::mem::transmute(&piplugin), ::core::mem::transmute_copy(&datainterest), ::core::mem::transmute_copy(&hrerrorcode), ::core::mem::transmute_copy(&lptrkey)).into()
        }
        unsafe extern "system" fn UpdateMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pirtssrc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateMapping(::core::mem::transmute(&pirtssrc)).into()
        }
        unsafe extern "system" fn DataInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataInterest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatainterest, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ::windows::core::RuntimeName for IStylusSyncPlugin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStylusSyncPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStylusSyncPlugin_Impl, const OFFSET: isize>() -> IStylusSyncPlugin_Vtbl {
        Self { base__: IStylusPlugin_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStylusSyncPlugin as ::windows::core::Interface>::IID || iid == &<IStylusPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextInputPanel_Impl: Sized {
    fn AttachedEditWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SetAttachedEditWindow(&self, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CurrentInteractionMode(&self) -> ::windows::core::Result<InteractionMode>;
    fn DefaultInPlaceState(&self) -> ::windows::core::Result<InPlaceState>;
    fn SetDefaultInPlaceState(&self, state: InPlaceState) -> ::windows::core::Result<()>;
    fn CurrentInPlaceState(&self) -> ::windows::core::Result<InPlaceState>;
    fn DefaultInputArea(&self) -> ::windows::core::Result<PanelInputArea>;
    fn SetDefaultInputArea(&self, area: PanelInputArea) -> ::windows::core::Result<()>;
    fn CurrentInputArea(&self) -> ::windows::core::Result<PanelInputArea>;
    fn CurrentCorrectionMode(&self) -> ::windows::core::Result<CorrectionMode>;
    fn PreferredInPlaceDirection(&self) -> ::windows::core::Result<InPlaceDirection>;
    fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> ::windows::core::Result<()>;
    fn ExpandPostInsertionCorrection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetExpandPostInsertionCorrection(&self, expand: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceVisibleOnFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetInPlaceVisibleOnFocus(&self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn PopUpCorrectionHeight(&self) -> ::windows::core::Result<i32>;
    fn PopDownCorrectionHeight(&self) -> ::windows::core::Result<i32>;
    fn CommitPendingInput(&self) -> ::windows::core::Result<()>;
    fn SetInPlaceVisibility(&self, visible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::Result<()>;
    fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> ::windows::core::Result<()>;
    fn Advise(&self, eventsink: &::core::option::Option<ITextInputPanelEventSink>, eventmask: u32) -> ::windows::core::Result<()>;
    fn Unadvise(&self, eventsink: &::core::option::Option<ITextInputPanelEventSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITextInputPanel {}
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>() -> ITextInputPanel_Vtbl {
        unsafe extern "system" fn AttachedEditWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachedEditWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachededitwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachededitwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachedEditWindow(::core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentInteractionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentinteractionmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInPlaceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultInPlaceState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultInPlaceState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentInPlaceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentInPlaceState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInputArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultInputArea() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(area, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInputArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultInputArea(::core::mem::transmute_copy(&area)).into()
        }
        unsafe extern "system" fn CurrentInputArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, area: *mut PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentInputArea() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(area, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCorrectionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentCorrectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: *mut InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredInPlaceDirection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(direction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: InPlaceDirection) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredInPlaceDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpandPostInsertionCorrection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expand, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExpandPostInsertionCorrection(::core::mem::transmute_copy(&expand)).into()
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InPlaceVisibleOnFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPlaceVisibleOnFocus(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InPlaceBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundingrectangle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PopUpCorrectionHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(height, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, height: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PopDownCorrectionHeight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(height, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitPendingInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitPendingInput().into()
        }
        unsafe extern "system" fn SetInPlaceVisibility<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPlaceVisibility(::core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn SetInPlacePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPlacePosition(::core::mem::transmute_copy(&xposition), ::core::mem::transmute_copy(&yposition), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xposition: i32, yposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPlaceHoverTargetPosition(::core::mem::transmute_copy(&xposition), ::core::mem::transmute_copy(&yposition)).into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: *mut ::core::ffi::c_void, eventmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Advise(::core::mem::transmute(&eventsink), ::core::mem::transmute_copy(&eventmask)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute(&eventsink)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::Result<()>;
    fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::Result<()>;
    fn InPlaceSizeChanging(&self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InPlaceSizeChanged(&self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::Result<()>;
    fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::Result<()>;
    fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::Result<()>;
    fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::Result<()>;
    fn InPlaceVisibilityChanging(&self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InPlaceVisibilityChanged(&self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TextInserting(&self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn TextInserted(&self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ITextInputPanelEventSink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextInputPanelEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>() -> ITextInputPanelEventSink_Vtbl {
        unsafe extern "system" fn InPlaceStateChanging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceStateChanging(::core::mem::transmute_copy(&oldinplacestate), ::core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceStateChanged(::core::mem::transmute_copy(&oldinplacestate), ::core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceSizeChanging(::core::mem::transmute(&oldboundingrectangle), ::core::mem::transmute(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceSizeChanged(::core::mem::transmute(&oldboundingrectangle), ::core::mem::transmute(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InputAreaChanging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InputAreaChanging(::core::mem::transmute_copy(&oldinputarea), ::core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn InputAreaChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InputAreaChanged(::core::mem::transmute_copy(&oldinputarea), ::core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn CorrectionModeChanging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorrectionModeChanging(::core::mem::transmute_copy(&oldcorrectionmode), ::core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn CorrectionModeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorrectionModeChanged(::core::mem::transmute_copy(&oldcorrectionmode), ::core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceVisibilityChanging(::core::mem::transmute_copy(&oldvisible), ::core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InPlaceVisibilityChanged(::core::mem::transmute_copy(&oldvisible), ::core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn TextInserting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TextInserting(::core::mem::transmute_copy(&ink)).into()
        }
        unsafe extern "system" fn TextInserted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TextInserted(::core::mem::transmute_copy(&ink)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn IsTipRunning(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITextInputPanelRunInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ITextInputPanelRunInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelRunInfo_Impl, const OFFSET: isize>() -> ITextInputPanelRunInfo_Vtbl {
        unsafe extern "system" fn IsTipRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITextInputPanelRunInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTipRunning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrunning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsTipRunning: IsTipRunning::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextInputPanelRunInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITipAutoCompleteClient_Impl: Sized {
    fn AdviseProvider(&self, hwndfield: super::super::Foundation::HWND, piprovider: &::core::option::Option<ITipAutoCompleteProvider>) -> ::windows::core::Result<()>;
    fn UnadviseProvider(&self, hwndfield: super::super::Foundation::HWND, piprovider: &::core::option::Option<ITipAutoCompleteProvider>) -> ::windows::core::Result<()>;
    fn UserSelection(&self) -> ::windows::core::Result<()>;
    fn PreferredRects(&self, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RequestShowUI(&self, hwndlist: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITipAutoCompleteClient {}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>() -> ITipAutoCompleteClient_Vtbl {
        unsafe extern "system" fn AdviseProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseProvider(::core::mem::transmute_copy(&hwndfield), ::core::mem::transmute(&piprovider)).into()
        }
        unsafe extern "system" fn UnadviseProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseProvider(::core::mem::transmute_copy(&hwndfield), ::core::mem::transmute(&piprovider)).into()
        }
        unsafe extern "system" fn UserSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UserSelection().into()
        }
        unsafe extern "system" fn PreferredRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreferredRects(::core::mem::transmute_copy(&prcaclist), ::core::mem::transmute_copy(&prcfield), ::core::mem::transmute_copy(&prcmodifiedaclist), ::core::mem::transmute_copy(&pfshownabovetip)).into()
        }
        unsafe extern "system" fn RequestShowUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestShowUI(::core::mem::transmute_copy(&hwndlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowshowing, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn UpdatePendingText(&self, bstrpendingtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITipAutoCompleteProvider {}
#[cfg(feature = "Win32_Foundation")]
impl ITipAutoCompleteProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>() -> ITipAutoCompleteProvider_Vtbl {
        unsafe extern "system" fn UpdatePendingText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpendingtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatePendingText(::core::mem::transmute(&bstrpendingtext)).into()
        }
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITipAutoCompleteProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&fshow)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ::windows::core::RuntimeName for _IInkCollectorEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkCollectorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkCollectorEvents_Impl, const OFFSET: isize>() -> _IInkCollectorEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkCollectorEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEditEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkEditEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEditEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkEditEvents_Impl, const OFFSET: isize>() -> _IInkEditEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEditEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkEvents_Impl, const OFFSET: isize>() -> _IInkEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkOverlayEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkOverlayEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkOverlayEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkOverlayEvents_Impl, const OFFSET: isize>() -> _IInkOverlayEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkOverlayEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkPictureEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkPictureEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkPictureEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkPictureEvents_Impl, const OFFSET: isize>() -> _IInkPictureEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkPictureEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkRecognitionEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkRecognitionEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkRecognitionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkRecognitionEvents_Impl, const OFFSET: isize>() -> _IInkRecognitionEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkRecognitionEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IInkStrokesEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IInkStrokesEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IInkStrokesEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IInkStrokesEvents_Impl, const OFFSET: isize>() -> _IInkStrokesEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IInkStrokesEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IMathInputControlEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IMathInputControlEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IMathInputControlEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IMathInputControlEvents_Impl, const OFFSET: isize>() -> _IMathInputControlEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IMathInputControlEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IPenInputPanelEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _IPenInputPanelEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IPenInputPanelEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _IPenInputPanelEvents_Impl, const OFFSET: isize>() -> _IPenInputPanelEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
