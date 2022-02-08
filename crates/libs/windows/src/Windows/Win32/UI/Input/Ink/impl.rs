pub trait IInkCommitRequestHandler_Impl: Sized {
    fn OnCommitRequested(&self) -> ::windows::core::Result<()>;
}
impl IInkCommitRequestHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCommitRequestHandler_Impl, const OFFSET: isize>() -> IInkCommitRequestHandler_Vtbl {
        unsafe extern "system" fn OnCommitRequested<Identity: ::windows::core::IUnknownImpl, Impl: IInkCommitRequestHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCommitRequested().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnCommitRequested: OnCommitRequested::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCommitRequestHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInkD2DRenderer_Impl: Sized {
    fn Draw(&self, pd2d1devicecontext: &::core::option::Option<::windows::core::IUnknown>, pinkstrokeiterable: &::core::option::Option<::windows::core::IUnknown>, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInkD2DRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer_Impl, const OFFSET: isize>() -> IInkD2DRenderer_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute(&pd2d1devicecontext), ::core::mem::transmute(&pinkstrokeiterable), ::core::mem::transmute_copy(&fhighcontrast)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer as ::windows::core::Interface>::IID
    }
}
pub trait IInkD2DRenderer2_Impl: Sized {
    fn Draw(&self, pd2d1devicecontext: &::core::option::Option<::windows::core::IUnknown>, pinkstrokeiterable: &::core::option::Option<::windows::core::IUnknown>, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()>;
}
impl IInkD2DRenderer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer2_Impl, const OFFSET: isize>() -> IInkD2DRenderer2_Vtbl {
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute(&pd2d1devicecontext), ::core::mem::transmute(&pinkstrokeiterable), ::core::mem::transmute_copy(&highcontrastadjustment)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Draw: Draw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer2 as ::windows::core::Interface>::IID
    }
}
pub trait IInkDesktopHost_Impl: Sized {
    fn QueueWorkItem(&self, workitem: &::core::option::Option<IInkHostWorkItem>) -> ::windows::core::Result<()>;
    fn CreateInkPresenter(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateAndInitializeInkPresenter(&self, rootvisual: &::core::option::Option<::windows::core::IUnknown>, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IInkDesktopHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHost_Impl, const OFFSET: isize>() -> IInkDesktopHost_Vtbl {
        unsafe extern "system" fn QueueWorkItem<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueueWorkItem(::core::mem::transmute(&workitem)).into()
        }
        unsafe extern "system" fn CreateInkPresenter<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInkPresenter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAndInitializeInkPresenter(::core::mem::transmute(&rootvisual), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueueWorkItem: QueueWorkItem::<Identity, Impl, OFFSET>,
            CreateInkPresenter: CreateInkPresenter::<Identity, Impl, OFFSET>,
            CreateAndInitializeInkPresenter: CreateAndInitializeInkPresenter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDesktopHost as ::windows::core::Interface>::IID
    }
}
pub trait IInkHostWorkItem_Impl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl IInkHostWorkItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkHostWorkItem_Impl, const OFFSET: isize>() -> IInkHostWorkItem_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IInkHostWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkHostWorkItem as ::windows::core::Interface>::IID
    }
}
pub trait IInkPresenterDesktop_Impl: Sized {
    fn SetRootVisual(&self, rootvisual: &::core::option::Option<::windows::core::IUnknown>, device: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetCommitRequestHandler(&self, handler: &::core::option::Option<IInkCommitRequestHandler>) -> ::windows::core::Result<()>;
    fn GetSize(&self, width: *mut f32, height: *mut f32) -> ::windows::core::Result<()>;
    fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()>;
    fn OnHighContrastChanged(&self) -> ::windows::core::Result<()>;
}
impl IInkPresenterDesktop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>() -> IInkPresenterDesktop_Vtbl {
        unsafe extern "system" fn SetRootVisual<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRootVisual(::core::mem::transmute(&rootvisual), ::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn SetCommitRequestHandler<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCommitRequestHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn OnHighContrastChanged<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHighContrastChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetRootVisual: SetRootVisual::<Identity, Impl, OFFSET>,
            SetCommitRequestHandler: SetCommitRequestHandler::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            OnHighContrastChanged: OnHighContrastChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterDesktop as ::windows::core::Interface>::IID
    }
}
