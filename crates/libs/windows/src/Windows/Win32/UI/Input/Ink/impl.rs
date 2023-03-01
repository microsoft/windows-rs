#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"implement\"`*"]
pub trait IInkCommitRequestHandler_Impl: Sized {
    fn OnCommitRequested(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkCommitRequestHandler {}
impl IInkCommitRequestHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCommitRequestHandler_Impl, const OFFSET: isize>() -> IInkCommitRequestHandler_Vtbl {
        unsafe extern "system" fn OnCommitRequested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkCommitRequestHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCommitRequested().into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCommitRequested: OnCommitRequested::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCommitRequestHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInkD2DRenderer_Impl: Sized {
    fn Draw(&self, pd2d1devicecontext: ::core::option::Option<&::windows::core::IUnknown>, pinkstrokeiterable: ::core::option::Option<&::windows::core::IUnknown>, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInkD2DRenderer {}
#[cfg(feature = "Win32_Foundation")]
impl IInkD2DRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkD2DRenderer_Impl, const OFFSET: isize>() -> IInkD2DRenderer_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkD2DRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::windows::core::from_raw_borrowed(&pd2d1devicecontext), ::windows::core::from_raw_borrowed(&pinkstrokeiterable), ::core::mem::transmute_copy(&fhighcontrast)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"implement\"`*"]
pub trait IInkD2DRenderer2_Impl: Sized {
    fn Draw(&self, pd2d1devicecontext: ::core::option::Option<&::windows::core::IUnknown>, pinkstrokeiterable: ::core::option::Option<&::windows::core::IUnknown>, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkD2DRenderer2 {}
impl IInkD2DRenderer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkD2DRenderer2_Impl, const OFFSET: isize>() -> IInkD2DRenderer2_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkD2DRenderer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::windows::core::from_raw_borrowed(&pd2d1devicecontext), ::windows::core::from_raw_borrowed(&pinkstrokeiterable), ::core::mem::transmute_copy(&highcontrastadjustment)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"implement\"`*"]
pub trait IInkDesktopHost_Impl: Sized {
    fn QueueWorkItem(&self, workitem: ::core::option::Option<&IInkHostWorkItem>) -> ::windows::core::Result<()>;
    fn CreateInkPresenter(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateAndInitializeInkPresenter(&self, rootvisual: ::core::option::Option<&::windows::core::IUnknown>, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkDesktopHost {}
impl IInkDesktopHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: isize>() -> IInkDesktopHost_Vtbl {
        unsafe extern "system" fn QueueWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueueWorkItem(::windows::core::from_raw_borrowed(&workitem)).into()
        }
        unsafe extern "system" fn CreateInkPresenter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInkPresenter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAndInitializeInkPresenter(::windows::core::from_raw_borrowed(&rootvisual), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueWorkItem: QueueWorkItem::<Identity, Impl, OFFSET>,
            CreateInkPresenter: CreateInkPresenter::<Identity, Impl, OFFSET>,
            CreateAndInitializeInkPresenter: CreateAndInitializeInkPresenter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDesktopHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"implement\"`*"]
pub trait IInkHostWorkItem_Impl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkHostWorkItem {}
impl IInkHostWorkItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkHostWorkItem_Impl, const OFFSET: isize>() -> IInkHostWorkItem_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkHostWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke().into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkHostWorkItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`, `\"implement\"`*"]
pub trait IInkPresenterDesktop_Impl: Sized {
    fn SetRootVisual(&self, rootvisual: ::core::option::Option<&::windows::core::IUnknown>, device: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetCommitRequestHandler(&self, handler: ::core::option::Option<&IInkCommitRequestHandler>) -> ::windows::core::Result<()>;
    fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()>;
    fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()>;
    fn OnHighContrastChanged(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInkPresenterDesktop {}
impl IInkPresenterDesktop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>() -> IInkPresenterDesktop_Vtbl {
        unsafe extern "system" fn SetRootVisual<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRootVisual(::windows::core::from_raw_borrowed(&rootvisual), ::windows::core::from_raw_borrowed(&device)).into()
        }
        unsafe extern "system" fn SetCommitRequestHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCommitRequestHandler(::windows::core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn OnHighContrastChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHighContrastChanged().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetRootVisual: SetRootVisual::<Identity, Impl, OFFSET>,
            SetCommitRequestHandler: SetCommitRequestHandler::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            OnHighContrastChanged: OnHighContrastChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterDesktop as ::windows::core::ComInterface>::IID
    }
}
