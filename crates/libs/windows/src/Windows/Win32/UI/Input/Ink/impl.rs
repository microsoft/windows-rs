pub trait IInkCommitRequestHandlerImpl: Sized {
    fn OnCommitRequested();
}
impl IInkCommitRequestHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCommitRequestHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkCommitRequestHandlerVtbl {
        unsafe extern "system" fn OnCommitRequested<Impl: IInkCommitRequestHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnCommitRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkCommitRequestHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInkD2DRendererImpl: Sized {
    fn Draw();
}
#[cfg(feature = "Win32_Foundation")]
impl IInkD2DRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkD2DRendererVtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Draw::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer as ::windows::core::Interface>::IID
    }
}
pub trait IInkD2DRenderer2Impl: Sized {
    fn Draw();
}
impl IInkD2DRenderer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkD2DRenderer2Vtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRenderer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Draw::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkD2DRenderer2 as ::windows::core::Interface>::IID
    }
}
pub trait IInkDesktopHostImpl: Sized {
    fn QueueWorkItem();
    fn CreateInkPresenter();
    fn CreateAndInitializeInkPresenter();
}
impl IInkDesktopHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkDesktopHostVtbl {
        unsafe extern "system" fn QueueWorkItem<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInkPresenter<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueueWorkItem::<Impl, IMPL_OFFSET>, CreateInkPresenter::<Impl, IMPL_OFFSET>, CreateAndInitializeInkPresenter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkDesktopHost as ::windows::core::Interface>::IID
    }
}
pub trait IInkHostWorkItemImpl: Sized {
    fn Invoke();
}
impl IInkHostWorkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkHostWorkItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkHostWorkItemVtbl {
        unsafe extern "system" fn Invoke<Impl: IInkHostWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkHostWorkItem as ::windows::core::Interface>::IID
    }
}
pub trait IInkPresenterDesktopImpl: Sized {
    fn SetRootVisual();
    fn SetCommitRequestHandler();
    fn GetSize();
    fn SetSize();
    fn OnHighContrastChanged();
}
impl IInkPresenterDesktopVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktopImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkPresenterDesktopVtbl {
        unsafe extern "system" fn SetRootVisual<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCommitRequestHandler<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnHighContrastChanged<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetRootVisual::<Impl, IMPL_OFFSET>, SetCommitRequestHandler::<Impl, IMPL_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, SetSize::<Impl, IMPL_OFFSET>, OnHighContrastChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkPresenterDesktop as ::windows::core::Interface>::IID
    }
}
