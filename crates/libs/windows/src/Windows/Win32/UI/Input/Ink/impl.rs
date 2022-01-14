pub trait IInkCommitRequestHandler_Impl: Sized {
    fn OnCommitRequested(&mut self) -> ::windows::core::Result<()>;
}
impl IInkCommitRequestHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCommitRequestHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCommitRequestHandler_Vtbl {
        unsafe extern "system" fn OnCommitRequested<Impl: IInkCommitRequestHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCommitRequested().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCommitRequested: OnCommitRequested::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCommitRequestHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInkD2DRenderer_Impl: Sized {
    fn Draw(&mut self, pd2d1devicecontext: ::core::option::Option<::windows::core::IUnknown>, pinkstrokeiterable: ::core::option::Option<::windows::core::IUnknown>, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInkD2DRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkD2DRenderer_Vtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute(&pd2d1devicecontext), ::core::mem::transmute(&pinkstrokeiterable), ::core::mem::transmute_copy(&fhighcontrast)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Draw: Draw::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer as ::windows::core::Interface>::IID
    }
}
pub trait IInkD2DRenderer2_Impl: Sized {
    fn Draw(&mut self, pd2d1devicecontext: ::core::option::Option<::windows::core::IUnknown>, pinkstrokeiterable: ::core::option::Option<::windows::core::IUnknown>, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()>;
}
impl IInkD2DRenderer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkD2DRenderer2_Vtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRenderer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute(&pd2d1devicecontext), ::core::mem::transmute(&pinkstrokeiterable), ::core::mem::transmute_copy(&highcontrastadjustment)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Draw: Draw::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer2 as ::windows::core::Interface>::IID
    }
}
pub trait IInkDesktopHost_Impl: Sized {
    fn QueueWorkItem(&mut self, workitem: ::core::option::Option<IInkHostWorkItem>) -> ::windows::core::Result<()>;
    fn CreateInkPresenter(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateAndInitializeInkPresenter(&mut self, rootvisual: ::core::option::Option<::windows::core::IUnknown>, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IInkDesktopHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDesktopHost_Vtbl {
        unsafe extern "system" fn QueueWorkItem<Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueueWorkItem(::core::mem::transmute(&workitem)).into()
        }
        unsafe extern "system" fn CreateInkPresenter<Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInkPresenter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateAndInitializeInkPresenter(::core::mem::transmute(&rootvisual), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueueWorkItem: QueueWorkItem::<Impl, IMPL_OFFSET>,
            CreateInkPresenter: CreateInkPresenter::<Impl, IMPL_OFFSET>,
            CreateAndInitializeInkPresenter: CreateAndInitializeInkPresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDesktopHost as ::windows::core::Interface>::IID
    }
}
pub trait IInkHostWorkItem_Impl: Sized {
    fn Invoke(&mut self) -> ::windows::core::Result<()>;
}
impl IInkHostWorkItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkHostWorkItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkHostWorkItem_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IInkHostWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkHostWorkItem as ::windows::core::Interface>::IID
    }
}
pub trait IInkPresenterDesktop_Impl: Sized {
    fn SetRootVisual(&mut self, rootvisual: ::core::option::Option<::windows::core::IUnknown>, device: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetCommitRequestHandler(&mut self, handler: ::core::option::Option<IInkCommitRequestHandler>) -> ::windows::core::Result<()>;
    fn GetSize(&mut self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, width: f32, height: f32) -> ::windows::core::Result<()>;
    fn OnHighContrastChanged(&mut self) -> ::windows::core::Result<()>;
}
impl IInkPresenterDesktop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterDesktop_Vtbl {
        unsafe extern "system" fn SetRootVisual<Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootVisual(::core::mem::transmute(&rootvisual), ::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn SetCommitRequestHandler<Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitRequestHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn GetSize<Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn SetSize<Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn OnHighContrastChanged<Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHighContrastChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetRootVisual: SetRootVisual::<Impl, IMPL_OFFSET>,
            SetCommitRequestHandler: SetCommitRequestHandler::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            OnHighContrastChanged: OnHighContrastChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterDesktop as ::windows::core::Interface>::IID
    }
}
