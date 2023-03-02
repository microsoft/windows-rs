#[doc = "*Required features: `\"UI_WebUI\"`, `\"implement\"`*"]
pub trait IActivatedEventArgsDeferral_Impl: Sized {
    fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgsDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedEventArgsDeferral";
}
impl IActivatedEventArgsDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>() -> IActivatedEventArgsDeferral_Vtbl {
        unsafe extern "system" fn ActivatedOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsDeferral, OFFSET>(),
            ActivatedOperation: ActivatedOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgsDeferral as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"implement\"`*"]
pub trait IWebUIBackgroundTaskInstance_Impl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn SetSucceeded(&self, succeeded: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
}
impl IWebUIBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>() -> IWebUIBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn Succeeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSucceeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSucceeded(succeeded).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebUIBackgroundTaskInstance, OFFSET>(),
            Succeeded: Succeeded::<Identity, Impl, OFFSET>,
            SetSucceeded: SetSucceeded::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIBackgroundTaskInstance as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"implement\"`*"]
pub trait IWebUINavigatedEventArgs_Impl: Sized {
    fn NavigatedOperation(&self) -> ::windows::core::Result<WebUINavigatedOperation>;
}
impl ::windows::core::RuntimeName for IWebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedEventArgs";
}
impl IWebUINavigatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>() -> IWebUINavigatedEventArgs_Vtbl {
        unsafe extern "system" fn NavigatedOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NavigatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebUINavigatedEventArgs, OFFSET>(),
            NavigatedOperation: NavigatedOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUINavigatedEventArgs as ::windows::core::ComInterface>::IID
    }
}
