#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreview_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreview";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAppWindowPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAppWindowPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAppWindowPreview_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAppWindowPreview, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAppWindowPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait ICoreAppWindowPreviewStatics_Impl: Sized {
    fn GetIdFromWindow(&mut self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreAppWindowPreviewStatics {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics";
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ICoreAppWindowPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAppWindowPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAppWindowPreviewStatics_Vtbl {
        unsafe extern "system" fn GetIdFromWindow<Impl: ICoreAppWindowPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdFromWindow(&*(&window as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAppWindowPreviewStatics, BASE_OFFSET>(),
            GetIdFromWindow: GetIdFromWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAppWindowPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemNavigationCloseRequestedPreviewEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationCloseRequestedPreviewEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: ISystemNavigationCloseRequestedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemNavigationCloseRequestedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: ISystemNavigationCloseRequestedPreviewEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationCloseRequestedPreviewEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationCloseRequestedPreviewEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemNavigationManagerPreview_Impl: Sized {
    fn CloseRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationManagerPreview";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemNavigationManagerPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerPreview_Vtbl {
        unsafe extern "system" fn CloseRequested<Impl: ISystemNavigationManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseRequested(&*(&handler as *const <super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCloseRequested<Impl: ISystemNavigationManagerPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCloseRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationManagerPreview, BASE_OFFSET>(),
            CloseRequested: CloseRequested::<Impl, IMPL_OFFSET>,
            RemoveCloseRequested: RemoveCloseRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManagerPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerPreviewStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<SystemNavigationManagerPreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationManagerPreviewStatics {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationManagerPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerPreviewStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemNavigationManagerPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationManagerPreviewStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManagerPreviewStatics as ::windows::core::Interface>::IID
    }
}
