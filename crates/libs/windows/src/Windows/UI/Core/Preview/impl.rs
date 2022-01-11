#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreview";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAppWindowPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAppWindowPreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAppWindowPreviewVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAppWindowPreview>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAppWindowPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait ICoreAppWindowPreviewStaticsImpl: Sized {
    fn GetIdFromWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreAppWindowPreviewStatics {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics";
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ICoreAppWindowPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAppWindowPreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAppWindowPreviewStaticsVtbl {
        unsafe extern "system" fn GetIdFromWindow<Impl: ICoreAppWindowPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAppWindowPreviewStatics>, ::windows::core::GetTrustLevel, GetIdFromWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAppWindowPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemNavigationCloseRequestedPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemNavigationCloseRequestedPreviewEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationCloseRequestedPreviewEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemNavigationCloseRequestedPreviewEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationCloseRequestedPreviewEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemNavigationManagerPreviewImpl: Sized {
    fn CloseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationManagerPreview";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemNavigationManagerPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerPreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerPreviewVtbl {
        unsafe extern "system" fn CloseRequested<Impl: ISystemNavigationManagerPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCloseRequested<Impl: ISystemNavigationManagerPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCloseRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemNavigationManagerPreview>, ::windows::core::GetTrustLevel, CloseRequested::<Impl, IMPL_OFFSET>, RemoveCloseRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManagerPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerPreviewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemNavigationManagerPreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationManagerPreviewStatics {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationManagerPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerPreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerPreviewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemNavigationManagerPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemNavigationManagerPreviewStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManagerPreviewStatics as ::windows::core::Interface>::IID
    }
}
