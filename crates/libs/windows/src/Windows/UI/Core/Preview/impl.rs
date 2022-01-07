#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreview";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAppWindowPreviewVtbl {
    pub const fn new<Impl: ICoreAppWindowPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreAppWindowPreviewVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreAppWindowPreview>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreviewStaticsImpl: Sized {
    fn GetIdFromWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAppWindowPreviewStatics {
    const NAME: &'static str = "Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAppWindowPreviewStaticsVtbl {
    pub const fn new<Impl: ICoreAppWindowPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreAppWindowPreviewStaticsVtbl {
        unsafe extern "system" fn GetIdFromWindow<Impl: ICoreAppWindowPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdFromWindow(&*(&window as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreAppWindowPreviewStatics>, base.5, GetIdFromWindow::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationCloseRequestedPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationCloseRequestedPreviewEventArgsVtbl {
    pub const fn new<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemNavigationCloseRequestedPreviewEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: ISystemNavigationCloseRequestedPreviewEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemNavigationCloseRequestedPreviewEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerPreviewImpl: Sized {
    fn CloseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.ISystemNavigationManagerPreview";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationManagerPreviewVtbl {
    pub const fn new<Impl: ISystemNavigationManagerPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemNavigationManagerPreviewVtbl {
        unsafe extern "system" fn CloseRequested<Impl: ISystemNavigationManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseRequested(&*(&handler as *const <super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCloseRequested<Impl: ISystemNavigationManagerPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCloseRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemNavigationManagerPreview>, base.5, CloseRequested::<Impl, OFFSET>, RemoveCloseRequested::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISystemNavigationManagerPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemNavigationManagerPreviewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemNavigationManagerPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemNavigationManagerPreviewStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
